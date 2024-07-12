use std::str::FromStr;
use serde::de::IntoDeserializer;
use serde::Deserialize;
use crate::handler::query::vec::CommaSeparatedVec;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum AutoFeature {
    #[serde(rename = "compress")]
    Compress,
    #[serde(rename = "format")]
    Format,
}

impl AutoFeature {
    pub fn from_iter<I: IntoIterator<Item=AutoFeature>>(iter: I) -> CommaSeparatedVec<AutoFeature> {
        CommaSeparatedVec::from_iter(iter)
    }
}
