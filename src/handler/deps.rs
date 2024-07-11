use std::sync::Arc;
use std::time::Duration;
use crate::processor::chainer::ChainProcessor;
use crate::storage;

#[derive(Clone)]
pub struct Dependencies {
    pub storage: Arc<dyn storage::Getter + Send + Sync>,
    pub processor: Arc<ChainProcessor>,
    pub cache_time: Duration,
}
