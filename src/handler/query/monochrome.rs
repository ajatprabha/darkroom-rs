use std::fmt;
use serde::{de, Deserialize, Deserializer};
use serde::de::Visitor;

#[derive(Debug, PartialEq)]
pub(crate) enum MonoChrome {
    RGB(u8, u8, u8),
    ARGB(u8, u8, u8, u8),
}

impl<'de> Deserialize<'de> for MonoChrome {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonoChromeVisitor;

        impl<'de> Visitor<'de> for MonoChromeVisitor {
            type Value = MonoChrome;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a 6-digit hex color")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let hex = value.trim_start_matches('#');

                fn parse_hex<E: de::Error>(hex: &str) -> Result<u8, E> {
                    u8::from_str_radix(hex, 16).map_err(E::custom)
                }

                match hex.len() {
                    8 => {
                        Ok(MonoChrome::ARGB(
                            parse_hex(&hex[0..2])?,
                            parse_hex(&hex[2..4])?,
                            parse_hex(&hex[4..6])?,
                            parse_hex(&hex[6..8])?,
                        ))
                    }
                    6 => {
                        Ok(MonoChrome::RGB(
                            parse_hex(&hex[0..2])?,
                            parse_hex(&hex[2..4])?,
                            parse_hex(&hex[4..6])?,
                        ))
                    }
                    4 => {
                        Ok(MonoChrome::ARGB(
                            parse_hex(&format!("{0}{0}", &hex[0..1]))?,
                            parse_hex(&format!("{0}{0}", &hex[1..2]))?,
                            parse_hex(&format!("{0}{0}", &hex[2..3]))?,
                            parse_hex(&format!("{0}{0}", &hex[3..4]))?,
                        ))
                    }
                    3 => {
                        Ok(MonoChrome::RGB(
                            parse_hex(&format!("{0}{0}", &hex[0..1]))?,
                            parse_hex(&format!("{0}{0}", &hex[1..2]))?,
                            parse_hex(&format!("{0}{0}", &hex[2..3]))?,
                        ))
                    }
                    _ => Err(E::custom("invalid hex color")),
                }
            }
        }

        deserializer.deserialize_str(MonoChromeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::de::value::StringDeserializer;
    use super::*;

    type E = de::value::Error;

    #[test]
    fn test_monochrome() {
        let testcases: Vec<(&str, MonoChrome)> = vec![
            ("#000000", MonoChrome::RGB(0, 0, 0)),
            ("#ffffff", MonoChrome::RGB(255, 255, 255)),
            ("FFFFFF", MonoChrome::RGB(255, 255, 255)),
            ("#000", MonoChrome::RGB(0, 0, 0)),
            ("#f00", MonoChrome::RGB(255, 0, 0)),
            ("#0f0", MonoChrome::RGB(0, 255, 0)),
            ("#00f", MonoChrome::RGB(0, 0, 255)),
            ("#ff0000", MonoChrome::RGB(255, 0, 0)),
            ("38141494", MonoChrome::ARGB(56, 20, 20, 148)),
            ("A6000000", MonoChrome::ARGB(166, 0, 0, 0)),
            ("A000", MonoChrome::ARGB(170, 0, 0, 0)),
        ];

        for (input, expected) in testcases {
            let deserializer = StringDeserializer::<E>::new(input.to_owned());
            let actual = MonoChrome::deserialize(deserializer).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
