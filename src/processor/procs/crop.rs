use image::GenericImageView;
use image::imageops::FilterType;
use crate::processor::error::Error;
use crate::processor::image::Image;
use crate::processor::Processor;
use crate::handler::query::Crop as QueryCrop;
use crate::processor::procs::resize::Resize;

pub enum CropPoint {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
    Custom(u32, u32),
}

pub struct Crop {
    pub point: CropPoint,
    pub width: u32,
    pub height: u32,
}

impl Crop {
    fn resize_width_height_for_crop(&self, image: &mut Image) -> (u32, u32) {
        let (actual_width, actual_height) = image.dimensions();
        let h = (self.width * actual_height) / actual_width;
        if h > self.height {
            (self.width, h)
        } else {
            let w = (self.height * actual_width) / actual_height;
            (w, self.height)
        }
    }

    fn start_point_for_crop(&self, w: u32, h: u32) -> (u32, u32) {
        if let CropPoint::Custom(x, y) = self.point {
            return (x, y);
        }

        let (x, y) = ((w - self.width) / 2, (h - self.height) / 2);

        match self.point {
            CropPoint::TopLeft => (0, 0),
            CropPoint::Top => (x, 0),
            CropPoint::TopRight => (w - self.width, 0),
            CropPoint::Left => (0, y),
            CropPoint::Center => (x, y),
            CropPoint::Right => (w - self.width, y),
            CropPoint::BottomLeft => (0, h - self.height),
            CropPoint::Bottom => (x, h - self.height),
            CropPoint::BottomRight => (w - self.width, h - self.height),
            CropPoint::Custom(x, y) => (x, y),
        }
    }
}

impl Processor for Crop {
    fn process(&self, image: &mut Image) -> Result<(), Error> {
        if self.width == 0 || self.height == 0 {
            if self.width == 0 && self.height == 0 {
                return Ok(());
            }

            return Resize {
                width: self.width,
                height: self.height,
                maintain_aspect_ratio: true,
            }.process(image);
        }

        let (w, h) = self.resize_width_height_for_crop(image);

        *image = image.resize_exact(w, h, FilterType::Triangle).into();

        let (x, y) = self.start_point_for_crop(w, h);

        *image = image.crop(x, y, self.width, self.height).into();

        Ok(())
    }
}

impl From<Option<QueryCrop>> for CropPoint {
    fn from(val: Option<QueryCrop>) -> Self {
        match val {
            Some(QueryCrop::TopLeft) => CropPoint::TopLeft,
            Some(QueryCrop::Top) => CropPoint::Top,
            Some(QueryCrop::TopRight) => CropPoint::TopRight,
            Some(QueryCrop::Left) => CropPoint::Left,
            Some(QueryCrop::Center) => CropPoint::Center,
            Some(QueryCrop::Right) => CropPoint::Right,
            Some(QueryCrop::BottomLeft) => CropPoint::BottomLeft,
            Some(QueryCrop::Bottom) => CropPoint::Bottom,
            Some(QueryCrop::BottomRight) => CropPoint::BottomRight,
            None => CropPoint::Center,
        }
    }
}
