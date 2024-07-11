mod image;
pub(crate) mod error;
mod chain;
mod procs;

pub(crate) mod chainer;

pub use crate::processor::image::Image;
use crate::processor::error::Error;

pub trait Processor {
    fn process(&self, image: &mut Image) -> Result<(), Error>;
}
