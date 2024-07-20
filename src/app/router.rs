use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use opentelemetry::metrics::MeterProvider;
use prometheus::Registry;
use crate::config::{Config, SourceKind};
use crate::error::Error;
use crate::handler::Dependencies;
use crate::processor::chainer::ChainProcessor;
use crate::{handler, storage};
use crate::storage::webfolder::WebFolderGetter;
use crate::prelude::Result;

pub struct Router {
    inner: axum::Router,
}

impl Deref for Router {
    type Target = axum::Router;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl DerefMut for Router {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl Router {
    pub fn new(cfg: &Config, reg: Registry) -> Result<Self> {
        let storage: Arc<dyn storage::Getter + Send + Sync> = match cfg.source.kind {
            SourceKind::WebFolder => {
                let web_folder = cfg.source.web_folder.clone()
                    .ok_or(Error::Generic("WebFolder source is not configured".into()))?;
                let prefix = cfg.source.path_prefix.clone();
                let prefix = prefix.map(|s| Box::leak(s.into_boxed_str()) as &str);
                Arc::new(
                    WebFolderGetter::new(web_folder.base_url, prefix)
                )
            }
        };

        let meter = Arc::new(
            opentelemetry::global::meter_provider()
                .meter("darkroom-rs")
        );
        let deps = Arc::new(Dependencies {
            registry: Arc::new(reg),
            storage,
            processor: Arc::new(ChainProcessor::new(meter)),
            cache_time: cfg.handler.response.cache_duration,
        });

        Ok(Self { inner: Self::build_router(deps) })
    }

    fn build_router(deps: Arc<Dependencies>) -> axum::Router {
        axum::Router::new()
            .route("/*path", get(handler::image))
            .route("/metrics", get(metrics_handler))
            .layer(Extension(deps))
    }
}

async fn metrics_handler(
    Extension(deps): Extension<Arc<Dependencies>>,
) -> std::result::Result<impl IntoResponse, StatusCode> {
    let body = prometheus::TextEncoder::new()
        .encode_to_string(&deps.registry.gather())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, body))
}
