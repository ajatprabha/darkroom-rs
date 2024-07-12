use std::collections::HashMap;
use std::time::Duration;
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use axum::response::IntoResponse;
use image::ImageFormat;
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
                _ => {}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_response() {
        let response = Response {
            image: (vec![1, 2, 3], Some(ImageFormat::Jpeg)),
            cache_time: Duration::from_secs(3600),
        };

        let res = response.into_response();
        assert_eq!(res.status(), 200);
        assert_eq!(res.headers().get("Content-Type").unwrap(), "image/jpeg");
        assert_eq!(res.headers().get("Content-Length").unwrap(), "3");
        assert_eq!(res.headers().get("Cache-Control").unwrap(), "public, max-age=3600");
    }

    #[test]
    fn test_into_response_content_types() {
        let testcases = vec![
            (ImageFormat::Png, Some("image/png")),
            (ImageFormat::Jpeg, Some("image/jpeg")),
            (ImageFormat::WebP, Some("image/webp")),
            // Catch all
            (ImageFormat::Farbfeld, None),
        ];

        for testcase in testcases {
            let response = Response {
                image: (vec![1, 2, 3], Some(testcase.0)),
                cache_time: Duration::from_secs(3600),
            };

            let res = response.into_response();
            assert_eq!(
                res.headers().get("Content-Type").and_then(|v| v.to_str().ok()),
                testcase.1
            );
        }
    }
}
