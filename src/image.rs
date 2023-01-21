use std::fmt::Display;
use image::{imageops::FilterType, RgbaImage, DynamicImage, Pixel, GenericImageView};
use itertools::Itertools;
use super::{rasterisable::Rasterisable, hxbitmap::HXBitmap};

pub struct Image {
    image: DynamicImage
}

impl Image {
    pub fn new(image: DynamicImage, size: f32) -> Self {
        let ratio = image.height() as f32/image.width() as f32;
        let image = image.resize(size as u32, (size*ratio) as u32, FilterType::Nearest);
        Self {image}
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} image", self.image.width(), self.image.height())
    }
}

impl Rasterisable for Image {
    fn to_bitmap(&self) -> HXBitmap {
        let mut bitmap = HXBitmap::new(self.image.width() as usize, self.image.height() as usize);
        let values = if self.image.color().has_alpha() {
            self.image.as_bytes().iter().skip(3).step_by(4).cloned().collect_vec()
        } else {
            vec![255; (self.image.width()*self.image.height()) as usize]
        };
        bitmap.add_bitmap(self.image.width() as usize, &values, 0, 0);
        bitmap
    }

    fn draw(&self, image: &mut RgbaImage, pos: (usize, usize)) {
        let px = pos.0 as u32;
        let py = pos.1 as u32;
        let max_x = (px+self.image.width()).min(image.width())-px;
        let max_y = (py+self.image.height()).min(image.height())-py;
        for x in 0..max_x {
            for y in 0..max_y {
                image.get_pixel_mut(x+px, y+py).blend(&self.image.get_pixel(x, y));
            }
        }
    }
}