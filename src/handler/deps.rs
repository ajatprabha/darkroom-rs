use std::sync::Arc;
use std::time::Duration;
use prometheus::Registry;
use crate::processor::chainer::ChainProcessor;
use crate::storage;

#[derive(Clone)]
pub struct Dependencies {
    pub registry: Arc<Registry>,
    pub storage: Arc<dyn storage::Getter + Send + Sync>,
    pub processor: Arc<ChainProcessor>,
    pub cache_time: Duration,
}
