use crate::processor;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error: {0}")]
    Generic(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    Image(#[from] processor::error::Error),

    #[cfg(feature = "gpu")]
    #[error(transparent)]
    GPU(#[from] processor::gpuprocs::error::Error),
}
