use std::ops::{Deref, DerefMut};
use serde::Deserialize;
use url::{ParseError, Url as ImplUrl};
use crate::prelude::W;

#[derive(Debug, PartialEq, Clone)]
pub struct Url(ImplUrl);

impl Url {
    pub fn new(url: &str) -> Result<Url, ParseError> { Ok(Url(ImplUrl::parse(url)?)) }
}

impl Deref for Url {
    type Target = ImplUrl;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl TryFrom<W<&String>> for Url {
    type Error = ParseError;
    fn try_from(val: W<&String>) -> Result<Url, ParseError> { Ok(Url::new(&val.0)?) }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let url = String::deserialize(deserializer)?;
        Ok(W(&url).try_into().map_err(serde::de::Error::custom)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let url = Url::new("https://example.com").unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host_str(), Some("example.com"));
    }

    #[test]
    fn test_set() {
        let mut url = Url::new("https://example.com").unwrap();
        url.set_scheme("http").unwrap();
        assert_eq!(url.scheme(), "http");
    }
}
