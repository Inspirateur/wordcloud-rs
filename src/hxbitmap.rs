use std::{ops::{Shl, Shr}, fmt::Display, vec};
use anyhow::{Result, anyhow};
use itertools::{iproduct, enumerate};
use super::{util::{bits, next_multiple}, ring_reader::RingReader};
use rand::thread_rng;
use rand::seq::SliceRandom;
pub const CHARS: [&str; 2] = [" ", "█"];
// Horizontally Accelerated Bitmap
#[derive(Clone)]
pub struct HXBitmap {
    // official width
    pub width: usize,
    // height
    pub height: usize,
    // allocated width
    _w: usize,
    // _w/usize::BITS
    vec_w: usize,
    data: Vec<usize>,
    // efficiently spreads content around
    poses: RingReader<(usize, usize)>
}

impl HXBitmap {
    pub fn new(width: usize, height: usize) -> Self {
        let _w = next_multiple(width, usize::BITS as usize);
        let vec_w = _w/usize::BITS as usize;
        let mut poses = Vec::from_iter(iproduct!(0..vec_w, 0..(height-3.min(height))));
        poses.shuffle(&mut thread_rng());
        Self {
            width, height, _w, vec_w, data: vec![0; vec_w*height],
            poses: RingReader::new(poses)
        }
    }

    fn idx2d(&self, i: usize) -> (usize, usize) {
        (i.rem_euclid(self.vec_w), i/self.vec_w)
    }

    fn idx1d(&self, vec_x: usize, y: usize) -> usize {
        vec_x+y*self.vec_w
    }

    fn iter2d(&self) -> impl Iterator<Item=(&usize, (usize, usize))> {
        enumerate(&self.data).map(|(i, v)| (v, self.idx2d(i)))
    }

    fn h_offsets(&self) -> Vec<Self> {
        // only do the offsets if it's 1 usize long cause it's hellish otherwise
        if self.vec_w > 1 {
            vec![self.clone()]
        } else {
            let max = *self.data.iter().max().unwrap();
            (0..max.leading_zeros()).into_iter().map(|hshift| self.clone() << hshift).collect()
        }
    }

    fn overlaps(&self, other: &Self, vec_x: usize, y: usize) -> bool {
        for (v, (dx, dy)) in other.iter2d() {
            let i = self.idx1d(vec_x+dx, y+dy);
            if self.data[i] & v != 0 {
                return true;
            }
        }
        false
    }

    fn set(&mut self, value: usize, vec_x: usize, y: usize) {
        let i = self.idx1d(vec_x, y);
        self.data[i] |= value;
    }

    pub fn add(&mut self, other: &Self, vec_x: usize, y: usize) {
        for (v, (dx, dy)) in other.iter2d() {
            self.set(*v, vec_x+dx, y+dy);
        }
    }

    pub fn add_bitmap(&mut self, other_width: usize, other_data: &Vec<u8>, x: usize, y: usize) {
        let mut buff: usize = 0;
        let _dx = x.rem_euclid(usize::BITS as usize);
        let _x = x/usize::BITS as usize;
        let mut dx: usize = _dx;
        let mut vec_x = _x;
        let mut vec_y = y;
        for (i, val) in enumerate(other_data) {
            if i != 0 && i % other_width == 0 {
                self.set(buff, vec_x, vec_y);
                buff = 0;
                dx = _dx;
                vec_x = _x;
                vec_y += 1;
            }
            buff += ((*val).min(1) as usize) << dx;
            dx += 1;
            if dx as u32 == usize::BITS {
                self.set(buff, vec_x, vec_y);
                buff = 0;
                dx = 0;
                vec_x += 1;
            }
        }
    }

    pub fn place(&mut self, other: Self) -> Result<(usize, usize)> {
        if other.width > self.width || other.height > self.height {
            return Err(anyhow!(
                "Bitmap: Can't fit {:?} into {:?}", (other.width, other.height), (self.width, self.height)
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
                    if !self.overlaps(other, vec_x, y) {
                        self.add(other, vec_x, y);
                        self.poses.reset();
                        return Ok((vec_x*usize::BITS as usize + dx, y));
                    }
                }
            }
        }
        return Err(anyhow!("Bitmap: Not enough room left to fit the object"));
    }
}

impl Shl<u32> for HXBitmap {
    type Output = HXBitmap;

    fn shl(self, rhs: u32) -> Self::Output {
        // A naive shift that only works when width = 1
        Self {
            width: self.width,
            height: self.height,
            _w: self._w,
            vec_w: self.vec_w,
            data: self.data.into_iter().map(|v| v << rhs).collect(),
            poses: self.poses.clone()
        }
    }
}

impl Shr<u32> for HXBitmap {
    type Output = HXBitmap;

    fn shr(self, rhs: u32) -> Self::Output {
        // A naive shift that only works when width = 1
        Self {
            width: self.width,
            height: self.height,
            _w: self._w,
            vec_w: self.vec_w,
            data: self.data.into_iter().map(|v| v >> rhs).collect(),
            poses: self.poses.clone()
        }
    }
}

impl Display for HXBitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = "|".to_string();
        for (i, v) in enumerate(&self.data) {
            if i != 0 && i % self.vec_w == 0 {
                res.push_str("\n|");
            }
            for b in bits(*v) {
                res.push_str(CHARS[b]);
            }
        }
        write!(f, "{}\n", res)
    }
}
