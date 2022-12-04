use super::hxbitmap::HXBitmap;
use image::RgbaImage;
use std::fmt::{Display};

pub trait Rasterisable: Display {
    fn to_bitmap(&self) -> HXBitmap;

    fn draw(&self, image: &mut RgbaImage, pos: (usize, usize)); 
}