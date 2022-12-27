use std::fmt::Display;
use std::path::Path;
use super::{rasterisable::Rasterisable, collision_map::CollisionMap};
use image::RgbaImage;
use log::{info, warn};

#[derive(Clone)]
pub enum Token {
    Text(String),
    Img(String)
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Text(text) => write!(f, "{}", text),
            Token::Img(path) => write!(
                f, "{:?}", Path::new(path).file_name().unwrap()
            ),
        }
        
    }
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
        let mut bitmap = token.to_bitmap();
        if bitmap.width*bitmap.height == 0 {
            warn!(target: "Word Cloud", "Token bitmap has area of 0");
            return false;
        }
        bitmap.blur();
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