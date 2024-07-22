use wgpu::BufferAsyncError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error: {0}")]
    Generic(String),
    #[error("processor: gpu error")]
    GPUError,
    #[error(transparent)]
    BufferError(#[from] BufferAsyncError),
    #[error("processor: failed to create image buffer")]
    ImageBufferCreateError,
}
