use std::{fs};
use fontdue::{Font, FontSettings};
use image::RgbaImage;
use itertools::enumerate;
use log::{error, debug, warn};
use super::{colors::{Colors, ColorScheme, ColorGen}, Token, wordcloud::WorldCloud, rasterisable::Rasterisable, text::Text, image::Image};

fn size_factor(dim: (usize, usize), tokens: &Vec<(Token, f32)>) -> f32 {
    let sum = tokens.iter().fold(0., |i, (_, s)| i+s);
    // magical formula that seems to work well ¯\_(ツ)_/¯
    1.5*(tokens.len() as f32).log(10.)*dim.0 as f32/sum
}

fn wordcloud(font: &Font, dim: (usize, usize), mut tokens: Vec<(Token, f32)>, colors: &mut Colors) -> RgbaImage {
    tokens.sort_by(|(_, s1), (_, s2)| s2.partial_cmp(s1).unwrap());
    tokens.truncate(100);
    tokens.iter_mut().for_each(|(_, v)| *v = v.sqrt());
    let c = size_factor(dim, &tokens); 
    let mut wc = WorldCloud::new(dim);
    // shrink tokens if they don't fit, up to a point
    let mut adjust = 1.;
    let len = tokens.len();
    'outer: for (i, (token, size)) in enumerate(tokens) {
        debug!(target: "Word Cloud", "{} {}", size, token);
        loop {
            let rasterisable: Box<dyn Rasterisable> = match token.clone() {
                Token::Text(text) => Box::new(Text::new(text, font.clone(), (2.+size*c)*adjust, colors.get())),
                Token::Img(path) => Box::new(Image::new(path, (2.+size*c)*adjust))
            };
            if wc.add(rasterisable) {
                break;
            }
            if adjust < 0.5 {
                warn!(target: "Word Cloud", "Could only fit {}/{} tokens", i, len);
                break 'outer;
            }
            adjust -= 0.1;
            warn!(target: "Word Cloud", "Adjusting scale to {}", adjust)
        };
    }
    wc.image
}

pub struct Builder {
    dim: (usize, usize),
    font: Font,
    colors: Colors,
}

impl Builder {
    pub fn new() -> Self {
        let font = include_bytes!("../assets/whitneymedium.otf") as &[u8];
        // Parse it into the font type.
        let font = Font::from_bytes(font, FontSettings::default()).unwrap();
        Self {
            dim: (800, 400),
            font, 
            colors: ColorScheme::Rainbow {luminance: 90., chroma: 128.}.into()
        }
    }

    pub fn font(mut self, path: &str) -> Self {
        match fs::read(path) {
            Ok(bytes) => match Font::from_bytes(bytes, FontSettings::default()) {
                Ok(font) => self.font = font,
                Err(err) => error!("{}", err)
            },
            Err(err) => error!("{}", err)
        }
        self
    }

    pub fn dim(mut self, width: usize, height: usize) -> Self {
        self.dim = (width, height);
        self
    }

    pub fn colors(mut self, colors: impl Into<Colors>) -> Self {
        self.colors = colors.into();
        self
    }

    pub fn generate(&mut self, tokens: Vec<(Token, f32)>) -> RgbaImage {
        wordcloud(&self.font, self.dim, tokens, &mut self.colors)
    }
}