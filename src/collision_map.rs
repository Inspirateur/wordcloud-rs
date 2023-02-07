use itertools::{iproduct, enumerate};
use log::warn;
use rand::thread_rng;
use rand::seq::SliceRandom;
use anyhow::{Result, anyhow, Ok};
use crate::{hxbitmap::HXBitmap, ring_reader::RingReader, rasterisable::Rasterisable};


pub struct CollisionMap {
    bitmap: HXBitmap,
    // efficiently spreads content around
    spots: RingReader<(usize, usize)>
}

impl CollisionMap {
    pub fn new(width: usize, height: usize) -> Self {
        let bitmap = HXBitmap::new(width, height);
        let mut spots = Vec::from_iter(iproduct!(
            0..bitmap.vec_w, 
            0..(bitmap.height-3.min(bitmap.height))
        ));
        spots.shuffle(&mut thread_rng());
        Self {
            bitmap, spots: RingReader::new(spots)
        }
    }
    
    fn bitmap_to_place(&self, token: &Box<dyn Rasterisable>) -> Result<HXBitmap> {
        let mut bitmap = token.to_bitmap();
        if bitmap.width > self.bitmap.width || bitmap.height > self.bitmap.height {
            return Err(anyhow!(
                "Can't fit {:?} into {:?}", 
                (bitmap.width, bitmap.height), 
                (self.bitmap.width, self.bitmap.height)
            ));
        }
        bitmap.blur();
        Ok(bitmap)
    }

    pub fn place(&mut self, token: &Box<dyn Rasterisable>) -> Result<(usize, usize)> {
        let bitmap = self.bitmap_to_place(token)?;
        if bitmap.width*bitmap.height == 0 {
            warn!(target: "wordcloud", "Token bitmap has area of 0");
            return Ok((0, 0));
        }
        let mut others: Vec<_> = enumerate(bitmap.h_offsets()).collect();
        others.shuffle(&mut thread_rng());
        while let Some((vec_x, vec_y)) = self.spots.next() {
            if vec_y+bitmap.height < self.bitmap.height {
                for (dx, other) in &others {
                    if vec_x*usize::BITS as usize+dx+bitmap.width >= self.bitmap.width {
                        break;
                    }
                    if !self.bitmap.overlaps(other, vec_x, vec_y) {
                        self.bitmap.add(other, vec_x, vec_y);
                        self.spots.reset();
                        return Ok((vec_x*usize::BITS as usize + dx, vec_y));
                    }
                }
            }
        }
        self.spots.reset();
        Err(anyhow!("Not enough room left to fit the object"))
    }
}