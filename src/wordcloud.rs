use super::{hxbitmap::HXBitmap, rasterisable::Rasterisable};
use image::RgbaImage;
use log::{info, warn};
pub enum Token {
    Text(String),
    Img(String)
}

pub struct WorldCloud {
    bitmap: HXBitmap,
    pub image: RgbaImage,
}

impl WorldCloud {
    pub fn new(dim: (usize, usize)) -> Self {
        let bitmap = HXBitmap::new(dim.0, dim.1);
        let image = RgbaImage::new(dim.0 as u32, dim.1 as u32);
        Self { bitmap, image }
    }

    pub fn add(&mut self, token: Box<dyn Rasterisable>) -> bool {
        let bitmap = token.to_bitmap();
        if bitmap.width*bitmap.height == 0 {
            return false;
        }
        match self.bitmap.place(bitmap) {
            Ok(pos) => {
                token.draw(&mut self.image, pos);
                info!("Placed `{}` at {:?}", token, pos);
                true
            },
            Err(err) => {
                warn!("{:?}", err);
                false
            }
        }
    }
}