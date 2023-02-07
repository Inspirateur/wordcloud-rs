use std::{fmt::Display, path::Path};
use super::{rasterisable::Rasterisable, collision_map::CollisionMap};
use image::{RgbaImage, DynamicImage, open};
use log::{info, warn};

#[derive(Clone)]
pub enum Token {
    Text(String),
    Img(DynamicImage)
}

impl Token {
    pub fn from<P>(path: P) -> Self
    where P: AsRef<Path> {
        Self::Img(open(&path).unwrap())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Text(text) => write!(f, "{}", text),
            Token::Img(_img) => write!(
                f, "Image"
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
        match self.collision_map.place(&token) {
            Ok(pos) => {
                token.draw(&mut self.image, pos);
                info!(target: "wordcloud", "Placed `{}` at {:?}", token, pos);
                true
            },
            Err(err) => {
                warn!(target: "wordcloud", "{:?}", err);
                false
            }
        }
    }
}