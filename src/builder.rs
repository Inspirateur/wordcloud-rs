use std::fs;

use fontdue::{Font, FontSettings};
use image::RgbaImage;
use itertools::enumerate;
use log::{error, debug, warn};
use super::{
    colors::{ColorScheme, ColorGen}, Token, wordcloud::WorldCloud, 
    rasterisable::Rasterisable, text::Text, image::Image
};
use twemoji_rs::get_twemoji;

fn convert_emojis(tokens: &mut Vec<(Token, f32)>) {
    // Convert unicode emojis to images with Twemoji
    tokens.iter_mut().for_each(|(token, _v)| {
        if let Token::Text(str) = token {
            if let Some(path) = get_twemoji(str) {
                *token = Token::from(&path);
            }
        }
    });
}

fn size_factor(dim: (usize, usize), tokens: &Vec<(Token, f32)>) -> f32 {
    let sum = tokens.iter().fold(0., |i, (_, s)| i+s);
    // magical formula that seems to work well ¯\_(ツ)_/¯
    2.*(tokens.len() as f32).log(10.)*dim.0 as f32/sum
}

fn wordcloud(font: &Font, dim: (usize, usize), mut tokens: Vec<(Token, f32)>, colors: &mut Box<dyn ColorGen>) -> RgbaImage {
    tokens.sort_by(|(_, s1), (_, s2)| s2.partial_cmp(s1).unwrap());
    tokens.truncate(100);
    tokens.iter_mut().for_each(|(_, v)| *v = v.sqrt());
    #[cfg(feature = "fs")]
    convert_emojis(&mut tokens);
    let c = size_factor(dim, &tokens); 
    let mut wc = WorldCloud::new(dim);
    // shrink tokens if they don't fit, up to a point
    let len = tokens.len();
    'outer: for (i, (token, size)) in enumerate(tokens) {
        let mut adjust = 1.;
        debug!(target: "wordcloud", "{} {}", size, token);
        loop {
            let rasterisable: Box<dyn Rasterisable> = match token.clone() {
                Token::Text(text) => Box::new(Text::new(text, font.clone(), (2.+size*c)*adjust, colors.get())),
                Token::Img(image) => Box::new(Image::new(image, (2.+size*c)*adjust*1.5))
            };
            if wc.add(rasterisable) {
                break;
            }
            if adjust < 0.5 {
                warn!(target: "wordcloud", "Could only fit {}/{} tokens", i, len);
                break 'outer;
            }
            adjust -= 0.1;
            warn!(target: "wordcloud", "Adjusting scale to {}", adjust)
        };
    }
    wc.image
}

pub struct Builder {
    dim: (usize, usize),
    font: Font,
    colors: Box<dyn ColorGen>,
}

impl Builder {
    pub fn new() -> Self {
        let font = include_bytes!("../assets/whitneymedium.otf") as &[u8];
        // Parse it into the font type.
        let font = Font::from_bytes(font, FontSettings::default()).unwrap();
        Self {
            dim: (896, 448),
            font, 
            colors: ColorScheme::Rainbow {luminance: 70., chroma: 100.}.into()
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

    /// witdh and height both need to be multiples of usize::BITS (64 on most machine) !
    pub fn dim(mut self, width: usize, height: usize) -> Self {
        self.dim = (width, height);
        self
    }

    pub fn colors(mut self, colors: impl Into<Box<dyn ColorGen>>) -> Self {
        self.colors = colors.into();
        self
    }

    pub fn generate(&mut self, tokens: Vec<(Token, f32)>) -> RgbaImage {
        wordcloud(&self.font, self.dim, tokens, &mut self.colors)
    }
}