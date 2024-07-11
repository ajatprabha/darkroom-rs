use serde::{de, Deserialize, Deserializer};
use serde::de::Visitor;

#[derive(Debug, PartialEq)]
pub enum Flip {
    Horizontal,
    Vertical,
    VerticalHorizontal,
}

impl<'de> Deserialize<'de> for Flip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FlipVisitor)
    }
}

struct FlipVisitor;

impl<'de> Visitor<'de> for FlipVisitor {
    type Value = Flip;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("valid values: h, v, vh, hv")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "h" => Ok(Flip::Horizontal),
            "v" => Ok(Flip::Vertical),
            "vh" => Ok(Flip::VerticalHorizontal),
            "hv" => Ok(Flip::VerticalHorizontal),
            _ => Err(de::Error::custom("invalid flip value")),
        }
    }
}
