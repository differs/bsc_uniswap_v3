mod abi;
mod pb;
use crate::pb::io::chainstream::v1::common::{Chain, DApp, Status, Transaction};
use hex_literal::hex;
use pb::io::blockchain::v1::dex::trade;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use std::collections::HashMap;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

const UNISWAPV3POOL_TRACKED_CONTRACT: [u8; 20] = hex!("d857e4a8fe599ed936157076674b2756d9df6fe8");

fn map_uniswapv3pool_events(blk: &eth::Block, out: &mut trade::TradeEvents) {
    use crate::abi::uniswapv3pool_contract::events as u3e;

    // 预构建：log.block_index -> (tx_hash_hex, tx_index)
    let mut log_to_tx: HashMap<u32, (String, u32)> = HashMap::new();
    for tx in &blk.transaction_traces {
        let tx_hash_hex = format!("0x{}", Hex(&tx.hash));
        let tx_index = tx.index;
        if let Some(receipt) = &tx.receipt {
            for lg in &receipt.logs {
                log_to_tx.insert(lg.block_index, (tx_hash_hex.clone(), tx_index));
            }
        }
    }

    for log in blk.logs() {
        // filter by contract address
        if log.address().as_ref() != UNISWAPV3POOL_TRACKED_CONTRACT {
            continue;
        }

        // match and decode Swap event
        if u3e::Swap::match_log(log.log) {
            if let Ok(ev) = u3e::Swap::decode(log.log) {
                // Build Trade message (fill minimally with available fields)
                // 打印 ev 的完整信息
                substreams::log::info!("ev: {:?}", ev);

                // 根据 block_index 查找 tx hash 与 index
                let (tx_hash_hex, tx_index) = match log_to_tx.get(&log.log.block_index) {
                    Some((h, i)) => (h.clone(), *i),
                    None => (String::new(), 0u32),
                };
                let trade = trade::Trade {
                    token_a_address: String::new(),
                    token_b_address: String::new(),
                    user_a_token_account_address: String::new(),
                    user_a_account_owner_address: format!("0x{}", Hex(&ev.sender)),
                    user_b_token_account_address: String::new(),
                    user_b_account_owner_address: format!("0x{}", Hex(&ev.recipient)),
                    user_a_amount: ev.amount0.to_string(),
                    user_b_amount: ev.amount1.to_string(),
                    user_a_pre_amount: String::new(),
                    user_a_post_amount: String::new(),
                    user_b_pre_amount: String::new(),
                    user_b_post_amount: String::new(),
                    was_original_direction: ev.amount0.clone().to_string().starts_with('-'),
                    pool_address: format!("0x{}", Hex(log.address().as_ref())),
                    vault_a: String::new(),
                    vault_b: String::new(),
                    vault_a_owner_address: String::new(),
                    vault_b_owner_address: String::new(),
                    vault_a_amount: String::new(),
                    vault_b_amount: String::new(),
                    vault_a_pre_amount: String::new(),
                    vault_b_pre_amount: String::new(),
                    vault_a_post_amount: String::new(),
                    vault_b_post_amount: String::new(),
                    pool_config_address: String::new(),
                };

                let te = trade::TradeEvent {
                    instruction: None,
                    block: None,
                    transaction: Some(Transaction {
                        fee: 0,
                        fee_payer: String::new(),
                        index: tx_index as u32,
                        signature: tx_hash_hex,
                        signer: String::new(),
                        status: Status::Unspecified as i32,
                    }),
                    d_app: Some(DApp {
                        program_address: format!("0x{}", Hex(log.address().as_ref())),
                        inner_program_address: String::new(),
                        chain: Chain::Bsc as i32,
                    }),
                    trade: Some(trade),
                    bonding_curve: None,
                };

                out.events.push(te);
            }
        }
    }
}

fn map_uniswapv3pool_calls(_blk: &eth::Block, _calls: &mut trade::TradeEvents) {
    // Intentionally left empty during schema migration (plan B)
}

#[substreams::handlers::map]
fn map_events_calls(
    mut events: trade::TradeEvents,
    mut calls: trade::TradeEvents,
) -> Result<trade::TradeEvents, substreams::errors::Error> {
    // Merge two TradeEvents into one
    let mut merged = trade::TradeEvents { events: Vec::new() };
    merged.events.append(&mut events.events);
    merged.events.append(&mut calls.events);
    Ok(merged)
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<trade::TradeEvents, substreams::errors::Error> {
    let mut events = trade::TradeEvents { events: vec![] };
    map_uniswapv3pool_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<trade::TradeEvents, substreams::errors::Error> {
    let mut calls = trade::TradeEvents { events: vec![] };
    map_uniswapv3pool_calls(&blk, &mut calls);
    Ok(calls)
}
