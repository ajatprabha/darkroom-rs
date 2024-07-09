use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Fit {
    #[serde(rename = "crop")]
    Crop,
    #[serde(rename = "scale")]
    Scale,
}
