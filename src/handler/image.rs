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
    let res = deps.storage.get(GetRequest { path, options: None })
        .await
        .map_err(|e| e.status_code())?;

    if params.is_noop() {
        return Ok((StatusCode::OK, Response {
            image: (res.content, None),
            cache_time: deps.cache_time,
        }));
    }

    let cursor = Cursor::new(res.content);

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

    Ok((StatusCode::OK, Response {
        image: (buffer.into_inner(), image.format),
        cache_time: deps.cache_time,
    }))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::get;
    use crate::processor::chainer::ChainProcessor;
    use super::*;
    use mockall::predicate::*;
    use crate::storage::GetResponse;
    use crate::storage::getter::MockGetter;
    use tower::ServiceExt;
    use http_body_util::BodyExt;
    use crate::{handler, storage};


    fn deps(mock: MockGetter) -> Arc<Dependencies> {
        Arc::new(Dependencies {
            storage: Arc::new(mock),
            processor: Arc::new(ChainProcessor {}),
            cache_time: Duration::from_secs(300),
        })
    }

    fn router(deps: Arc<Dependencies>) -> axum::Router {
        axum::Router::new()
            .route("/*path", get(handler::image))
            .layer(Extension(deps))
    }

    #[tokio::test]
    async fn noop() {
        let mut mock = MockGetter::new();
        mock.expect_get()
            .with(eq(GetRequest {
                path: "test.jpg".to_string(),
                options: None,
            }))
            .times(1)
            .returning(|_| Ok(GetResponse {
                content: vec![1, 2, 3],
                metadata: None,
            }));

        let res = router(deps(mock))
            .oneshot(
                Request::builder().uri("/test.jpg")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body.len(), 3);
    }

    #[tokio::test]
    async fn storage_get_error() {
        let mut mock = MockGetter::new();
        mock.expect_get()
            .with(eq(GetRequest {
                path: "test.jpg".to_string(),
                options: None,
            }))
            .times(1)
            .returning(|_| Err(storage::errors::Error::Upstream {
                status_code: Some(401),
                message: "Unauthorized".to_string(),
            }));

        let res = router(deps(mock))
            .oneshot(
                Request::builder().uri("/test.jpg")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }
}
