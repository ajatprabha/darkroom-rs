use crate::processor::{Image, Processor};
use crate::processor::error::Error;

pub struct Rotate {
    pub degrees: f32,
}


impl Processor for Rotate {
    fn process(&self, image: &mut Image) -> Result<(), Error> {
        if self.degrees == 0.0 { return Ok(()); }

        let rotated = match self.degrees {
            90.0 => Some(image.rotate90()),
            180.0 => Some(image.rotate180()),
            270.0 => Some(image.rotate270()),
            _ => None
        };

        if let Some(rotated) = rotated {
            *image = rotated.into();
        }

        Ok(())
    }
}
