mod config;
mod storage;
mod error;
mod prelude;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use axum::{Extension, Router, ServiceExt};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use tokio::net::TcpListener;
use config::Config;
use crate::config::SourceKind;
use crate::storage::{GetRequest};
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

    let deps = Arc::new(Dependencies {
        storage,
        cache_time: cfg.handler.response.cache_duration,
    });

    let app = Router::new().route("/*path", get(image_handler)).layer(Extension(deps));

    let listener = TcpListener::bind(cfg.http.bind_address).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[derive(Clone)]
struct Dependencies {
    storage: Arc<dyn storage::Getter + Send + Sync>,
    cache_time: Duration,
}

async fn image_handler(
    Extension(deps): Extension<Arc<Dependencies>>,
    Path(path): Path<String>,
    Query(_query_params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = deps.storage.get(GetRequest {
        path,
        options: None,
    }).await;

    if let Err(err) = res {
        return Err(err.status_code());
    }

    Ok((StatusCode::OK, res.map_err(|err| err.status_code())?.with_cache_control(deps.cache_time)))
}
