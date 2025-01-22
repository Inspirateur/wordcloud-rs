use std::{collections::HashMap, fs};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordcloud_rs::{Token, WordCloud};
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
fn bench_wordcloud(c: &mut Criterion) {
    let text = fs::read_to_string("assets/sample_text.txt").unwrap();
    let mut tokens = tokenize(text);
    tokens.push((Token::from("assets/alan_turing.jpg"), 15.));
    tokens.push((Token::from("assets/turing_statue_bletchley.jpg"), 20.));
    tokens.push((Token::Text("💻".to_string()), 20.));
    // sort the token by importance
    tokens.sort_by(|(_, s1), (_, s2)| s2.partial_cmp(s1).unwrap());
    // only keep the top 100
    tokens.truncate(100);
    // "squish" importance values
    tokens.iter_mut().for_each(|(_, v)| *v = v.sqrt());    
    let wc = WordCloud::new();
    c.bench_function(&format!("wordcloud {}x{}", wc.width(), wc.height()), |b| b.iter(|| {
        WordCloud::new().generate(black_box(tokens.clone()));
    }));
}

criterion_group!(
    wordcloud, 
    bench_wordcloud, 
);
criterion_main!(wordcloud);