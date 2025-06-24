use serde::{Deserialize, Serialize};

structstruck::strike! {
    #[structstruck::each[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]]
    pub struct Profile {
        #[serde(rename = "$schema")]
        pub schema: Option<String>,
        pub name: String,
        pub pretty_name: String,
        pub category: String,
        #[serde(default)]
        pub tags: Vec<String>,
        pub l2: Option<bool>,
        pub description: String,
        pub summary: Option<String>,
        pub logo: String,
        pub color: String,
        pub status: String,
        pub vip: Option<pub struct Vip {
            pub forum_url: String,
            pub actions: Vec<pub struct VipAction {
                pub title: String,
                pub description: String,
            }>,
        }>,
        pub social: pub struct Social {
            pub website: String,
            pub twitter: String,
        },
    }

    #[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]
    pub struct ProfileList(pub Vec<Profile>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_deserialization() {
        let sample_json = r##"
        {
            "$schema": "../profile.schema.json",
            "name": "bfb",
            "pretty_name": "Battle for Blockchain",
            "category": "Gaming",
            "l2": true,
            "description": "Fully onchain battle world, where value gets redistributed based on players performance.",
            "summary": "Battle world, with value redistribution protocols.",
            "logo": "https://registry.initia.xyz/images/culinaris.png",
            "color": "#46BEFF",
            "status": "live",
            "vip": {
                "forum_url": "https://forum.initia.xyz/t/whitelist-bfb-on-vip/96",
                "actions": [
                    {
                        "title": "Conquer Culinaris",
                        "description": "Winning as much $BFB in-game for each war."
                    }
                ]
            },
            "social": {
                "website": "https://battleforblockchain.com",
                "twitter": "https://x.com/battleforblock"
            }
        }
        "##;

        let profile: Profile =
            serde_json::from_str(sample_json).expect("Failed to deserialize profile");

        assert_eq!(profile.name, "bfb");
        assert_eq!(profile.pretty_name, "Battle for Blockchain");
        assert_eq!(profile.category, "Gaming");
        assert_eq!(profile.l2, Some(true));
        assert!(profile.vip.is_some());
        assert_eq!(profile.social.website, "https://battleforblockchain.com");
    }

    #[test]
    fn test_profile_without_vip() {
        let sample_json = r##"{
            "name": "minity",
            "pretty_name": "Minity",
            "category": "Portfolio",
            "tags": ["Tooling"],
            "l2": false,
            "description": "Track your assets, DeFi Positions, and NFTs across every interwoven rollup.",
            "summary": "Track your assets and positions across L1 and L2s",
            "logo": "https://registry.initia.xyz/images/minity.png",
            "color": "#FFD700",
            "status": "live",
            "social": {
                "website": "https://minity.xyz",
                "twitter": "https://x.com/minity_xyz"
            }
        }"##;

        let profile: Profile =
            serde_json::from_str(sample_json).expect("Failed to deserialize profile");

        assert_eq!(profile.name, "minity");
        assert_eq!(profile.tags, vec!["Tooling"]);
        assert!(profile.vip.is_none());
        assert_eq!(profile.social.twitter, "https://x.com/minity_xyz");
    }
}
