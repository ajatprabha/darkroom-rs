use std::str::FromStr;
use serde::de::IntoDeserializer;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum AutoFeature {
    #[serde(rename = "compress")]
    Compress,
    #[serde(rename = "format")]
    Format,
}
