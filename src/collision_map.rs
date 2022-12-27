use itertools::{iproduct, enumerate};
use rand::thread_rng;
use rand::seq::SliceRandom;
use anyhow::{Result, anyhow};
use crate::{hxbitmap::HXBitmap, ring_reader::RingReader};


pub struct CollisionMap {
    width: usize,
    height: usize,
    bitmap: HXBitmap,
    // efficiently spreads content around
    poses: RingReader<(usize, usize)>
}

impl CollisionMap {
    pub fn new(width: usize, height: usize) -> Self {
        let bitmap = HXBitmap::new(width, height);
        let mut poses = Vec::from_iter(iproduct!(0..bitmap.vec_w, 0..(height-3.min(height))));
        poses.shuffle(&mut thread_rng());
        Self {
            width, height, bitmap,
            poses: RingReader::new(poses)
        }
    }
    
    pub fn place(&mut self, other: HXBitmap) -> Result<(usize, usize)> {
        if other.width > self.width || other.height > self.height {
            return Err(anyhow!(
                "Can't fit {:?} into {:?}", (other.width, other.height), (self.width, self.height)
            ));
        }
        let mut others: Vec<_> = enumerate(other.h_offsets()).collect();
        others.shuffle(&mut thread_rng());
        while let Some((vec_x, y)) = self.poses.next() {
            if y+other.height < self.height {
                for (dx, other) in &others {
                    if vec_x*usize::BITS as usize+dx+other.width >= self.width {
                        break;
                    }
                    if !self.bitmap.overlaps(other, vec_x, y) {
                        self.bitmap.add(other, vec_x, y);
                        self.poses.reset();
                        return Ok((vec_x*usize::BITS as usize + dx, y));
                    }
                }
            }
        }
        self.poses.reset();
        return Err(anyhow!("Not enough room left to fit the object"));
    }
}