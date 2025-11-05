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
        pub logo_uris: ImageType,
        #[serde(default)]
        pub traces: Vec<#[serde(tag = "type")] pub enum Trace {
            #[serde(rename = "op")]
            Op {
                counterparty: pub struct OpCounterparty {
                    pub chain_name: String,
                    pub base_denom: String,
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
            #[serde(rename = "wrapped")]
            Wrapped {
                counterparty: pub struct WrappedCounterparty {
                    pub chain_name: String,
                    pub base_denom: String,
                },
                chain: pub struct WrappedChain {
                    pub contract: String,
                },
                provider: String,
            }
        }>
    }

}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct AssetList {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub chain_name: String,
    pub assets: Vec<Asset>,
}

impl Trace {
    pub fn chain_name(&self) -> &str {
        match self {
            Trace::Op { counterparty, .. } => &counterparty.chain_name,
            Trace::Ibc { counterparty, .. } => &counterparty.chain_name,
            Trace::Wrapped { counterparty, .. } => &counterparty.chain_name,
        }
    }

    pub fn base_denom(&self) -> &str {
        match self {
            Trace::Op { counterparty, .. } => &counterparty.base_denom,
            Trace::Ibc { counterparty, .. } => &counterparty.base_denom,
            Trace::Wrapped { counterparty, .. } => &counterparty.base_denom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ImageType;
    use serde_json;

    #[test]
    fn test_deserialize_native_init_token() {
        let json = r#"{
            "description": "The native token of Initia",
            "denom_units": [
                {
                    "denom": "uinit",
                    "exponent": 0
                },
                {
                    "denom": "INIT",
                    "exponent": 6
                }
            ],
            "base": "uinit",
            "display": "INIT",
            "name": "Initia Native Token",
            "symbol": "INIT",
            "coingecko_id": "initia",
            "images": [
                {
                    "png": "https://registry.initia.xyz/images/INIT.png"
                }
            ],
            "logo_URIs": {
                "png": "https://registry.initia.xyz/images/INIT.png"
            }
        }"#;

        let asset: Asset = serde_json::from_str(json).expect("Failed to deserialize INIT token");

        assert_eq!(asset.description, "The native token of Initia");
        assert_eq!(asset.base, "uinit");
        assert_eq!(asset.display, "INIT");
        assert_eq!(asset.name, "Initia Native Token");
        assert_eq!(asset.symbol, "INIT");
        assert_eq!(asset.coingecko_id, Some("initia".to_string()));
        assert_eq!(asset.type_asset, None);

        // Test denom units
        assert_eq!(asset.denom_units.len(), 2);
        assert_eq!(asset.denom_units[0].denom, "uinit");
        assert_eq!(asset.denom_units[0].exponent, 0);
        assert_eq!(asset.denom_units[1].denom, "INIT");
        assert_eq!(asset.denom_units[1].exponent, 6);

        // Test images
        assert_eq!(asset.images.len(), 1);
        match &asset.images[0] {
            ImageType::PNG(url) => assert_eq!(url, "https://registry.initia.xyz/images/INIT.png"),
            _ => panic!("Expected PNG image"),
        }

        // Test logo URIs
        match &asset.logo_uris {
            ImageType::PNG(url) => assert_eq!(url, "https://registry.initia.xyz/images/INIT.png"),
            _ => panic!("Expected PNG logo"),
        }

        // Test traces (should be empty for native token)
        assert!(asset.traces.is_empty());
    }

    #[test]
    fn test_deserialize_ibc_token_usdc() {
        let json = r#"{
            "description": "USDC on Initia",
            "denom_units": [
                {
                    "denom": "ibc/6490A7EAB61059BFC1CDDEB05917DD70BDF3A611654162A1A47DB930D40D8AF4",
                    "exponent": 0
                },
                {
                    "denom": "USDC",
                    "exponent": 6
                }
            ],
            "base": "ibc/6490A7EAB61059BFC1CDDEB05917DD70BDF3A611654162A1A47DB930D40D8AF4",
            "display": "USDC",
            "name": "USD Coin",
            "symbol": "USDC",
            "coingecko_id": "usd-coin",
            "traces": [
                {
                    "type": "ibc",
                    "counterparty": {
                        "chain_name": "noble",
                        "base_denom": "uusdc",
                        "channel_id": "channel-129"
                    },
                    "chain": {
                        "channel_id": "channel-3",
                        "path": "transfer/channel-3/uusdc"
                    }
                }
            ],
            "images": [
                {
                    "png": "https://registry.initia.xyz/images/USDC.png"
                }
            ],
            "logo_URIs": {
                "png": "https://registry.initia.xyz/images/USDC.png"
            }
        }"#;

        let asset: Asset = serde_json::from_str(json).expect("Failed to deserialize USDC token");

        assert_eq!(asset.description, "USDC on Initia");
        assert_eq!(
            asset.base,
            "ibc/6490A7EAB61059BFC1CDDEB05917DD70BDF3A611654162A1A47DB930D40D8AF4"
        );
        assert_eq!(asset.symbol, "USDC");
        assert_eq!(asset.coingecko_id, Some("usd-coin".to_string()));

        // Test traces
        assert_eq!(asset.traces.len(), 1);
        match &asset.traces[0] {
            Trace::Ibc {
                counterparty,
                chain,
            } => {
                assert_eq!(counterparty.chain_name, "noble");
                assert_eq!(counterparty.base_denom, "uusdc");
                assert_eq!(counterparty.channel_id, "channel-129");
                assert_eq!(chain.channel_id, "channel-3");
                assert_eq!(chain.path, "transfer/channel-3/uusdc");
            }
            _ => panic!("Expected IBC trace"),
        }
    }

    #[test]
    fn test_deserialize_op_ibc_token() {
        let json = r#"{
            "description": "OP-IBC bridged INIT of Yominet",
            "denom_units": [
                {
                    "denom": "ibc/07FFEEEE36370551554531FF7189EB2D3353B187CB6CE715DA1291C3A4B4EAC2",
                    "exponent": 0
                },
                {
                    "denom": "INIT.yominet",
                    "exponent": 6
                }
            ],
            "base": "ibc/07FFEEEE36370551554531FF7189EB2D3353B187CB6CE715DA1291C3A4B4EAC2",
            "display": "INIT.yominet",
            "name": "INIT.yominet",
            "symbol": "INIT.yominet",
            "traces": [
                {
                    "type": "op",
                    "counterparty": {
                        "base_denom": "uinit",
                        "chain_name": "initia"
                    },
                    "chain": {
                        "bridge_id": "11"
                    }
                },
                {
                    "type": "ibc",
                    "counterparty": {
                        "chain_name": "yominet",
                        "base_denom": "l2/8f73cfaf153520f511b4fc0bd71d60d64b4e19eff04a350e642718a3c1ab3b06",
                        "channel_id": "channel-0"
                    },
                    "chain": {
                        "channel_id": "channel-25",
                        "path": "transfer/channel-25/l2/8f73cfaf153520f511b4fc0bd71d60d64b4e19eff04a350e642718a3c1ab3b06"
                    }
                }
            ],
            "images": [
                {
                    "png": "https://registry.initia.xyz/images/yominet.ibcopinit.png"
                }
            ],
            "logo_URIs": {
                "png": "https://registry.initia.xyz/images/yominet.ibcopinit.png"
            }
        }"#;

        let asset: Asset = serde_json::from_str(json).expect("Failed to deserialize OP-IBC token");

        assert_eq!(asset.description, "OP-IBC bridged INIT of Yominet");
        assert_eq!(asset.symbol, "INIT.yominet");
        assert_eq!(asset.coingecko_id, None);

        // Test traces (should have both OP and IBC)
        assert_eq!(asset.traces.len(), 2);

        // Test OP trace
        match &asset.traces[0] {
            Trace::Op {
                counterparty,
                chain,
            } => {
                assert_eq!(counterparty.base_denom, "uinit");
                assert_eq!(counterparty.chain_name, "initia");
                assert_eq!(chain.bridge_id, "11");
            }
            _ => panic!("Expected OP trace"),
        }

        // Test IBC trace
        match &asset.traces[1] {
            Trace::Ibc {
                counterparty,
                chain,
            } => {
                assert_eq!(counterparty.chain_name, "yominet");
                assert_eq!(counterparty.channel_id, "channel-0");
                assert_eq!(chain.channel_id, "channel-25");
            }
            _ => panic!("Expected IBC trace"),
        }
    }

    #[test]
    fn test_deserialize_move_token() {
        let json = r#"{
            "description": "ETH token via LayerZero",
            "denom_units": [
                {
                    "denom": "move/edfcddacac79ab86737a1e9e65805066d8be286a37cb94f4884b892b0e39f954",
                    "exponent": 0
                },
                {
                    "denom": "ETH",
                    "exponent": 6
                }
            ],
            "base": "move/edfcddacac79ab86737a1e9e65805066d8be286a37cb94f4884b892b0e39f954",
            "display": "ETH",
            "name": "Ethereum Native Token",
            "symbol": "ETH",
            "coingecko_id": "ethereum",
            "traces": [],
            "images": [
                {
                    "png": "https://registry.initia.xyz/images/ETH.png"
                }
            ],
            "logo_URIs": {
                "png": "https://registry.initia.xyz/images/ETH.png"
            }
        }"#;

        let asset: Asset = serde_json::from_str(json).expect("Failed to deserialize ETH token");

        assert_eq!(asset.description, "ETH token via LayerZero");
        assert_eq!(
            asset.base,
            "move/edfcddacac79ab86737a1e9e65805066d8be286a37cb94f4884b892b0e39f954"
        );
        assert_eq!(asset.symbol, "ETH");
        assert_eq!(asset.coingecko_id, Some("ethereum".to_string()));
        assert!(asset.traces.is_empty());
    }

    #[test]
    fn test_deserialize_assetlist() {
        let json = r#"{
            "$schema": "../../assetlist.schema.json",
            "chain_name": "initia",
            "assets": [
                {
                    "description": "The native token of Initia",
                    "denom_units": [
                        {
                            "denom": "uinit",
                            "exponent": 0
                        },
                        {
                            "denom": "INIT",
                            "exponent": 6
                        }
                    ],
                    "base": "uinit",
                    "display": "INIT",
                    "name": "Initia Native Token",
                    "symbol": "INIT",
                    "coingecko_id": "initia",
                    "images": [
                        {
                            "png": "https://registry.initia.xyz/images/INIT.png"
                        }
                    ],
                    "logo_URIs": {
                        "png": "https://registry.initia.xyz/images/INIT.png"
                    }
                },
                {
                    "description": "USDC on Initia",
                    "denom_units": [
                        {
                            "denom": "ibc/6490A7EAB61059BFC1CDDEB05917DD70BDF3A611654162A1A47DB930D40D8AF4",
                            "exponent": 0
                        },
                        {
                            "denom": "USDC",
                            "exponent": 6
                        }
                    ],
                    "base": "ibc/6490A7EAB61059BFC1CDDEB05917DD70BDF3A611654162A1A47DB930D40D8AF4",
                    "display": "USDC",
                    "name": "USD Coin",
                    "symbol": "USDC",
                    "coingecko_id": "usd-coin",
                    "traces": [
                        {
                            "type": "ibc",
                            "counterparty": {
                                "chain_name": "noble",
                                "base_denom": "uusdc",
                                "channel_id": "channel-129"
                            },
                            "chain": {
                                "channel_id": "channel-3",
                                "path": "transfer/channel-3/uusdc"
                            }
                        }
                    ],
                    "images": [
                        {
                            "png": "https://registry.initia.xyz/images/USDC.png"
                        }
                    ],
                    "logo_URIs": {
                        "png": "https://registry.initia.xyz/images/USDC.png"
                    }
                }
            ]
        }"#;

        let asset_list: AssetList =
            serde_json::from_str(json).expect("Failed to deserialize asset list");

        assert_eq!(asset_list.schema, "../../assetlist.schema.json");
        assert_eq!(asset_list.chain_name, "initia");
        assert_eq!(asset_list.assets.len(), 2);

        // Test first asset (INIT)
        assert_eq!(asset_list.assets[0].symbol, "INIT");
        assert_eq!(asset_list.assets[0].base, "uinit");
        assert!(asset_list.assets[0].traces.is_empty());

        // Test second asset (USDC)
        assert_eq!(asset_list.assets[1].symbol, "USDC");
        assert_eq!(asset_list.assets[1].traces.len(), 1);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let asset = Asset {
            description: "Test token".to_string(),
            denom_units: vec![
                DenomUnit {
                    denom: "utest".to_string(),
                    exponent: 0,
                },
                DenomUnit {
                    denom: "TEST".to_string(),
                    exponent: 6,
                },
            ],
            base: "utest".to_string(),
            display: "TEST".to_string(),
            name: "Test Token".to_string(),
            symbol: "TEST".to_string(),
            coingecko_id: Some("test-token".to_string()),
            type_asset: None,
            images: vec![ImageType::PNG("https://test.com/test.png".to_string())],
            logo_uris: ImageType::PNG("https://test.com/logo.png".to_string()),
            traces: vec![
                Trace::Op {
                    counterparty: OpCounterparty {
                        base_denom: "uoriginal".to_string(),
                        chain_name: "original".to_string(),
                    },
                    chain: OpChain {
                        bridge_id: "1".to_string(),
                    },
                },
                Trace::Ibc {
                    counterparty: IbcCounterparty {
                        chain_name: "source".to_string(),
                        base_denom: "usource".to_string(),
                        channel_id: "channel-0".to_string(),
                    },
                    chain: IbcTraceChain {
                        channel_id: "channel-1".to_string(),
                        path: "transfer/channel-1/usource".to_string(),
                    },
                },
            ],
        };

        let json = serde_json::to_string(&asset).expect("Failed to serialize");
        let deserialized: Asset = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(asset, deserialized);
    }

    #[test]
    fn test_minimal_asset() {
        let json = r#"{
            "description": "Minimal test token",
            "denom_units": [
                {
                    "denom": "uminimal",
                    "exponent": 0
                }
            ],
            "base": "uminimal",
            "display": "MINIMAL",
            "name": "Minimal Token",
            "symbol": "MINIMAL",
            "logo_URIs": {
                "svg": "https://minimal.com/logo.svg"
            }
        }"#;

        let asset: Asset = serde_json::from_str(json).expect("Failed to deserialize minimal asset");

        assert_eq!(asset.description, "Minimal test token");
        assert_eq!(asset.coingecko_id, None);
        assert_eq!(asset.type_asset, None);
        assert!(asset.images.is_empty());
        assert!(asset.traces.is_empty());

        match &asset.logo_uris {
            ImageType::SVG(url) => assert_eq!(url, "https://minimal.com/logo.svg"),
            _ => panic!("Expected SVG logo"),
        }
    }

    #[test]
    fn test_trace_types() {
        // Test OP trace
        let op_json = r#"{
            "type": "op",
            "counterparty": {
                "base_denom": "uinit",
                "chain_name": "initia"
            },
            "chain": {
                "bridge_id": "42"
            }
        }"#;

        let op_trace: Trace =
            serde_json::from_str(op_json).expect("Failed to deserialize OP trace");
        match op_trace {
            Trace::Op {
                counterparty,
                chain,
            } => {
                assert_eq!(counterparty.base_denom, "uinit");
                assert_eq!(counterparty.chain_name, "initia");
                assert_eq!(chain.bridge_id, "42");
            }
            _ => panic!("Expected OP trace"),
        }

        // Test IBC trace
        let ibc_json = r#"{
            "type": "ibc",
            "counterparty": {
                "chain_name": "osmosis",
                "base_denom": "uosmo",
                "channel_id": "channel-0"
            },
            "chain": {
                "channel_id": "channel-1",
                "path": "transfer/channel-1/uosmo"
            }
        }"#;

        let ibc_trace: Trace =
            serde_json::from_str(ibc_json).expect("Failed to deserialize IBC trace");
        match ibc_trace {
            Trace::Ibc {
                counterparty,
                chain,
            } => {
                assert_eq!(counterparty.chain_name, "osmosis");
                assert_eq!(counterparty.base_denom, "uosmo");
                assert_eq!(counterparty.channel_id, "channel-0");
                assert_eq!(chain.channel_id, "channel-1");
                assert_eq!(chain.path, "transfer/channel-1/uosmo");
            }
            _ => panic!("Expected IBC trace"),
        }
    }

    #[test]
    fn test_denom_unit() {
        let json = r#"{
            "denom": "uinit",
            "exponent": 6
        }"#;

        let denom_unit: DenomUnit =
            serde_json::from_str(json).expect("Failed to deserialize denom unit");
        assert_eq!(denom_unit.denom, "uinit");
        assert_eq!(denom_unit.exponent, 6);
    }

    #[test]
    fn test_empty_traces_and_images() {
        let json = r#"{
            "description": "Token without traces",
            "denom_units": [
                {
                    "denom": "utoken",
                    "exponent": 0
                }
            ],
            "base": "utoken",
            "display": "TOKEN",
            "name": "Token",
            "symbol": "TOKEN",
            "logo_URIs": {
                "png": "https://token.com/logo.png"
            }
        }"#;

        let asset: Asset =
            serde_json::from_str(json).expect("Failed to deserialize asset without traces");
        assert!(asset.traces.is_empty());
        assert!(asset.images.is_empty());
        assert_eq!(asset.coingecko_id, None);
        assert_eq!(asset.type_asset, None);
    }

    #[test]
    fn test_assetlist_roundtrip() {
        let asset_list = AssetList {
            schema: "test.schema.json".to_string(),
            chain_name: "testnet".to_string(),
            assets: vec![Asset {
                description: "Test asset".to_string(),
                denom_units: vec![DenomUnit {
                    denom: "utest".to_string(),
                    exponent: 6,
                }],
                base: "utest".to_string(),
                display: "TEST".to_string(),
                name: "Test Asset".to_string(),
                symbol: "TEST".to_string(),
                coingecko_id: None,
                type_asset: None,
                images: vec![],
                logo_uris: ImageType::PNG("https://test.com/logo.png".to_string()),
                traces: vec![],
            }],
        };

        let json = serde_json::to_string(&asset_list).expect("Failed to serialize asset list");
        let deserialized: AssetList =
            serde_json::from_str(&json).expect("Failed to deserialize asset list");

        assert_eq!(asset_list, deserialized);
    }

    #[tokio::test]
    async fn test_fetch_and_parse_echelon_assetlist_json() {
        let url = "https://registry.initia.xyz/chains/initia/assetlist.json";
        let response = reqwest::get(url)
            .await
            .expect("Failed to fetch assetlist.json");

        assert!(
            response.status().is_success(),
            "HTTP request failed with status: {}",
            response.status()
        );

        let json_text = response.text().await.expect("Failed to read response body");

        let asset_list: AssetList =
            serde_json::from_str(&json_text).expect("Failed to parse assetlist.json");

        // Verify that we got at least one asset
        assert!(
            !asset_list.assets.is_empty(),
            "Expected at least one asset in the response"
        );

        // Verify the chain name
        assert_eq!(asset_list.chain_name, "echelon");

        // Verify that the INIT token exists
        let init_asset = asset_list
            .assets
            .iter()
            .find(|asset| asset.symbol == "INIT")
            .expect("Expected to find INIT asset");

        assert_eq!(init_asset.symbol, "INIT");
        assert_eq!(init_asset.display, "INIT");
        assert_eq!(init_asset.name, "Initia Native Token");

        // Verify INIT has OP trace
        assert!(
            !init_asset.traces.is_empty(),
            "Expected INIT to have traces"
        );
        match &init_asset.traces[0] {
            Trace::Op { counterparty, .. } => {
                assert_eq!(counterparty.chain_name, "initia");
                assert_eq!(counterparty.base_denom, "uinit");
            }
            _ => panic!("Expected OP trace for INIT"),
        }
    }
}
