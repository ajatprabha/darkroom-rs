use std::collections::HashMap;
use std::time::Duration;
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use axum::response::IntoResponse;
use image::ImageFormat;
use crate::processor::Image;
use axum::response::Response as AxumResponse;

pub struct Response {
    pub image: (Vec<u8>, Option<ImageFormat>),
    pub cache_time: Duration,
}

impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        let mut headers = HashMap::from([
            ("Vary", "Accept".to_string()),
            ("Content-Disposition", "inline".to_string()),
            ("Cache-Control", format!("public, max-age={}", self.cache_time.as_secs())),
        ]);

        let image = self.image.0;
        let content_length = image.len();
        headers.insert("Content-Length", content_length.to_string());
        let mut res = AxumResponse::new(image.into());

        if let Some(fmt) = self.image.1 {
            match fmt {
                ImageFormat::Png => {
                    headers.insert("Content-Type", "image/png".to_string());
                }
                ImageFormat::Jpeg => {
                    headers.insert("Content-Type", "image/jpeg".to_string());
                }
                ImageFormat::WebP => {
                    headers.insert("Content-Type", "image/webp".to_string());
                }
                _ => {
                    headers.insert("Content-Type", "image/jpeg".to_string());
                }
            }
        }

        let mut header_map = HeaderMap::new();

        for (key, value) in headers {
            let header_name = key.parse::<HeaderName>().expect("Invalid header name");
            let header_value = HeaderValue::from_str(&value).expect("Invalid header value");
            header_map.insert(header_name, header_value);
        }

        res.headers_mut().extend(header_map);

        res
    }
}
