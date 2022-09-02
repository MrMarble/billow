//! Support for the [`image`][::image] crate.

use crate::Image;
use image::{DynamicImage, GenericImageView};

impl Image for DynamicImage {
    fn size(&self) -> (usize, usize) {
        (self.width() as usize, self.height() as usize)
    }
    fn get_pixel_at(&self, x: usize, y: usize) -> [u8; 4] {
        let pixel = self.get_pixel(x as u32, y as u32);
        [pixel[0], pixel[1], pixel[2], pixel[3]]
    }
}
