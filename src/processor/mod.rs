mod image;
pub(crate) mod error;
mod chain;
mod procs;

pub(crate) mod chainer;

use std::any::type_name;
pub use crate::processor::image::Image;
use crate::processor::error::Error;

pub trait Processor {
    fn name<'a>(&self) -> &'a str { type_name::<Self>() }

    fn process(&self, image: &mut Image) -> Result<(), Error>;
}
