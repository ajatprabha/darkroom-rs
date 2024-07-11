use std::collections::HashSet;
use std::fmt;
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};

#[derive(Debug, PartialEq)]
pub(crate) enum Crop {
    Top,
    TopLeft,
    TopRight,
    Left,
    Right,
    Bottom,
    BottomLeft,
    BottomRight,
    Center,
}

impl<'de> Deserialize<'de> for Crop {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CropVisitor;

        impl<'de> Visitor<'de> for CropVisitor {
            type Value = Crop;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a comma-separated list of crop values")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let set: HashSet<&str> = value.split(',').collect();
                match set.iter().map(|&s| s).collect::<Vec<_>>().as_slice() {
                    ["top"] => Ok(Crop::Top),
                    ["top", "left"] | ["left", "top"] => Ok(Crop::TopLeft),
                    ["top", "right"] | ["right", "top"] => Ok(Crop::TopRight),
                    ["left"] => Ok(Crop::Left),
                    ["right"] => Ok(Crop::Right),
                    ["bottom"] => Ok(Crop::Bottom),
                    ["bottom", "left"] | ["left", "bottom"] => Ok(Crop::BottomLeft),
                    ["bottom", "right"] | ["right", "bottom"] => Ok(Crop::BottomRight),
                    ["center"] => Ok(Crop::Center),
                    _ => Err(E::custom(format!("invalid crop: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(CropVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::de::IntoDeserializer;
    use serde::de::value::{BoolDeserializer, StrDeserializer};
    use super::*;

    type E = de::value::Error;

    #[test]
    fn test_crop() {
        let testcases = vec![
            ("top", Crop::Top),
            ("left", Crop::Left),
            ("right", Crop::Right),
            ("bottom", Crop::Bottom),
            ("top,left", Crop::TopLeft),
            ("top,right", Crop::TopRight),
            ("left,top", Crop::TopLeft),
            ("right,top", Crop::TopRight),
            ("bottom,left", Crop::BottomLeft),
            ("bottom,right", Crop::BottomRight),
            ("left,bottom", Crop::BottomLeft),
            ("right,bottom", Crop::BottomRight),
            ("center", Crop::Center),
        ];

        for testcase in testcases {
            assert_eq!(
                testcase.1,
                Crop::deserialize::<StrDeserializer<E>>
                    (testcase.0.into_deserializer()).unwrap()
            );
        }
    }

    #[test]
    fn test_error() {
        let err = Crop::deserialize::<StrDeserializer<E>>("invalid".into_deserializer()).unwrap_err();
        assert_eq!(err.to_string(), "invalid crop: invalid");
    }

    #[test]
    fn test_unexpected() {
        let err = Crop::deserialize::<BoolDeserializer<E>>(true.into_deserializer()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid type: boolean `true`, expected a comma-separated list of crop values"
        );
    }
}
