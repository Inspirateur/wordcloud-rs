use super::{rasterisable::Rasterisable, collision_map::CollisionMap};
use image::RgbaImage;
use log::{info, warn};
pub enum Token {
    Text(String),
    Img(String)
}

pub struct WorldCloud {
    collision_map: CollisionMap,
    pub image: RgbaImage,
}

impl WorldCloud {
    pub fn new(dim: (usize, usize)) -> Self {
        let collision_map = CollisionMap::new(dim.0, dim.1);
        let image = RgbaImage::new(dim.0 as u32, dim.1 as u32);
        Self { collision_map, image }
    }

    pub fn add(&mut self, token: Box<dyn Rasterisable>) -> bool {
        let bitmap = token.to_bitmap();
        if bitmap.width*bitmap.height == 0 {
            return false;
        }
        match self.collision_map.place(bitmap) {
            Ok(pos) => {
                token.draw(&mut self.image, pos);
                info!(target: "Word Cloud", "Placed `{}` at {:?}", token, pos);
                true
            },
            Err(err) => {
                warn!(target: "Word Cloud", "{:?}", err);
                false
            }
        }
    }
}