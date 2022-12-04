use fontdue::{Font, layout::{Layout, CoordinateSystem, TextStyle}, Metrics};
use image::{RgbaImage, GenericImage, Rgba};
use itertools::enumerate;
use super::{hxbitmap::{HXBitmap}, rasterisable::Rasterisable, indexed_chars::IndexedChars};
use std::{iter::zip, fmt::Display};

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

fn compute_dim(layout: &Layout) -> (usize, usize) {
    let (mut x1, mut y1, mut x2, mut y2): (i32, i32, i32, i32) = (0, 0, 0, 0);
    for pos in layout.glyphs() {
        x1 = x1.min(pos.x as i32);
        y1 = y1.min(pos.y as i32);
        x2 = x2.max(pos.x as i32+pos.width as i32);
        y2 = y2.max(pos.y as i32+pos.height as i32);
    }
    return (1+(x2-x1) as usize, (y2-y1) as usize)
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Rasterisable for Text {
    fn to_bitmap(&self) -> HXBitmap {
        let dim = compute_dim(&self.layout);
        let mut bitmap = HXBitmap::new(dim.0, dim.1);
        for (pos, (metrics, char_bitmap)) in zip(self.layout.glyphs(), &self.glyphs) {
            bitmap.add_bitmap(metrics.width, char_bitmap, pos.x as usize, pos.y as usize);
        }
        bitmap
    }

    fn draw(&self, image: &mut RgbaImage, pos: (usize, usize)) {
        for (dpos, (metrics, char_bitmap)) in zip(self.layout.glyphs(), &self.glyphs) {
            let x = pos.0 as u32 + dpos.x as u32;
            let y = pos.1 as u32 + dpos.y as u32;
            let mut subimg = image.sub_image(x, y, metrics.width as u32, metrics.height as u32);
            for (i, value) in enumerate(char_bitmap) {
                let dx = i % metrics.width;
                let dy = i / metrics.width;
                let mut color = self.color.clone();
                color.0[3] = (color.0[3] as f32*(*value as f32)/255.) as u8;
                subimg.put_pixel(dx as u32, dy as u32, color);
            }
        }
    }
}
