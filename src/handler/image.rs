use std::io::Cursor;
use std::sync::Arc;
use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use image::ImageFormat;
use image::io::Reader as ImageReader;
use crate::handler::Dependencies;
use crate::storage::GetRequest;
use crate::handler::query::ProcessParams;
use crate::handler::response::Response;
use crate::processor::Image;

pub async fn image(
    Extension(deps): Extension<Arc<Dependencies>>,
    Path(path): Path<String>,
    Query(params): Query<ProcessParams>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = deps.storage.get(GetRequest {
        path,
        options: None,
    }).await;

    if let Err(err) = res {
        return Err(err.status_code());
    }

    if res.is_ok() {
        if params.is_noop() {
            return Ok((StatusCode::OK, Response {
                image: (res.unwrap().content, None),
                cache_time: deps.cache_time,
            }));
        }

        let cursor = Cursor::new(res.unwrap().content);

        let reader = ImageReader::new(cursor)
            .with_guessed_format();

        if let Err(_) = reader {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let reader = reader.unwrap();
        let format = reader.format();

        let decoded = reader.decode();
        if let Err(_) = decoded {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        let decoded = decoded.unwrap();

        let mut image = if format.is_none() {
            Image::new(decoded)
        } else {
            Image::format(decoded, format.unwrap())
        };

        if deps.processor.process(&mut image, params).is_err() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }

        let mut buffer = Cursor::new(Vec::new());
        if let Err(e) = image.write_to(&mut buffer, format.unwrap_or_else(|| ImageFormat::Jpeg)) {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        return Ok((StatusCode::OK, Response {
            image: (buffer.into_inner(), image.format),
            cache_time: deps.cache_time,
        }));
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
