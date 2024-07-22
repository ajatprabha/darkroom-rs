use std::ops::{Deref, DerefMut};
use image::{DynamicImage, ImageFormat};

pub struct Image {
    inner: DynamicImage,
    pub format: Option<ImageFormat>,
}

impl Image {
    pub fn format(inner: DynamicImage, format: ImageFormat) -> Self { Self { inner, format: Some(format) } }

    pub fn new(inner: DynamicImage) -> Self { Self { inner, format: None } }

    pub fn extend(&mut self, image: DynamicImage) {
        self.inner = image;
    }
}

impl Deref for Image {
    type Target = DynamicImage;
    fn deref(&self) -> &Self::Target { &self.inner }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl From<DynamicImage> for Image {
    fn from(val: DynamicImage) -> Self { Self { inner: val, format: None } }
}
