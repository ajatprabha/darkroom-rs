use crate::processor::error::Error;
use crate::processor::{Image, Processor};

pub const MAX_BLUR_RADIUS: u16 = 2000;

pub struct Blur {
    pub radius: u16,
}

impl Processor for Blur {
    fn process(&self, image: &mut Image) -> Result<(), Error> {
        if self.radius == 0 { return Ok(()); }

        let s = if self.radius > MAX_BLUR_RADIUS {
            MAX_BLUR_RADIUS as f32 / 3.0
        } else {
            self.radius as f32 / 3.0
        };

        *image = image.blur(s).into();

        Ok(())
    }
}
