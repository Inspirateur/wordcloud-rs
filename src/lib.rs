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

    fn tokenize(text: String) -> Vec<(Token, f32)> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for token in text.replace(&['.', ',', '"', '\''], "").split_whitespace() {
            *counts.entry(token.to_string()).or_default() += 1;
        }
        counts.into_iter().map(|(k, v)| (Token::Text(k), v as f32)).collect()
    }

    #[test]
    fn it_works() {
        let text = fs::read_to_string("assets/sample_text.txt").unwrap();
        let wc = WordCloud::new().generate(tokenize(text));
        wc.save("sample_cloud.png").unwrap();
    }
}
