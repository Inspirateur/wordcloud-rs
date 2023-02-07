mod wordcloud;
mod builder;
mod hxbitmap;
mod util;
mod colors;
mod indexed_chars;
mod collision_map;
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
    use env_logger;

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
        env_logger::builder()
            .filter_module("wordcloud", log::LevelFilter::Info)
            .init();
        let text = fs::read_to_string("assets/sample_text.txt").unwrap();
        let mut tokens = tokenize(text);
        tokens.push((Token::from("assets/alan_turing.jpg"), 15.));
        tokens.push((Token::from("assets/turing_statue_bletchley.jpg"), 20.));
        tokens.push((Token::Text("💻".to_string()), 20.));
        let wc = WordCloud::new().generate(tokens);
        wc.save("sample_cloud.png").unwrap();
    }
}
