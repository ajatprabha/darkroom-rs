use async_trait::async_trait;
use image::GenericImageView;
use image::imageops::FilterType;
use crate::processor::{Image, Processor};
use crate::processor::error::Error;

pub struct Resize {
    pub width: u32,
    pub height: u32,
    pub maintain_aspect_ratio: bool,
}

impl Resize {
    fn resize_width_height(&self, image: &mut Image) -> (u32, u32) {
        let (actual_width, actual_height) = image.dimensions();

        if self.height == 0 {
            (self.width, (self.width * actual_height) / actual_width)
        } else if self.width == 0 {
            ((self.height * actual_width) / actual_height, self.height)
        } else {
            let h = (self.width * actual_height) / actual_width;
            if h <= self.height {
                return (self.width, h);
            }
            ((self.height * actual_width) / actual_height, self.height)
        }
    }

    fn resize(&self, image: &mut Image) -> Result<(), Error> {
        let (iw, ih) = image.dimensions();
        let (w, h) = self.resize_width_height(image);

        if w != iw || h != ih {
            image.extend(image.resize_exact(w, h, FilterType::Triangle));
        }

        Ok(())
    }
}

#[async_trait]
impl Processor for Resize {
    async fn process(&self, image: &mut Image) -> Result<(), Error> {
        if self.maintain_aspect_ratio {
            self.resize(image)
        } else {
            image.extend(image.resize(self.width, self.height, FilterType::Triangle));
            Ok(())
        }
    }
}
