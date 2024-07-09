use std::fmt;
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};

#[derive(Debug, PartialEq)]
pub(crate) struct Rotate(f32);

impl<'de> Deserialize<'de> for Rotate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RotateVisitor;

        impl<'de> Visitor<'de> for RotateVisitor {
            type Value = Rotate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an angle value")
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
    use serde::de::IntoDeserializer;
    use serde::de::value::StringDeserializer;
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
            assert_eq!(Rotate::deserialize::<StringDeserializer<E>>(
                testcase.0.to_string().into_deserializer()
            ).unwrap(), Rotate(testcase.1));
        }
    }
}