use std::fmt;
use std::iter::FusedIterator;

pub trait IntoPeekableNth: Iterator {
    fn peekable_nth(self) -> PeekableNth<Self>
    where
        Self: Sized;
}

impl<I: Iterator> IntoPeekableNth for I {
    fn peekable_nth(self) -> PeekableNth<Self> {
        PeekableNth::with_capacity(self, 32)
    }
}

pub struct PeekableNth<I: Iterator> {
    source: I,
    buf: Vec<I::Item>,
}

impl<I: Iterator> PeekableNth<I> {
    // TODO: Investigate: Is using `SmallVec` for the buffer is a good optimisation?
    fn with_capacity(source: I, capacity: usize) -> Self {
        Self {
            source,
            buf: Vec::with_capacity(capacity),
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    pub fn peek_nth(&mut self, offset: usize) -> Option<&I::Item> {
        while self.buf.len() <= offset {
            let item = self.source.next()?;
            self.buf.push(item);
        }
        Some(self.buf.get(offset).unwrap())
    }

    pub fn next_if(&mut self, condition: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        if condition(self.peek()?) {
            Some(self.next().unwrap())
        } else {
            None
        }
    }

    pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
    }

    pub fn next_exact<T>(&mut self, expected: &T) -> I::Item
    where
        T: ?Sized + fmt::Debug,
        I::Item: PartialEq<T> + fmt::Debug,
    {
        match self.peek() {
            Some(actual) if actual == expected => self.next().unwrap(),
            Some(actual) => unreachable!("expected {:?} but was {:?}", expected, actual),
            None => unreachable!("expected {:?} but was <end>", expected),
        }
    }

    pub fn collect_until<B>(&mut self, condition: impl Fn(&I::Item) -> bool) -> B
    where
        B: FromIterator<I::Item>,
    {
        self.collect_while(|next| !condition(next))
    }

    pub fn collect_while<B>(&mut self, condition: impl Fn(&I::Item) -> bool) -> B
    where
        B: FromIterator<I::Item>,
    {
        let mut idx = 0;
        loop {
            match self.peek_nth(idx) {
                Some(item) if condition(item) => idx += 1,
                Some(_) | None => break,
            }
        }
        FromIterator::from_iter(self.take(idx))
    }

    fn try_consume_all<I2>(&mut self, expected: I2) -> bool
    where
        I::Item: PartialEq<I2::Item>,
        I2: Iterator,
    {
        let mut count = 0;
        for (idx, ref expected) in expected.enumerate() {
            match self.peek_nth(idx) {
                Some(item) if item == expected => count += 1,
                Some(_) | None => return false,
            }
        }
        self.advance_by(count).unwrap();
        true
    }
}

impl<I: Iterator<Item = char>> PeekableNth<I> {
    pub fn try_consume_str(&mut self, expected: &str) -> bool {
        self.try_consume_all(expected.chars())
    }
}

impl<I: Iterator> Iterator for PeekableNth<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buf.is_empty() {
            Some(self.buf.remove(0))
        } else {
            self.source.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let buf_size = self.buf.len();
        match self.source.size_hint() {
            (lower, Some(upper)) => (lower.saturating_add(buf_size), upper.checked_add(buf_size)),
            (lower, None) => (lower.saturating_add(buf_size), None),
        }
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for PeekableNth<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.source.next_back() {
            Some(item) => Some(item),
            None => self.buf.pop(),
        }
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for PeekableNth<I> {}

impl<I: FusedIterator> FusedIterator for PeekableNth<I> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        let mut iter = "Hello, world!".chars().peekable_nth();

        assert_eq!(iter.peek_nth(4), Some(&'o'));
        assert_eq!(iter.peek_nth(3), Some(&'l'));
        assert_eq!(iter.peek_nth(2), Some(&'l'));
        assert_eq!(iter.peek_nth(1), Some(&'e'));
        assert_eq!(iter.peek_nth(0), Some(&'H'));
        assert_eq!(iter.peek_nth(7), Some(&'w'));
        assert_eq!(iter.next(), Some('H'));
        assert_eq!(iter.next(), Some('e'));
        assert_eq!(iter.peek(), Some(&'l'));
        assert_eq!(iter.peek_nth(10), Some(&'!'));
        assert_eq!(iter.peek_nth(11), None);
        assert_eq!(iter.peek_nth(usize::MAX), None);
        assert_eq!(iter.collect::<String>(), "llo, world!");
    }

    #[test]
    fn collect_while() {
        let mut iter = "Hello, world!".chars().peekable_nth();

        let hello: Vec<_> = iter.collect_while(|ch| *ch != ',');
        assert_eq!(hello, ['H', 'e', 'l', 'l', 'o']);
        assert_eq!(iter.next(), Some(','));

        let space: Vec<_> = iter.collect_while(|ch| !ch.is_ascii_alphabetic());
        assert_eq!(space, [' ']);
        assert_eq!(iter.peek(), Some(&'w'));

        let world: String = iter.collect_while(|_| true);
        assert_eq!(world, "world!");
        assert_eq!(iter.peek(), None);
    }
}
