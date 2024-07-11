use crate::processor::{Image, Processor};
use crate::processor::error::Error;

pub struct Rotate {
    pub degrees: f32,
}

impl Rotate {
    fn calculate_bounding_box(&self, width: u32, height: u32) -> (u32, u32) {
        let angle_radians = self.degrees.to_radians();
        let cos_angle = angle_radians.cos().abs();
        let sin_angle = angle_radians.sin().abs();

        let new_width = (width as f32 * cos_angle + height as f32 * sin_angle).ceil() as u32;
        let new_height = (width as f32 * sin_angle + height as f32 * cos_angle).ceil() as u32;

        (new_width, new_height)
    }
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
