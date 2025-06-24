use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]
pub enum ImageType {
    #[serde(rename = "png")]
    PNG(String),
    #[serde(rename = "svg")]
    SVG(String),
}
