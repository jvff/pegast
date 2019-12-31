mod buffered_iterator;

pub trait Input {
    fn position(&self) -> usize;
    fn advance(&mut self, amount: usize);
    fn advance_to(&mut self, position: usize);
    fn check(&mut self, string: &str) -> bool;
    fn consume(&mut self, string: &str) -> bool;
    fn next(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
}
