use std::fmt;
use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};

#[derive(Debug, PartialEq)]
pub(crate) struct Rotate(pub f32);

impl<'de> Deserialize<'de> for Rotate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RotateVisitor;

        impl<'de> Visitor<'de> for RotateVisitor {
            type Value = Rotate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a floating angle value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.parse::<f32>() {
                    Ok(angle) => {
                        let angle = angle % 360.0;
                        if angle < 0.0 {
                            Ok(Rotate(angle + 360.0))
                        } else {
                            Ok(Rotate(angle))
                        }
                    }
                    Err(_) => Err(E::custom(format!("invalid angle: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(RotateVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::de::{Error, IntoDeserializer};
    use serde::de::value::{BoolDeserializer, StrDeserializer};
    use super::*;

    type E = de::value::Error;

    #[test]
    fn test_rotate() {
        let testcases: Vec<(&str, f32)> = vec![
            ("0", 0.0),
            ("90", 90.0),
            ("-90", 270.0),
            ("180", 180.0),
            ("-180", 180.0),
            ("270", 270.0),
            ("-270", 90.0),
            ("360", 0.0),
            ("-360", 0.0),
            ("720", 0.0),
            ("-540", 180.0),
        ];

        for testcase in testcases {
            assert_eq!(
                Ok(Rotate(testcase.1)),
                Rotate::deserialize::<StrDeserializer<E>>(testcase.0.into_deserializer())
            );
        }
    }

    #[test]
    fn test_rotate_error() {
        assert_eq!(
            Rotate::deserialize::<StrDeserializer<E>>("".into_deserializer()),
            Err(Error::custom("invalid angle: "))
        );
    }

    #[test]
    fn test_rotate_expect() {
        assert_eq!(
            Rotate::deserialize::<BoolDeserializer<E>>(true.into_deserializer()),
            Err(Error::custom("invalid type: boolean `true`, expected a floating angle value"))
        );
    }
}
