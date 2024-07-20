mod config;
mod storage;
mod error;
mod prelude;
mod handler;
mod app;
mod processor;

use crate::config::Config;
use crate::app::Server;
use crate::prelude::Result;

use opentelemetry::metrics::MeterProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use prometheus::{Registry};
use crate::error::Error::Generic;

#[tokio::main]
async fn main() -> Result<()> {
    let registry = Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build().map_err(|_| Generic("Failed to build prometheus exporter".to_string()))?;
    let provider = SdkMeterProvider::builder()
        .with_reader(exporter)
        .build();

    opentelemetry::global::set_meter_provider(provider);

    Server::new(&Config::new()?, registry)?.serve().await
}
