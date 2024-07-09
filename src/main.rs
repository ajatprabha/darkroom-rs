mod config;
mod storage;
mod error;
mod prelude;
mod handler;

use std::sync::Arc;
use axum::{Extension, Router, ServiceExt};
use axum::response::IntoResponse;
use axum::routing::get;
use tokio::net::TcpListener;
use config::Config;
use crate::config::SourceKind;
use crate::handler::Dependencies;
use crate::storage::webfolder::WebFolderGetter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Config::new()?;

    let storage: Arc<dyn storage::Getter + Send + Sync> = match cfg.source.kind {
        SourceKind::WebFolder => {
            let web_folder = cfg.source.web_folder.ok_or("WebFolder source is not configured")?;
            let prefix = cfg.source.path_prefix.clone();
            let prefix = prefix.map(|s| Box::leak(s.into_boxed_str()) as &str);
            Arc::new(
                WebFolderGetter::new(web_folder.base_url, prefix)
            )
        }
    };

    let deps = Arc::new(Dependencies::new {
        storage,
        cache_time: cfg.handler.response.cache_duration,
    });

    let app = Router::new().route("/*path", get(handler::image)).layer(Extension(deps));

    let listener = TcpListener::bind(cfg.http.bind_address).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
