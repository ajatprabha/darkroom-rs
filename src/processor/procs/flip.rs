use crate::handler::query::Flip as QueryFlip;
use crate::processor::{Image, Processor};
use crate::processor::error::Error;

pub enum FlipType {
    Horizontal,
    Vertical,
    VerticalHorizontal,
}

impl From<QueryFlip> for FlipType {
    fn from(flip: QueryFlip) -> Self {
        match flip {
            QueryFlip::Horizontal => FlipType::Horizontal,
            QueryFlip::Vertical => FlipType::Vertical,
            QueryFlip::VerticalHorizontal => FlipType::VerticalHorizontal,
        }
    }
}

pub struct Flip {
    pub flip_type: FlipType,
}

impl Processor for Flip {
    fn process(&self, image: &mut Image) -> Result<(), Error> {
        *image = match self.flip_type {
            FlipType::Horizontal => image.fliph(),
            FlipType::Vertical => image.flipv(),
            FlipType::VerticalHorizontal => image.fliph().flipv(),
        }.into();
        Ok(())
    }
}
