// @generated
pub mod io {
    pub mod blockchain {
        pub mod v1 {
            pub mod dex {
                // @@protoc_insertion_point(attribute:io.blockchain.v1.dex.trade)
                pub mod trade {
                    include!("io.blockchain.v1.dex.trade.rs");
                    // @@protoc_insertion_point(io.blockchain.v1.dex.trade)
                }
            }
        }
    }
    pub mod chainstream {
        pub mod v1 {
            // @@protoc_insertion_point(attribute:io.chainstream.v1.common)
            pub mod common {
                include!("io.chainstream.v1.common.rs");
                // @@protoc_insertion_point(io.chainstream.v1.common)
            }
        }
    }
}
pub mod sf {
    pub mod ethereum {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.ethereum.type.v2)
            pub mod v2 {
                include!("sf.ethereum.type.v2.rs");
                // @@protoc_insertion_point(sf.ethereum.type.v2)
            }
        }
        pub mod substreams {
            // @@protoc_insertion_point(attribute:sf.ethereum.substreams.v1)
            pub mod v1 {
                include!("sf.ethereum.substreams.v1.rs");
                // @@protoc_insertion_point(sf.ethereum.substreams.v1)
            }
        }
    }
}
