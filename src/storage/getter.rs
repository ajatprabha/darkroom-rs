use async_trait::async_trait;
use crate::storage::errors::Error;
use crate::storage::types::{GetRequest, GetResponse};

#[cfg(test)]
use mockall::{automock};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Getter {
    async fn get(&self, req: GetRequest) -> Result<GetResponse, Error>;
}
