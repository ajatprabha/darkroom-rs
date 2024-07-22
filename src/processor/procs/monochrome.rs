use async_trait::async_trait;
use crate::handler::query::{MonoChrome as QueryMonoChrome};
use crate::processor::{Image, Processor};
use crate::processor::error::Error;

pub const BLACK: QueryMonoChrome = QueryMonoChrome::RGB(0, 0, 0);
pub const BLACK_ALPHA: QueryMonoChrome = QueryMonoChrome::ARGB(0, 0, 0, 0);

pub struct MonoChrome {
    pub color: QueryMonoChrome,
}

#[async_trait]
impl Processor for MonoChrome {
    async fn process(&self, image: &mut Image) -> Result<(), Error> {
        if self.color != BLACK || self.color != BLACK_ALPHA {
            return Ok(());
        }

        image.extend(image.grayscale());

        Ok(())
    }
}
