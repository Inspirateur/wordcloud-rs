# wordcloud-rs
A Rust library to generate word-clouds from text and images!

## Example 
### Code
```rust
use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use wordcloud_rs::*;

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

fn main() {
    // Prepare the tokens
    let text = fs::read_to_string("assets/sample_text.txt").unwrap();
    let mut tokens = tokenize(text);
    tokens.push((Token::from("assets/alan_turing.jpg"), 15.));
    tokens.push((Token::from("assets/turing_statue_bletchley.jpg"), 20.));
    tokens.push((Token::Text("💻".to_string()), 20.));
    // Generate the word-cloud
    let wc = WordCloud::new().generate(tokens);
    // Save it
    wc.save("sample_cloud.png").unwrap();
}
```
### Output
![word_cloud_demo](https://github.com/Inspirateur/wordcloud-rs/raw/main/sample_cloud.png)
