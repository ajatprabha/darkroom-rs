use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use axum::Extension;
use axum::routing::get;
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
    pub fn new(cfg: &Config) -> Result<Self> {
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

        let deps = Arc::new(Dependencies {
            storage,
            processor: Arc::new(ChainProcessor {}),
            cache_time: cfg.handler.response.cache_duration,
        });

        Ok(Self { inner: Self::build_router(deps) })
    }

    fn build_router(deps: Arc<Dependencies>) -> axum::Router {
        axum::Router::new()
            .route("/*path", get(handler::image))
            .layer(Extension(deps))
    }
}
