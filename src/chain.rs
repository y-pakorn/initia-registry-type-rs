use serde::{Deserialize, Serialize};

use crate::common::ImageType;

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
        pub logo_uris: ImageType,
        pub slip44: u32,
        pub bech32_prefix: String,
        pub network_type: String,
        pub evm_chain_id: Option<u64>,
    }

}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct ChainList(pub Vec<Chain>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ImageType;
    use serde_json;

    #[test]
    fn test_deserialize_initia_mainnet() {
        let json = r#"{
            "chain_id": "interwoven-1",
            "chain_name": "initia",
            "pretty_name": "Initia",
            "description": "Initia Mainnet",
            "website": "https://initia.xyz",
            "fees": {
                "fee_tokens": [
                    {
                        "denom": "uinit",
                        "fixed_min_gas_price": 0.015,
                        "low_gas_price": 0.015,
                        "average_gas_price": 0.015,
                        "high_gas_price": 0.04
                    },
                    {
                        "denom": "ibc/6490A7EAB61059BFC1CDDEB05917DD70BDF3A611654162A1A47DB930D40D8AF4"
                    }
                ]
            },
            "apis": {
                "rpc": [
                    {
                        "address": "https://rpc.initia.xyz",
                        "provider": "Initia Labs"
                    },
                    {
                        "address": "https://rpc-skip.initia.xyz",
                        "provider": "Initia Labs",
                        "authorizedUser": "skip"
                    }
                ],
                "rest": [
                    {
                        "address": "https://rest.initia.xyz",
                        "provider": "Initia Labs"
                    }
                ],
                "api": [
                    {
                        "address": "https://api.initia.xyz",
                        "provider": "Initia Labs"
                    }
                ],
                "grpc": [
                    {
                        "address": "grpc.initia.xyz:443",
                        "provider": "Initia Labs"
                    }
                ]
            },
            "explorers": [
                {
                    "kind": "initia scan",
                    "url": "https://scan.initia.xyz/interwoven-1",
                    "tx_page": "https://scan.initia.xyz/interwoven-1/txs/${txHash}",
                    "account_page": "https://scan.initia.xyz/interwoven-1/accounts/${accountAddress}"
                }
            ],
            "metadata": {
                "is_l1": true,
                "assetlist": "https://registry.initia.xyz/chains/initia/assetlist.json",
                "ibc_channels": [
                    {
                        "chain_id": "osmosis-1",
                        "port_id": "transfer",
                        "channel_id": "channel-71",
                        "version": "ics20-1"
                    }
                ]
            },
            "logo_URIs": {
                "png": "https://registry.initia.xyz/images/INIT.png"
            },
            "slip44": 60,
            "bech32_prefix": "init",
            "network_type": "mainnet"
        }"#;

        let chain: Chain = serde_json::from_str(json).expect("Failed to deserialize chain");

        assert_eq!(chain.chain_id, "interwoven-1");
        assert_eq!(chain.chain_name, "initia");
        assert_eq!(chain.pretty_name, "Initia");
        assert_eq!(chain.description, "Initia Mainnet");
        assert_eq!(chain.website, "https://initia.xyz");
        assert_eq!(chain.slip44, 60);
        assert_eq!(chain.bech32_prefix, "init");
        assert_eq!(chain.network_type, "mainnet");
        assert_eq!(chain.evm_chain_id, None);

        // Test fees
        assert_eq!(chain.fees.fee_tokens.len(), 2);
        assert_eq!(chain.fees.fee_tokens[0].denom, "uinit");
        assert_eq!(chain.fees.fee_tokens[0].fixed_min_gas_price, Some(0.015));
        assert_eq!(chain.fees.fee_tokens[1].fixed_min_gas_price, None);

        // Test APIs
        assert_eq!(chain.apis.rpc.len(), 2);
        assert_eq!(chain.apis.rpc[0].address, "https://rpc.initia.xyz");
        assert_eq!(chain.apis.rpc[0].provider, Some("Initia Labs".to_string()));
        assert_eq!(chain.apis.rpc[1].authorized_user, Some("skip".to_string()));

        // Test metadata
        assert_eq!(chain.metadata.is_l1, Some(true));
        assert_eq!(
            chain.metadata.assetlist,
            Some("https://registry.initia.xyz/chains/initia/assetlist.json".to_string())
        );
        assert_eq!(chain.metadata.ibc_channels.len(), 1);
        assert_eq!(chain.metadata.ibc_channels[0].chain_id, "osmosis-1");

        // Test logo URIs
        match &chain.logo_uris {
            ImageType::PNG(url) => assert_eq!(url, "https://registry.initia.xyz/images/INIT.png"),
            _ => panic!("Expected PNG image type"),
        }
    }

    #[test]
    fn test_deserialize_minievm_chain() {
        let json = r#"{
            "chain_id": "yominet-1",
            "chain_name": "yominet",
            "pretty_name": "Yominet",
            "description": "The first economically independent virtual world living onchain. Home to the Kamigotchi.",
            "website": "https://kamigotchi.io",
            "fees": {
                "fee_tokens": [
                    {
                        "denom": "evm/E1Ff7038eAAAF027031688E1535a055B2Bac2546",
                        "fixed_min_gas_price": 5000000
                    }
                ]
            },
            "apis": {
                "rpc": [
                    {
                        "address": "https://rpc-yominet-1.anvil.asia-southeast.initia.xyz"
                    }
                ],
                "rest": [
                    {
                        "address": "https://rest-yominet-1.anvil.asia-southeast.initia.xyz"
                    }
                ],
                "grpc": [
                    {
                        "address": "grpc-yominet-1.anvil.asia-southeast.initia.xyz:443"
                    }
                ],
                "json-rpc": [
                    {
                        "address": "https://jsonrpc-yominet-1.anvil.asia-southeast.initia.xyz"
                    }
                ],
                "json-rpc-websocket": [
                    {
                        "address": "wss://jsonrpc-ws-yominet-1.anvil.asia-southeast.initia.xyz"
                    }
                ]
            },
            "explorers": [
                {
                    "kind": "initia scan",
                    "url": "https://scan.initia.xyz/yominet-1",
                    "tx_page": "https://scan.initia.xyz/yominet-1/txs/${txHash}",
                    "account_page": "https://scan.initia.xyz/yominet-1/accounts/${accountAddress}"
                }
            ],
            "metadata": {
                "op_bridge_id": "11",
                "op_denoms": ["uinit"],
                "executor_uri": "https://opinit-api-yominet-1.anvil.asia-southeast.initia.xyz",
                "ibc_channels": [
                    {
                        "chain_id": "interwoven-1",
                        "port_id": "nft-transfer",
                        "channel_id": "channel-1",
                        "version": "ics721-1"
                    }
                ],
                "assetlist": "https://registry.initia.xyz/chains/yominet/assetlist.json",
                "minitia": {
                    "type": "minievm",
                    "version": "v1.0.0-rc.0-kami.1"
                }
            },
            "logo_URIs": {
                "png": "https://registry.initia.xyz/images/yominet.png"
            },
            "slip44": 60,
            "bech32_prefix": "init",
            "network_type": "mainnet",
            "evm_chain_id": 428962654539583
        }"#;

        let chain: Chain = serde_json::from_str(json).expect("Failed to deserialize minievm chain");

        assert_eq!(chain.chain_id, "yominet-1");
        assert_eq!(chain.evm_chain_id, Some(428962654539583));

        // Test minitia metadata
        let minitia = chain.metadata.minitia.expect("Expected minitia metadata");
        assert_eq!(minitia.ty, MinitiaType::MiniEVM);
        assert_eq!(minitia.version, "v1.0.0-rc.0-kami.1");

        // Test op bridge metadata
        assert_eq!(chain.metadata.op_bridge_id, Some("11".to_string()));
        assert_eq!(chain.metadata.op_denoms, vec!["uinit"]);
        assert_eq!(
            chain.metadata.executor_uri,
            Some("https://opinit-api-yominet-1.anvil.asia-southeast.initia.xyz".to_string())
        );

        // Test json-rpc endpoints
        assert_eq!(chain.apis.json_rpc.len(), 1);
        assert_eq!(chain.apis.json_rpc_websocket.len(), 1);
        assert_eq!(
            chain.apis.json_rpc[0].address,
            "https://jsonrpc-yominet-1.anvil.asia-southeast.initia.xyz"
        );
    }

    #[test]
    fn test_deserialize_minimove_chain() {
        let json = r#"{
            "chain_id": "rena-nuwa-1",
            "chain_name": "rena",
            "pretty_name": "Rena",
            "description": "Building the first Trusted Execution Environment (TEE) abstraction middleware.",
            "website": "https://renalabs.xyz",
            "fees": {
                "fee_tokens": [
                    {
                        "denom": "l2/9d3d65bf3329e45ad659f9cbee7d6dc7b6246b001e32131a9b465215eab90562",
                        "fixed_min_gas_price": 0.015
                    }
                ]
            },
            "apis": {
                "rpc": [
                    {
                        "address": "https://rpc-rena-nuwa-1.anvil.asia-southeast.initia.xyz"
                    }
                ],
                "rest": [
                    {
                        "address": "https://rest-rena-nuwa-1.anvil.asia-southeast.initia.xyz"
                    }
                ],
                "grpc": [
                    {
                        "address": "grpc-rena-nuwa-1.anvil.asia-southeast.initia.xyz:443"
                    }
                ]
            },
            "explorers": [
                {
                    "kind": "initia scan",
                    "url": "https://scan.initia.xyz/rena-nuwa-1",
                    "tx_page": "https://scan.initia.xyz/rena-nuwa-1/txs/${txHash}",
                    "account_page": "https://scan.initia.xyz/rena-nuwa-1/accounts/${accountAddress}"
                }
            ],
            "metadata": {
                "op_bridge_id": "30",
                "op_denoms": ["uinit"],
                "executor_uri": "https://opinit-api-rena-nuwa-1.anvil.asia-southeast.initia.xyz",
                "ibc_channels": [
                    {
                        "chain_id": "interwoven-1",
                        "port_id": "nft-transfer",
                        "channel_id": "channel-1",
                        "version": "ics721-1"
                    }
                ],
                "assetlist": "https://registry.initia.xyz/chains/rena/assetlist.json",
                "minitia": {
                    "type": "minimove",
                    "version": "v1.0.2"
                }
            },
            "logo_URIs": {
                "png": "https://registry.initia.xyz/images/rena.png"
            },
            "slip44": 60,
            "bech32_prefix": "init",
            "network_type": "mainnet"
        }"#;

        let chain: Chain =
            serde_json::from_str(json).expect("Failed to deserialize minimove chain");

        let minitia = chain.metadata.minitia.expect("Expected minitia metadata");
        assert_eq!(minitia.ty, MinitiaType::MiniMove);
        assert_eq!(minitia.version, "v1.0.2");
    }

    #[test]
    fn test_serialize_roundtrip() {
        let chain = Chain {
            chain_id: "test-1".to_string(),
            chain_name: "test".to_string(),
            pretty_name: "Test Chain".to_string(),
            description: "A test chain".to_string(),
            website: "https://test.com".to_string(),
            fees: Fees {
                fee_tokens: vec![FeeToken {
                    denom: "utest".to_string(),
                    fixed_min_gas_price: Some(0.01),
                    low_gas_price: Some(0.01),
                    average_gas_price: Some(0.02),
                    high_gas_price: Some(0.03),
                }],
            },
            apis: Apis {
                rpc: vec![Endpoint {
                    address: "https://rpc.test.com".to_string(),
                    provider: Some("Test Provider".to_string()),
                    authorized_user: None,
                }],
                rest: vec![],
                api: vec![],
                grpc: vec![],
                json_rpc: vec![],
                json_rpc_websocket: vec![],
            },
            explorers: vec![Explorer {
                kind: "test scan".to_string(),
                url: "https://scan.test.com".to_string(),
                tx_page: "https://scan.test.com/txs/${txHash}".to_string(),
                account_page: "https://scan.test.com/accounts/${accountAddress}".to_string(),
            }],
            metadata: Metadata {
                op_bridge_id: None,
                op_denoms: vec![],
                executor_uri: None,
                assetlist: None,
                is_l1: Some(false),
                ibc_channels: vec![],
                minitia: Some(Minitia {
                    ty: MinitiaType::MiniWasm,
                    version: "v1.0.0".to_string(),
                }),
            },
            logo_uris: ImageType::PNG("https://test.com/logo.png".to_string()),
            slip44: 118,
            bech32_prefix: "test".to_string(),
            network_type: "testnet".to_string(),
            evm_chain_id: None,
        };

        let json = serde_json::to_string(&chain).expect("Failed to serialize");
        let deserialized: Chain = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(chain, deserialized);
    }

    #[test]
    fn test_chainlist_deserialization() {
        let json = r#"[
            {
                "chain_id": "test-1",
                "chain_name": "test1",
                "pretty_name": "Test 1",
                "description": "Test chain 1",
                "website": "https://test1.com",
                "fees": {"fee_tokens": []},
                "apis": {},
                "explorers": [],
                "metadata": {},
                "logo_URIs": {"png": "https://test1.com/logo.png"},
                "slip44": 118,
                "bech32_prefix": "test1",
                "network_type": "testnet"
            },
            {
                "chain_id": "test-2",
                "chain_name": "test2",
                "pretty_name": "Test 2",
                "description": "Test chain 2",
                "website": "https://test2.com",
                "fees": {"fee_tokens": []},
                "apis": {},
                "explorers": [],
                "metadata": {},
                "logo_URIs": {"svg": "https://test2.com/logo.svg"},
                "slip44": 60,
                "bech32_prefix": "test2",
                "network_type": "mainnet"
            }
        ]"#;

        let chain_list: ChainList =
            serde_json::from_str(json).expect("Failed to deserialize chain list");
        assert_eq!(chain_list.0.len(), 2);
        assert_eq!(chain_list.0[0].chain_id, "test-1");
        assert_eq!(chain_list.0[1].chain_id, "test-2");

        // Test logo URI types
        match &chain_list.0[0].logo_uris {
            ImageType::PNG(url) => assert_eq!(url, "https://test1.com/logo.png"),
            _ => panic!("Expected PNG logo"),
        }
        match &chain_list.0[1].logo_uris {
            ImageType::SVG(url) => assert_eq!(url, "https://test2.com/logo.svg"),
            _ => panic!("Expected SVG logo"),
        }
    }

    #[test]
    fn test_optional_fields() {
        let minimal_json = r#"{
            "chain_id": "minimal-1",
            "chain_name": "minimal",
            "pretty_name": "Minimal Chain",
            "description": "A minimal chain",
            "website": "https://minimal.com",
            "fees": {"fee_tokens": []},
            "apis": {},
            "explorers": [],
            "metadata": {},
            "logo_URIs": {"png": "https://minimal.com/logo.png"},
            "slip44": 118,
            "bech32_prefix": "minimal",
            "network_type": "testnet"
        }"#;

        let chain: Chain =
            serde_json::from_str(minimal_json).expect("Failed to deserialize minimal chain");

        assert_eq!(chain.chain_id, "minimal-1");
        assert_eq!(chain.metadata.op_bridge_id, None);
        assert_eq!(chain.metadata.is_l1, None);
        assert_eq!(chain.metadata.minitia, None);
        assert_eq!(chain.evm_chain_id, None);
        assert!(chain.apis.rpc.is_empty());
        assert!(chain.metadata.ibc_channels.is_empty());
    }

    #[test]
    fn test_minitia_types() {
        // Test all minitia types
        let test_cases = vec![
            ("minievm", MinitiaType::MiniEVM),
            ("minimove", MinitiaType::MiniMove),
            ("miniwasm", MinitiaType::MiniWasm),
        ];

        for (type_str, expected_type) in test_cases {
            let json = format!(
                r#"{{
                "type": "{}",
                "version": "v1.0.0"
            }}"#,
                type_str
            );

            let minitia: Minitia =
                serde_json::from_str(&json).expect("Failed to deserialize minitia");
            assert_eq!(minitia.ty, expected_type);
            assert_eq!(minitia.version, "v1.0.0");
        }
    }

    #[test]
    fn test_fee_token_optional_prices() {
        let json = r#"{
            "denom": "utest"
        }"#;

        let fee_token: FeeToken =
            serde_json::from_str(json).expect("Failed to deserialize fee token");
        assert_eq!(fee_token.denom, "utest");
        assert_eq!(fee_token.fixed_min_gas_price, None);
        assert_eq!(fee_token.low_gas_price, None);
        assert_eq!(fee_token.average_gas_price, None);
        assert_eq!(fee_token.high_gas_price, None);
    }

    #[test]
    fn test_endpoint_optional_fields() {
        let json = r#"{
            "address": "https://test.com"
        }"#;

        let endpoint: Endpoint =
            serde_json::from_str(json).expect("Failed to deserialize endpoint");
        assert_eq!(endpoint.address, "https://test.com");
        assert_eq!(endpoint.provider, None);
        assert_eq!(endpoint.authorized_user, None);
    }
}
