use std::fmt::Debug;
use std::time::{Duration, SystemTime};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};

pub struct ByteRange(String);

pub struct GetRequestOptions {
    pub range: ByteRange,
}

pub struct GetRequest {
    pub path: String,
    pub options: Option<GetRequestOptions>,
}

pub struct Metadata {
    pub content_type: Option<String>,
    pub last_modified: Option<SystemTime>,
    pub cache_control: Option<Duration>,
}

pub struct GetResponse {
    pub content: Vec<u8>,
    pub metadata: Option<Metadata>,
}

impl GetResponse {
    pub fn with_cache_control(mut self, cache_control: Duration) -> Self {
        if let Some(ref mut metadata) = self.metadata {
            metadata.cache_control = Some(cache_control);
        } else {
            self.metadata = Some(Metadata {
                content_type: None,
                last_modified: None,
                cache_control: Some(cache_control),
            });
        }
        self
    }
}

impl IntoResponse for GetResponse {
    fn into_response(self) -> Response {
        let content_length = self.content.len();
        let mut res = Response::new(self.content.into());

        res.headers_mut().insert("Vary", "Accept".parse().unwrap());
        res.headers_mut().insert("Content-Length", content_length.to_string().parse().unwrap());
        res.headers_mut().insert("Content-Disposition", "inline".parse().unwrap());

        if let Some(metadata) = self.metadata {
            if let Some(content_type) = metadata.content_type {
                res.headers_mut().insert("Content-Type", content_type.parse().unwrap());
            }

            if let Some(cache_control) = metadata.cache_control {
                res.headers_mut()
                    .insert("Cache-Control", format!("max-age={}", cache_control.as_secs())
                        .parse().unwrap());
            }
        }

        res
    }
}
