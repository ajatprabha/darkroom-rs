pub mod getter;
pub(crate) mod errors;
mod types;
pub(crate) mod webfolder;
pub(crate) use getter::Getter;
pub(crate) use types::{GetRequest, GetRequestOptions, GetResponse};
