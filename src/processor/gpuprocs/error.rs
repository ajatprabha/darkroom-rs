#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("adapter not found")]
    AdapterNotFound,
    #[error(transparent)]
    DeviceError(#[from] wgpu::RequestDeviceError),
}
