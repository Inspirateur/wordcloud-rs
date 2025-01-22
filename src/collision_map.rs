use itertools::iproduct;
use rand::thread_rng;
use rand::seq::SliceRandom;
use anyhow::{Result, anyhow, Ok};
use crate::{ring_reader::RingReader, rasterisable::Rasterisable};
use binary_raster::BinaryRaster;

pub struct CollisionMap {
    bitmap: BinaryRaster,
    // efficiently spreads content around
    spots: RingReader<(usize, usize)>
}

impl CollisionMap {
    pub fn new(width: usize, height: usize) -> Self {
        let bitmap = BinaryRaster::new(width, height);
        let mut spots = Vec::from_iter(iproduct!(
            (2..(width-40)).step_by(8), 
            (2..(height-20)).step_by(8)
        ));
        spots.shuffle(&mut thread_rng());
        Self {
            bitmap, spots: RingReader::new(spots)
        }
    }

    pub fn place(&mut self, token: &Box<dyn Rasterisable>) -> Result<(usize, usize)> {
        let bitmap = token.to_bitmap();
        while let Some((x, y)) = self.spots.next() {
            if !self.bitmap.can_fit(&bitmap, (x, y)) {
                continue;
            }
            if self.bitmap.add_from_checked(&bitmap, (x, y)).is_ok() {
                self.spots.reset();
                return Ok((x, y));
            }
        }
        self.spots.reset();
        Err(anyhow!("Not enough room left to fit the object"))
    }

    pub fn get_display(&self, resolution: u32) -> String {
        self.bitmap.get_display(resolution)
    }
}