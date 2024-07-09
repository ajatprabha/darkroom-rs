use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::handler::Dependencies;
use crate::storage::GetRequest;

pub async fn image(
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
