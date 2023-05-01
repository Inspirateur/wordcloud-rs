use palette::{rgb::{Rgb, Rgba}, Lcha, FromColor, ShiftHue};
use image::Rgba as IRgba;
use rand::{distributions::{Uniform, }, seq::SliceRandom, rngs::ThreadRng, prelude::Distribution};
use rand_distr::Normal;

pub trait ColorGen<C = IRgba<u8>> {
    fn get(&mut self) -> C;
}

pub fn palette2image(color: Rgba) -> IRgba<u8> {
    IRgba([
        (color.red*255.) as u8,
        (color.green*255.) as u8,
        (color.blue*255.) as u8,
        (color.alpha*255.) as u8,
    ])
}

pub enum ColorScheme {
    Rainbow {luminance: f32, chroma: f32},
    BiaisedRainbow {anchor: Rgb, variance: f32},
    DoubleSplitCompl {anchor: Rgb},
}

pub struct ClustUniformColors {
    anchor: Lcha,
    hues: Vec<i32>,
    h_noise: Uniform<f32>,
    rng: ThreadRng,
}

impl ColorGen for ClustUniformColors {
    fn get(&mut self) -> IRgba<u8> {
        let mut hue = *self.hues.choose(&mut self.rng).unwrap_or(&0) as f32;
        hue += self.h_noise.sample(&mut self.rng);
        let new_color = self.anchor.shift_hue(hue);
        palette2image(Rgba::from_color(new_color))
    }
}

pub struct GaussianColors {
    anchor: Lcha,
    normal: Normal<f32>,
    rng: ThreadRng,
}

impl ColorGen for GaussianColors {
    fn get(&mut self) -> IRgba<u8> {
        let hue = self.normal.sample(&mut self.rng);
        let new_color = self.anchor.shift_hue(hue);
        palette2image(Rgba::from_color(new_color))
    }
}

impl From<ColorScheme> for Box<dyn ColorGen> {
    fn from(cs: ColorScheme) -> Self {
        match cs {
            ColorScheme::Rainbow {luminance, chroma} => Box::new(ClustUniformColors {
                anchor: Lcha::new(luminance, chroma, 0., 1.),
                hues: vec![0],
                h_noise: Uniform::from(-180.0..180.),
                rng: rand::thread_rng()
            }),
            ColorScheme::BiaisedRainbow {anchor, variance} => {
                let mut anchor = Lcha::from_color(anchor);
                anchor.chroma = anchor.chroma.max(30.);
                Box::new(GaussianColors {
                    anchor,
                    normal: Normal::new(0., variance).unwrap(),
                    rng: rand::thread_rng()
                })
            }
            ColorScheme::DoubleSplitCompl {anchor} => {
                let mut anchor = Lcha::from_color(anchor);
                anchor.chroma = anchor.chroma.max(30.);
                Box::new(ClustUniformColors {
                    anchor,
                    hues: vec![-15, 0, 15, 180-15, 180+15],
                    h_noise: Uniform::from(-2.0..2.),
                    rng: rand::thread_rng()
                })
            }
        }
    }
}