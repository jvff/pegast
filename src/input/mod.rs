mod buffered_iterator;

use self::buffered_iterator::{BufferedIterator, PeekingIntoBufferedIterator};

pub trait Input {
    fn position(&self) -> usize;
    fn advance(&mut self, amount: usize);
    fn advance_to(&mut self, position: usize);
    fn check(&mut self, string: &str) -> bool;
    fn consume(&mut self, string: &str) -> bool;
    fn next(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
    // TODO: Use generic associated types when available
    fn peek_only<'a>(&'a mut self) -> Box<dyn Input + 'a>;
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

    fn peek_only<'a>(&'a mut self) -> Box<dyn Input + 'a> {
        Box::new(PeekingInput::new(self.position, &mut self.iterator))
    }
}

pub struct PeekingInput<'a, T: Iterator> {
    start_position: usize,
    iterator: PeekingIntoBufferedIterator<'a, T>,
}

impl<'a, T> PeekingInput<'a, T>
where
    T: Iterator<Item = char>,
{
    pub fn new(start_position: usize, target: &'a mut BufferedIterator<T>) -> Self {
        PeekingInput {
            start_position,
            iterator: PeekingIntoBufferedIterator::new(target),
        }
    }
}

impl<'i, T> Input for PeekingInput<'i, T>
where
    T: Iterator<Item = char>,
{
    fn position(&self) -> usize {
        self.start_position + self.iterator.position()
    }

    fn advance(&mut self, amount: usize) {
        self.iterator.advance(amount);
    }

    fn advance_to(&mut self, position: usize) {
        assert!(
            position >= self.position(),
            "Attempt to advance input backwards"
        );

        self.iterator.advance(position - self.position());
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
        self.iterator.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.iterator.peek(1).next().cloned()
    }

    fn peek_only<'a>(&'a mut self) -> Box<dyn Input + 'a> {
        Box::new(PeekingInput {
            iterator: self.iterator.peek_into(),
            start_position: self.start_position,
        })
    }
}

impl<T> Input for Box<T>
where
    T: Input + ?Sized,
{
    fn position(&self) -> usize {
        self.as_ref().position()
    }

    fn advance(&mut self, amount: usize) {
        self.as_mut().advance(amount)
    }

    fn advance_to(&mut self, position: usize) {
        self.as_mut().advance_to(position)
    }

    fn check(&mut self, string: &str) -> bool {
        self.as_mut().check(string)
    }

    fn consume(&mut self, string: &str) -> bool {
        self.as_mut().consume(string)
    }

    fn next(&mut self) -> Option<char> {
        self.as_mut().next()
    }

    fn peek(&mut self) -> Option<char> {
        self.as_mut().peek()
    }

    fn peek_only<'a>(&'a mut self) -> Box<dyn Input + 'a> {
        self.as_mut().peek_only()
    }
}
