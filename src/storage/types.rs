use std::fmt::Debug;
use std::time::{Duration, SystemTime};

#[derive(Debug, PartialEq)]
pub struct ByteRange(String);

#[derive(Debug, PartialEq)]
pub struct GetRequestOptions {
    pub range: ByteRange,
}

#[derive(Debug, PartialEq)]
pub struct GetRequest {
    pub path: String,
    pub options: Option<GetRequestOptions>,
}

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub content_type: Option<String>,
    pub last_modified: Option<SystemTime>,
    pub cache_control: Option<Duration>,
}

#[derive(PartialEq)]
pub struct GetResponse {
    pub content: Vec<u8>,
    pub metadata: Option<Metadata>,
}
