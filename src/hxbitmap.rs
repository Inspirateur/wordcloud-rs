use std::{ops::{Shl, Shr}, fmt::Display, vec};
use itertools::{enumerate};
use super::util::{bits, next_multiple};
pub const CHARS: [&str; 2] = [" ", "█"];
// Horizontally Accelerated Bitmap
#[derive(Clone)]
pub struct HXBitmap {
    data: Vec<usize>,
    // allocated width/usize::BITS
    pub(crate) vec_w: usize,
    // official width
    pub width: usize,
    // height
    pub height: usize,
}

impl HXBitmap {
    pub fn new(width: usize, height: usize) -> Self {
        let _w = next_multiple(width, usize::BITS as usize);
        let vec_w = _w/usize::BITS as usize;
        Self {
            data: vec![0; vec_w*height], vec_w, width, height, 
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

    pub(crate) fn blur(&mut self) {
        // used to extend the "hitbox" of bitmaps a bit so that they are 
        // not right next to each other in the collision map
        // horizontal "blur"
        self.data.iter_mut().for_each(|v| *v |= *v << 1 | *v >> 1);
        // vertical "blur"
        self.height += 2;
        let w = self.vec_w;
        let mut new_data = vec![0; w*self.height];
        new_data[..w].copy_from_slice(&self.data[..w]);
        new_data[w..(self.data.len()+w)].copy_from_slice(&self.data);
        new_data[(self.data.len()+w)..].copy_from_slice(&self.data[(self.data.len()-w)..]);
        self.data = new_data;
    }

    pub(crate) fn h_offsets(&self) -> Vec<Self> {
        // only do the offsets if it's 1 usize long cause it's hellish otherwise
        if self.vec_w > 1 {
            vec![self.clone()]
        } else {
            let max = *self.data.iter().max().unwrap();
            (0..=max.leading_zeros()).into_iter().map(|hshift| self.clone() << hshift).collect()
        }
    }

    fn set(&mut self, value: usize, vec_x: usize, y: usize) {
        let i = self.idx1d(vec_x, y);
        self.data[i] |= value;
    }

    pub(crate) fn overlaps(&self, other: &Self, vec_x: usize, y: usize) -> bool {
        for (v, (dx, dy)) in other.iter2d() {
            let i = self.idx1d(vec_x+dx, y+dy);
            if self.data[i] & v != 0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn add(&mut self, other: &Self, vec_x: usize, y: usize) {
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
}

impl Shl<u32> for HXBitmap {
    type Output = HXBitmap;

    fn shl(self, rhs: u32) -> Self::Output {
        // A naive shift that only works when width = 1
        Self {
            data: self.data.into_iter().map(|v| v << rhs).collect(),
            vec_w: self.vec_w,
            width: self.width,
            height: self.height,
        }
    }
}

impl Shr<u32> for HXBitmap {
    type Output = HXBitmap;

    fn shr(self, rhs: u32) -> Self::Output {
        // A naive shift that only works when width = 1
        Self {
            data: self.data.into_iter().map(|v| v >> rhs).collect(),
            vec_w: self.vec_w,
            width: self.width,
            height: self.height,
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
