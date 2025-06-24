use serde::{Deserialize, Serialize};

use crate::common::ImageType;

structstruck::strike! {
    #[structstruck::each[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]]
    pub struct Asset {
        pub description: String,
        pub denom_units: Vec<pub struct DenomUnit {
            pub denom: String,
            pub exponent: u32,
        }>,
        pub base: String,
        pub display: String,
        pub name: String,
        pub symbol: String,
        pub coingecko_id: Option<String>,
        pub type_asset: Option<String>,
        #[serde(default)]
        pub images: Vec<ImageType>,
        #[serde(rename = "logo_URIs")]
        pub logo_uris: Vec<ImageType>,
        #[serde(default)]
        pub traces: Vec<#[serde(tag = "type")] pub enum Trace {
            #[serde(rename = "op")]
            Op {
                counterparty: pub struct OpCounterparty {
                    pub base_denom: String,
                    pub chain_name: String,
                },
                chain: pub struct OpChain {
                    pub bridge_id: String,
                },
            },
            #[serde(rename = "ibc")]
            Ibc {
                counterparty: pub struct IbcCounterparty {
                    pub chain_name: String,
                    pub base_denom: String,
                    pub channel_id: String,
                },
                chain: pub struct IbcTraceChain {
                    pub channel_id: String,
                    pub path: String,
                },
            },
        }>
    }

    #[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]
    pub struct AssetList {
        #[serde(rename = "$schema")]
        pub schema: String,
        pub chain_name: String,
        pub assets: Vec<Asset>,
    }
}
