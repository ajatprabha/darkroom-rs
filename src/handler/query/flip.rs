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

#[cfg(test)]
mod tests {
    use serde::de::{Error, IntoDeserializer};
    use serde::de::value::{BoolDeserializer, StrDeserializer};
    use super::*;

    type E = de::value::Error;

    #[test]
    fn test_deserialize() {
        let testcases = vec![
            ("h", Flip::Horizontal),
            ("v", Flip::Vertical),
            ("vh", Flip::VerticalHorizontal),
            ("hv", Flip::VerticalHorizontal),
        ];

        for testcase in testcases {
            assert_eq!(
                Flip::deserialize::<StrDeserializer<E>>(testcase.0.into_deserializer()),
                Ok(testcase.1),
            );
        }
    }

    #[test]
    fn test_deserialize_error() {
        assert_eq!(
            Flip::deserialize::<StrDeserializer<E>>("".into_deserializer()),
            Err(Error::custom("invalid flip value"))
        );
    }

    #[test]
    fn test_expect() {
        assert_eq!(
            Flip::deserialize::<BoolDeserializer<E>>(true.into_deserializer()),
            Err(Error::custom(
                "invalid type: boolean `true`, expected valid values: h, v, vh, hv"
            ))
        );
    }
}
