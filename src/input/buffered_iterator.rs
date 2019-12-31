use std::collections::VecDeque;

pub struct BufferedIterator<T: Iterator> {
    buffer: VecDeque<T::Item>,
    source: T,
}

impl<T> BufferedIterator<T>
where
    T: Iterator,
{
    pub fn new(elements: impl IntoIterator<Item = T::Item, IntoIter = T>) -> Self {
        BufferedIterator {
            buffer: VecDeque::new(),
            source: elements.into_iter(),
        }
    }

    pub fn peek(&mut self, count: usize) -> impl Iterator<Item = &T::Item> {
        if count > self.buffer.len() {
            for _ in 0..(count - self.buffer.len()) {
                match self.source.next() {
                    Some(item) => self.buffer.push_back(item),
                    None => break,
                }
            }
        }

        self.buffer.iter().take(count)
    }

    pub fn advance(&mut self, count: usize) -> usize {
        let initial_buffer_length = self.buffer.len();

        for _ in 0..count {
            if self.buffer.pop_front().is_none() {
                break;
            }
        }

        let mut removed_count = initial_buffer_length;

        if count > removed_count {
            for _ in 0..(count - removed_count) {
                if self.source.next().is_none() {
                    break;
                }

                removed_count += 1;
            }
        }

        removed_count
    }

    pub fn peek_into<'a>(&'a mut self) -> PeekingIntoBufferedIterator<'a, T> {
        PeekingIntoBufferedIterator::new(self)
    }
}

impl<T> Iterator for BufferedIterator<T>
where
    T: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.len() > 0 {
            self.buffer.pop_front()
        } else {
            self.source.next()
        }
    }
}

pub struct PeekingIntoBufferedIterator<'a, T: Iterator> {
    inner: &'a mut BufferedIterator<T>,
    position: usize,
}

impl<'a, T> PeekingIntoBufferedIterator<'a, T>
where
    T: Iterator,
{
    pub fn new(target: &'a mut BufferedIterator<T>) -> Self {
        PeekingIntoBufferedIterator {
            inner: target,
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn peek(&mut self, count: usize) -> impl Iterator<Item = &T::Item> {
        self.inner.peek(count + self.position).skip(self.position)
    }

    pub fn peek_into<'b>(&'b mut self) -> PeekingIntoBufferedIterator<'b, T> {
        PeekingIntoBufferedIterator {
            inner: &mut self.inner,
            position: self.position,
        }
    }

    pub fn advance(&mut self, count: usize) {
        self.position += count;
    }
}

impl<'a, T> Iterator for PeekingIntoBufferedIterator<'a, T>
where
    T: Iterator,
    T::Item: Copy,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.position;

        self.position += 1;

        if index < self.inner.buffer.len() {
            Some(self.inner.buffer[index])
        } else if let Some(item) = self.inner.source.next() {
            self.inner.buffer.push_back(item);
            Some(item)
        } else {
            self.position -= 1;
            None
        }
    }
}
