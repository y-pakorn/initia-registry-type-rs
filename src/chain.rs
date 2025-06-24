use serde::{Deserialize, Serialize};

structstruck::strike! {
    #[structstruck::each[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]]
    pub struct Chain {
        pub chain_id: String,
        pub chain_name: String,
        pub pretty_name: String,
        pub description: String,
        pub website: String,
        pub fees: pub struct Fees {
            pub fee_tokens: Vec<pub struct FeeToken {
                pub denom: String,
                pub fixed_min_gas_price: Option<f64>,
                pub low_gas_price: Option<f64>,
                pub average_gas_price: Option<f64>,
                pub high_gas_price: Option<f64>,
            }>
        },
        pub apis: pub struct Apis {
            #[serde(default)]
            pub rpc: Vec<pub struct Endpoint {
                pub address: String,
                pub provider: Option<String>,
                #[serde(rename = "authorizedUser")]
                pub authorized_user: Option<String>,
            }>,
            #[serde(default)]
            pub rest: Vec<Endpoint>,
            #[serde(default)]
            pub api: Vec<Endpoint>,
            #[serde(default)]
            pub grpc: Vec<Endpoint>,
            #[serde(rename = "json-rpc", default)]
            pub json_rpc: Vec<Endpoint>,
            #[serde(rename = "json-rpc-websocket", default)]
            pub json_rpc_websocket: Vec<Endpoint>,
        },
        pub explorers: Vec<pub struct Explorer {
            pub kind: String,
            pub url: String,
            pub tx_page: String,
            pub account_page: String,
        }>,
        pub metadata: pub struct Metadata {
            pub op_bridge_id: Option<String>,
            #[serde(default)]
            pub op_denoms: Vec<String>,
            pub executor_uri: Option<String>,
            pub assetlist: Option<String>,
            pub is_l1: Option<bool>,
            #[serde(default)]
            pub ibc_channels: Vec<pub struct IbcChannel {
                pub chain_id: String,
                pub channel_id: String,
                pub port_id: String,
                pub version: String,
            }>,
            pub minitia: Option<pub struct Minitia {
                #[serde(rename = "type")]
                pub ty: pub enum MinitiaType {
                    #[serde(rename = "minievm")]
                    MiniEVM,
                    #[serde(rename = "minimove")]
                    MiniMove,
                    #[serde(rename = "miniwasm")]
                    MiniWasm,
                },
                pub version: String,
            }>,
        },
        #[serde(rename = "logo_URIs")]
        pub logo_uris: pub enum LogoUris {
            #[serde(rename = "png")]
            PNG(String),
            #[serde(rename = "svg")]
            SVG(String),
        },
        pub slip44: u32,
        pub bech32_prefix: String,
        pub network_type: String,
        pub evm_chain_id: Option<u64>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChainList(Vec<Chain>);
}
