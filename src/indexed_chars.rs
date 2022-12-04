trait Palette<E> {
    fn index(&mut self, elem: E) -> usize;
}

impl<E: Eq> Palette<E> for Vec<E> {
    fn index(&mut self, elem: E) -> usize {
        self.iter().position(|other| *other == elem).unwrap_or({
            // Element is not present in the palette
            self.push(elem);
            self.len() - 1
        })
    }
}

pub struct IndexedChars {
    pub indexes: Vec<usize>,
    pub chars: Vec<char>
}

impl IndexedChars {
    pub fn new(text: &String) -> Self {
        let mut indexes = Vec::new();
        let mut chars = Vec::new();
        for c in text.chars() {
            indexes.push(chars.index(c));
        }
        Self { indexes, chars }
    }
}
