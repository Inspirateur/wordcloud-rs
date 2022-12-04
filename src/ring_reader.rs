#[derive(Clone)]
pub struct RingReader<E: Clone + Copy> {
    data: Vec<E>,
    start: usize,
    end: usize,
}

impl<E: Clone + Copy> RingReader<E> {
    pub fn new(data: Vec<E>) -> Self {
        Self {
            end: data.len(),
            data: data,
            start: 0
        }
    }

    pub fn next(&mut self) -> Option<E> {
        if self.start == self.end {
            return None
        }
        if self.start >= self.data.len() {
            self.start = 0;
        }
        let res = self.data[self.start];
        self.start += 1;
        Some(res)
    }

    pub fn reset(&mut self) {
        self.end = if self.start == 0 {
            self.data.len()
        } else {
            self.start-1
        };
    }
}