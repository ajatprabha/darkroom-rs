use std::{env};
use std::net::{SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use config::{ConfigError, Environment, File, FileFormat, FileSourceFile};
use serde::{Deserialize};
use crate::config::url::Url;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: Log,
    pub http: HTTP,
    pub source: Source,
    pub handler: Handler,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub kind: SourceKind,
    pub web_folder: Option<WebFolder>,
    pub path_prefix: Option<String>,
    pub network: Network,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum SourceKind {
    WebFolder,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebFolder {
    pub base_url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Network {
    pub config: NetworkConfig,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub enabled: Option<bool>,
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct HTTP {
    pub debug_mode: Option<bool>,
    pub bind_address: SocketAddr,
}

#[derive(Debug, Default, Deserialize)]
pub struct Handler {
    pub response: Response,
}

#[derive(Debug, Default, Deserialize)]
pub struct Response {
    #[serde(with = "humantime_serde")]
    pub cache_duration: Duration,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let home_dir = env::var("HOME").unwrap_or("".to_string());
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        Self::parse(vec![
            File::with_name("src/config/values/default")
                .required(false),
            File::with_name(format!("{}/.darkroom/default", home_dir).as_str())
                .required(false),
            File::with_name(&format!("{}/.darkroom/{}", home_dir, run_mode))
                .required(false),
        ])
    }

    fn parse(sources: Vec<File<FileSourceFile, FileFormat>>) -> Result<Self, ConfigError> {
        sources
            .iter()
            .fold(
                config::Config::builder(),
                |b, src| {
                    b.add_source(src.clone())
                },
            )
            .add_source(Environment::default().separator("__"))
            .build()?.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let cfg = Config::new().unwrap();

        assert_eq!(cfg.log.enabled, None);
        assert_eq!(cfg.log.level, "debug".to_string());
        assert_eq!(cfg.http.debug_mode, None);
        assert_eq!(cfg.http.bind_address, SocketAddr::from_str("127.0.0.1:3000").unwrap());
        assert_eq!(cfg.handler.response.cache_duration, Duration::from_secs(600));

        assert_eq!(cfg.source.kind, SourceKind::WebFolder);
        assert_eq!(cfg.source.web_folder.unwrap().base_url, Url::new("https://example.com").unwrap());
        assert_eq!(cfg.source.path_prefix, Some("/assets".to_string()));
        assert_eq!(cfg.source.network.config.timeout, Duration::from_secs(5));
    }
}
