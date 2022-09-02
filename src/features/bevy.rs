//! Support for the [`bevy`][::prelude][::Image] crate.

use crate::Image;

impl Image for bevy::prelude::Image {
    fn size(&self) -> (usize, usize) {
        (self.size().x as usize, self.size().y as usize)
    }
    fn get_pixel_at(&self, x: usize, y: usize) -> [u8; 4] {
        let width = self.size().x as usize;
        let idx = (x + y * width) * 4;

        self.data[idx..=idx + 3].try_into().unwrap()
    }
}
