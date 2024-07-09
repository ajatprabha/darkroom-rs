use std::sync::Arc;
use std::time::Duration;
use crate::storage;

#[derive(Clone)]
pub struct Dependencies {
    pub storage: Arc<dyn storage::Getter + Send + Sync>,
    pub cache_time: Duration,
}
