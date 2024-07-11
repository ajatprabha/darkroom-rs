use std::fmt::Debug;
use std::time::{Duration, SystemTime};

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
