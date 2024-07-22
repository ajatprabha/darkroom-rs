mod image;
pub(crate) mod error;
mod chain;
mod procs;

pub(crate) mod chainer;

#[cfg(feature = "gpu")]
pub(crate) mod gpuprocs;

use std::any::type_name;
pub use crate::processor::image::Image;
use crate::processor::error::Error;

#[async_trait::async_trait]
pub trait Processor {
    fn name<'a>(&self) -> &'a str { type_name::<Self>() }

    async fn process(&self, image: &mut Image) -> Result<(), Error>;
}
