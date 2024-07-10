use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use axum::{Extension, Router};
use axum::routing::get;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Notify;
use crate::config::{Config, SourceKind};
use crate::{handler, storage};

use crate::handler::Dependencies;
use crate::storage::webfolder::WebFolderGetter;

use crate::prelude::*;

pub(crate) struct Server {
    address: SocketAddr,
    router: Router,
}

impl Server {
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
            cache_time: cfg.handler.response.cache_duration,
        });

        let router = Router::new()
            .route("/*path", get(handler::image))
            .layer(Extension(deps));

        Ok(Self { router, address: cfg.http.bind_address })
    }

    pub async fn serve(self) -> Result<()> {
        let notify = Arc::new(Notify::new());
        let shutdown_counter = Arc::new(AtomicUsize::new(0));

        let nc = notify.clone();

        let listener = TcpListener::bind(self.address).await?;
        let s = axum::serve(listener, self.router)
            .with_graceful_shutdown(async move { nc.notified().await });

        let sig_handler = Server::shutdown_sig(notify.clone(), shutdown_counter.clone()).await?;
        tokio::spawn(sig_handler);

        s.await?;

        Ok(())
    }

    async fn shutdown_sig(notify: Arc<Notify>, shutdown_counter: Arc<AtomicUsize>) -> Result<impl Future<Output=()>> {
        let mut sigint = signal(SignalKind::interrupt()).map_err(Error::IO)?;
        let mut sigterm = signal(SignalKind::terminate()).map_err(Error::IO)?;

        return Ok(async move {
            loop {
                tokio::select! {
                    _ = sigint.recv() => Self::handle_sig(shutdown_counter.clone(), notify.clone()).await,
                    _ = sigterm.recv() => Self::handle_sig(shutdown_counter.clone(), notify.clone()).await,
                }
            }
        });
    }

    async fn handle_sig(shutdown_counter: Arc<AtomicUsize>, notify: Arc<Notify>) {
        let count = shutdown_counter.fetch_add(1, Ordering::SeqCst);
        if count == 0 {
            // TODO: Replace with logger
            println!("Graceful shutdown initiated. Press CTRL+C again to force shutdown.");
            notify.notify_one();
        } else {
            println!("Force shutdown initiated.");
            std::process::exit(0);
        }
    }
}
