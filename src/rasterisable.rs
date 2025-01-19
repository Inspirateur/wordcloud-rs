use image::RgbaImage;
use std::fmt::Display;
use binary_raster::BinaryRaster;

pub trait Rasterisable: Display {
    fn to_bitmap(&self) -> BinaryRaster;

    fn draw(&self, image: &mut RgbaImage, pos: (usize, usize)); 
}