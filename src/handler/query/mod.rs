mod crop;
mod fit;
mod params;
mod rotate;
mod auto;
mod vec;
mod monochrome;
mod flip;

pub use params::ProcessParams;
pub use fit::Fit;
pub(crate) use crop::Crop;
pub(crate) use flip::Flip;
pub(crate) use rotate::Rotate;
pub(crate) use auto::AutoFeature;
pub(crate) use monochrome::MonoChrome;
