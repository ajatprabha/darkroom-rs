use std::sync::Arc;
use async_trait::async_trait;
use axum::http::StatusCode;
use reqwest::Client;
use crate::config::config::Url;
use crate::storage::errors::Error;
use crate::storage::getter::Getter;
use crate::storage::types::{GetRequest, GetResponse};

pub struct WebFolderGetter<'a> {
    base_url: Url,
    path_prefix: Option<&'a str>,
    client: Arc<Client>,
}

impl<'a> WebFolderGetter<'a> {
    pub fn new(base_url: Url, path_prefix: Option<&'a str>) -> Self {
        WebFolderGetter {
            base_url,
            path_prefix,
            client: Arc::new(Client::new()),
        }
    }
}

#[async_trait]
impl<'a> Getter for WebFolderGetter<'a> {
    async fn get(&self, req: GetRequest) -> Result<GetResponse, Error> {
        let url = format!(
            "{}://{}{}",
            self.base_url.scheme(),
            self.base_url.host(),
            if let Some(prefix) = self.path_prefix {
                format!("{}/{}", prefix, req.path)
            } else {
                req.path
            }
        );

        let res = self.client.get(&url).send().await.map_err(|err| {
            Error::Upstream {
                status_code: err.status().map(|s| s.as_u16()),
                message: err.to_string(),
            }
        })?;

        if res.status() != StatusCode::OK {
            return Err(Error::Upstream {
                status_code: Some(res.status().as_u16()),
                message: res.text().await.unwrap_or_default(),
            })
        }

        let body = res.bytes().await.map_err(Error::Reqwest)?;

        Ok(GetResponse {
            content: body.to_vec(),
            metadata: None,
        })
    }
}
