#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error: {0}")]
    Generic(String),
}
