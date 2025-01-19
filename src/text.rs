use fontdue::{Font, layout::{Layout, CoordinateSystem, TextStyle}, Metrics};
use image::{RgbaImage, GenericImage, Rgba, Pixel, GenericImageView};
use itertools::{enumerate, Itertools};
use crate::indexed_chars::IndexedChars;

use super::rasterisable::Rasterisable;
use std::{iter::zip, fmt::Display};
use binary_raster::BinaryRaster;

pub struct Text {
    text: String,
    layout: Layout,
    glyphs: Vec<(Metrics, Vec<u8>)>,
    color: Rgba<u8>,
}

impl Text {
    pub fn new(text: String, font: Font, size: f32, color: Rgba<u8>) -> Self {
        let fonts = [font];
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.append(&fonts, &TextStyle::new(text.as_str(), size, 0));
        let indexed = IndexedChars::new(&text);
        let glyphs: Vec<_> = indexed.chars.iter().map(|c| fonts[0].rasterize(*c, size)).collect();
        Self { text, layout, glyphs, color }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Rasterisable for Text {
    fn to_bitmap(&self) -> BinaryRaster {
        let mut total_width = 0;
        let mut total_height = 0;
        for (pos, (metrics, _)) in zip(self.layout.glyphs(), &self.glyphs) {
            total_width = total_width.max(pos.x as usize+metrics.width);
            total_height = total_height.max(pos.y as usize+metrics.height);
        }
        let mut bitmap = BinaryRaster::new(total_width, total_height);
        for (pos, (metrics, char_bitmap)) in zip(self.layout.glyphs(), &self.glyphs) {
            bitmap.add_from(&BinaryRaster::from_raster(char_bitmap, metrics.width), (pos.x as usize, pos.y as usize));
        }
        bitmap
    }

    fn draw(&self, image: &mut RgbaImage, pos: (usize, usize)) {
        for (dpos, (metrics, char_bitmap)) in zip(self.layout.glyphs(), &self.glyphs) {
            let x = pos.0 as u32 + dpos.x as u32;
            let y = pos.1 as u32 + dpos.y as u32;
            if x + metrics.width as u32 > image.width() {
                println!("{} + {} > {}", x, metrics.width, image.width());
            }
            let mut subimg = image.sub_image(x, y, metrics.width as u32, metrics.height as u32);
            for (i, value) in enumerate(char_bitmap) {
                let dx = (i % metrics.width) as u32;
                let dy = (i / metrics.width) as u32;
                let mut color = self.color.clone();
                color.0[3] = (color.0[3] as f32*(*value as f32)/255.) as u8;
                let mut pixel = subimg.get_pixel(dx, dy);
                pixel.blend(&color);
                subimg.put_pixel(dx, dy, pixel);
            }
        }
    }
}
