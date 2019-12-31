mod buffered_iterator;

use self::buffered_iterator::BufferedIterator;

pub trait Input {
    fn position(&self) -> usize;
    fn advance(&mut self, amount: usize);
    fn advance_to(&mut self, position: usize);
    fn check(&mut self, string: &str) -> bool;
    fn consume(&mut self, string: &str) -> bool;
    fn next(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
}

pub struct ConsumingInput<T: Iterator<Item = char>> {
    position: usize,
    iterator: BufferedIterator<T>,
}

impl<T> ConsumingInput<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(source: impl IntoIterator<Item = char, IntoIter = T>) -> Self {
        ConsumingInput {
            position: 0,
            iterator: BufferedIterator::new(source),
        }
    }
}

impl<T> Input for ConsumingInput<T>
where
    T: Iterator<Item = char>,
{
    fn position(&self) -> usize {
        self.position
    }

    fn advance(&mut self, amount: usize) {
        self.iterator.advance(amount);
    }

    fn advance_to(&mut self, position: usize) {
        assert!(
            position >= self.position,
            "Attempt to advance input backwards"
        );

        self.advance(position - self.position);
    }

    fn check(&mut self, string: &str) -> bool {
        let mut count = 0;

        for (a, b) in string.chars().zip(self.iterator.peek(string.len())) {
            if a != *b {
                return false;
            }

            count += 1;
        }

        if count == string.len() {
            true
        } else {
            false
        }
    }

    fn consume(&mut self, string: &str) -> bool {
        if self.check(string) {
            self.advance(string.len());

            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Option<char> {
        let result = self.iterator.next();

        if result.is_some() {
            self.advance(1);
        }

        result
    }

    fn peek(&mut self) -> Option<char> {
        self.iterator.peek(1).next().cloned()
    }
}
