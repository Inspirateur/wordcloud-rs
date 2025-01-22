use std::fs;

use fontdue::{Font, FontSettings};
use image::RgbaImage;
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

fn _wordcloud(font: &Font, dim: (usize, usize), tokens: &Vec<(Token, f32)>, colors: &mut Box<dyn ColorGen>, size_factor: f32) -> Result<RgbaImage, RgbaImage> {
    let mut wc = WorldCloud::new(dim);
    for (token, size) in tokens {
        debug!(target: "wordcloud", "{} {}", size, token);
        let rasterisable: Box<dyn Rasterisable> = match token.clone() {
            Token::Text(text) => Box::new(Text::new(text, font.clone(), 2.+size*size_factor, colors.get())),
            Token::Img(image) => Box::new(Image::new(image, (2.+size*size_factor)*1.5))
        };
        if !wc.add(rasterisable) {
            return Err(wc.image);
        }
    }
    Ok(wc.image)
}

fn wordcloud(font: &Font, dim: (usize, usize), mut tokens: Vec<(Token, f32)>, colors: &mut Box<dyn ColorGen>) -> RgbaImage {
    #[cfg(feature = "fs")]
    convert_emojis(&mut tokens);
    let mut size_factor = size_factor(dim, &tokens); 
    let mut img_res = _wordcloud(font, dim, &tokens, colors, size_factor);
    debug!(target: "wordcloud", "Attempting to create {}x{} wordcloud with size factor of {}", dim.0, dim.1, size_factor);
    while img_res.is_err() && size_factor > 0.1 {
        size_factor *= 0.9;
        warn!(target: "wordcloud", "Couldn't fit every token, retrying with size factor of {}", size_factor);
        img_res = _wordcloud(font, dim, &tokens, colors, size_factor);
    }
    debug!(target: "wordcloud", "Wordcloud complete");
    match img_res {
        Ok(img) => img,
        Err(img) => img
    }
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

    pub fn width(&self) -> usize {
        self.dim.0
    }

    pub fn height(&self) -> usize {
        self.dim.1
    }
}