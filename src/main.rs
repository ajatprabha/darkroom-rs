mod config;
mod storage;
mod error;
mod prelude;
mod handler;
mod app;
mod processor;

use std::ops::Deref;
use crate::config::Config;
use crate::app::Server;
use crate::prelude::Result;

use opentelemetry::metrics::{MeterProvider, Unit};
use opentelemetry_sdk::metrics::{Aggregation, Instrument, new_view, SdkMeterProvider, Stream};
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
        .with_view(new_view(
            Instrument::new()
                .name("*_duration")
                .unit(Unit::new("s")),
            Stream::new().aggregation(
                Aggregation::ExplicitBucketHistogram {
                    boundaries: vec![
                        0.001, 0.002, 0.005, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0,
                    ],
                    record_min_max: true,
                },
            ),
        ).map_err(|e| Generic(e.to_string()))?)
        .build();

    opentelemetry::global::set_meter_provider(provider);

    Server::new(&Config::new()?, registry)?.serve().await
}
