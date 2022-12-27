use std::fmt::Display;
use image::{open, imageops::FilterType, GenericImage, RgbaImage, DynamicImage};
use itertools::Itertools;
use std::path::Path;
use super::{rasterisable::Rasterisable, hxbitmap::HXBitmap};

pub struct Image {
    path: String,
    image: DynamicImage
}

impl Image {
    pub fn new(path: String, size: f32) -> Self {
        let image = open(&path).expect(&format!("Couldn't open image `{}`", path));
        let ratio = image.height() as f32/image.width() as f32;
        let image = image.resize(size as u32, (size*ratio) as u32, FilterType::Nearest);
        Self {path, image}
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Path::new(&self.path).file_name())
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
        let mut subimg = image.sub_image(
            pos.0 as u32, pos.1 as u32, self.image.width(), self.image.height()
        );
        subimg.copy_from(&self.image, 0, 0).unwrap();
    }
}