mod config;
mod storage;
mod error;
mod prelude;
mod handler;
mod app;
mod processor;

use std::io::Write;
use env_logger::{TimestampPrecision, WriteStyle};
use log::LevelFilter;
use crate::config::Config;
use crate::app::Server;
use crate::prelude::Result;
use crate::app::router::Router;

use opentelemetry::metrics::Unit;
use opentelemetry_sdk::metrics::{Aggregation, Instrument, new_view, SdkMeterProvider, Stream};
use prometheus::{Registry};
use crate::error::Error::Generic;

#[cfg(feature = "gpu")]
use crate::processor::gpuprocs::GPUBackend;

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

    let cfg = Config::new()?;

    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Millis))
        .write_style(WriteStyle::Always)
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}: {}",
                buf.timestamp(),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    #[cfg(feature = "gpu")]
    let gpu_backend = GPUBackend::new().await?;
    #[cfg(feature = "gpu")]
    let router = Router::gpu(&cfg, registry, gpu_backend)?;

    #[cfg(not(feature = "gpu"))]
    let router = Router::new(&cfg, registry)?;

    Server {
        router: router.clone(),
        address: cfg.http.bind_address,
    }.serve().await
}
