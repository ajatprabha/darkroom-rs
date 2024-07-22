use async_trait::async_trait;
use crate::processor::error::Error;
use crate::processor::{Image, Processor};

pub const MAX_BLUR_RADIUS: u16 = 2000;

pub struct Blur {
    pub radius: u16,
}

#[async_trait]
impl Processor for Blur {
    async fn process(&self, image: &mut Image) -> Result<(), Error> {
        if self.radius == 0 { return Ok(()); }

        let s = if self.radius > MAX_BLUR_RADIUS {
            MAX_BLUR_RADIUS as f32 / 3.0
        } else {
            self.radius as f32 / 3.0
        };

        image.extend(image.blur(s));

        Ok(())
    }
}
