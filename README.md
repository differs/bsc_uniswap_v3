# BSC Uniswap V3 Swap Substreams

本项目基于 Substreams，针对 BSC 链上 Uniswap V3 池合约 `0xd857e4a8fe599ed936157076674b2756d9df6fe8` 的 `Swap` 事件进行解析，并输出固定的 Protobuf 类型 `io.blockchain.v1.dex.trade.TradeEvents`。

## 目录结构
- `substreams.yaml`：Substreams 清单，定义模块与过滤器（仅 `map_events`）。
- `proto/`：固定且不能修改的 proto 文件（`common.proto`, `dex_trade_event.proto`）。
- `src/abi/`：由 ABI 生成的合约绑定（已包含 Uniswap V3 Pool 的 `events::Swap`）。
- `src/pb/`：由 buf/prost 生成的 Rust Protobuf 代码。
- `src/lib.rs`：模块实现，解析 `Swap` 并填充 `TradeEvents`。

## 先决条件
- 已安装 `buf`（例如 `buf --version` 输出 `1.57.0`）。
- 已安装 `substreams` CLI。
- Rust toolchain，并能构建 wasm 目标：`wasm32-unknown-unknown`。

## 构建
两种方式均可：

1) 直接构建 wasm 产物
```bash
cargo build --release --target wasm32-unknown-unknown
```

2) 通过 Substreams 打包（会自动生成 Protobuf 代码并构建 wasm）
```bash
substreams build
```
构建完成后会生成：
- `./target/wasm32-unknown-unknown/release/substreams.wasm`
- `bscuniswapv3-v0.1.0.spkg`

## 认证（The Graph / StreamingFast）
```bash
substreams auth               # 打开浏览器完成授权
. ./.substreams.env           # 在当前 Shell 加载 SUBSTREAMS_API_TOKEN
```

## 运行（重点：端点格式）
Substreams 的 `-e` 端点需使用 `host:port` 形式，不能带 `https://` 前缀。

示例（BSC 端点）：
```bash
substreams run -e bsc.streamingfast.io:443 substreams.yaml map_events --start-block 61341628 -t +10 --output jsonl
```

参数说明：
- `-e bsc.streamingfast.io:443`：正确的 gRPC 端点写法（不要写成 `https://bsc.streamingfast.io:443`）。
- `substreams.yaml`：本地清单文件。
- `map_events`：模块名（解析目标池的 Swap 日志）。
- `--start-block` / `--stop-block`：回放区间。
- `--output jsonl`：以 JSON Lines 输出，便于检查。

## 模块说明
- `map_events`
  - 过滤器：`evt_addr:0xd857e4a8fe599ed936157076674b2756d9df6fe8`
  - 从日志解析 Uniswap V3 `Swap` 事件，映射到 `TradeEvents`：
    - `trade.user_a_account_owner_address` = `sender`
    - `trade.user_b_account_owner_address` = `recipient`
    - `trade.user_a_amount` = `amount0`
    - `trade.user_b_amount` = `amount1`
    - `trade.pool_address` = 日志 `address`
    - `trade.was_original_direction` 依据 `amount0` 正负判断
    - 其他字段保持空字符串，后续可按需求扩展

## 常见错误与排查
- 包不存在错误：
  - 报错：`read manifest "map_events": package does not exist on the Substreams registry`
  - 原因：命令参数顺序错误，把 `-e` 指向了本地文件，把模块名当作远程包。
  - 修复：使用 `substreams run -e <endpoint> substreams.yaml map_events ...` 的顺序。

- 端点格式错误：
  - 报错：`too many colons in address`
  - 原因：把 `-e` 写成了 `https://host:port`。
  - 修复：改为 `host:port`（例如 `bsc.streamingfast.io:443`）。

## 备注
- `proto/` 下的 `common.proto` 与 `dex_trade_event.proto` 为固定文件，不应修改。
- 若需发布到 Registry，可参考：
```bash
substreams registry login
substreams registry publish
