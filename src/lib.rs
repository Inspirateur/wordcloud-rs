mod wordcloud;
mod builder;
mod hxbitmap;
mod util;
mod colors;
mod indexed_chars;
mod text;
mod image;
mod rasterisable;
mod ring_reader;
pub use wordcloud::Token;
pub use builder::Builder as WordCloud;
pub use colors::ColorScheme as Colors;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use super::*;
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref RE_TOKEN: Regex = Regex::new(r"\w+").unwrap();
    }

    fn tokenize(text: String) -> Vec<(Token, f32)> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for token in RE_TOKEN.find_iter(&text) {
            *counts.entry(token.as_str().to_string()).or_default() += 1;
        }
        counts.into_iter().map(|(k, v)| (Token::Text(k), v as f32)).collect()
    }

    #[test]
    fn it_works() {
        let text = fs::read_to_string("assets/sample_text.txt").unwrap();
        let mut tokens = tokenize(text);
        tokens.push((Token::Img("assets/alan_turing.jpg".to_string()), 60.));
        tokens.push((Token::Img("assets/turing_statue_bletchley.jpg".to_string()), 80.));
        tokens.push((Token::Img("assets/computer_emoji.png".to_string()), 40.));
        let wc = WordCloud::new().generate(tokens);
        wc.save("sample_cloud.png").unwrap();
    }
}
