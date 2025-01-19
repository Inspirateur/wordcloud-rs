mod indexed_chars;
mod wordcloud;
mod builder;
mod util;
mod colors;
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
    #[cfg(feature = "fs")]
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
    #[cfg_attr(not(feature = "fs"), ignore)]
    #[cfg(feature = "fs")]
    fn it_works() {
        let text = fs::read_to_string("assets/sample_text.txt").unwrap();
        let mut tokens = tokenize(text);
        tokens.push((Token::from("assets/alan_turing.jpg"), 15.));
        tokens.push((Token::from("assets/turing_statue_bletchley.jpg"), 20.));
        tokens.push((Token::Text("💻".to_string()), 20.));
        let wc = WordCloud::new().generate(tokens);
        wc.save("sample_cloud.png").unwrap();
    }

    #[test]
    fn it_works_no_fs() {
        let text = r#"""
        Alan Mathison Turing (/ˈtjʊərɪŋ/; 23 June 1912 – 7 June 1954) was an English mathematician, computer scientist, logician, cryptanalyst, philosopher, and theoretical biologist.
        Turing was highly influential in the development of theoretical computer science, providing a formalisation of the concepts of algorithm and computation with the Turing machine, which can be considered a model of a general-purpose computer.
        He is widely considered to be the father of theoretical computer science and artificial intelligence.
        """#.to_string();
        let mut tokens = tokenize(text);
        tokens.push((Token::Text("💻".to_string()), 5.));
        let wc = WordCloud::new().generate(tokens);
        wc.save("sample_cloud_no_fs.png").unwrap();
    }
}
