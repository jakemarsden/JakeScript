use std::fmt;
use std::iter::FusedIterator;

pub(crate) trait IntoPeekableNth: Iterator {
    fn peekable_nth(self) -> PeekableNth<Self>
    where
        Self: Sized;
}

impl<I: Iterator> IntoPeekableNth for I {
    fn peekable_nth(self) -> PeekableNth<Self> {
        PeekableNth::with_capacity(self, 32)
    }
}

pub(crate) struct PeekableNth<I: Iterator> {
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

    pub(crate) fn peek(&mut self) -> Option<&I::Item> {
        self.peek_n(0)
    }

    pub(crate) fn peek_n(&mut self, offset: usize) -> Option<&I::Item> {
        while self.buf.len() <= offset {
            let item = self.source.next()?;
            self.buf.push(item);
        }
        Some(self.buf.get(offset).unwrap())
    }

    pub(crate) fn advance(&mut self) {
        self.advance_n(1)
    }

    pub(crate) fn advance_n(&mut self, count: usize) {
        for _ in 0..count {
            self.consume();
        }
    }

    pub(crate) fn consume(&mut self) -> Option<I::Item> {
        if !self.buf.is_empty() {
            Some(self.buf.remove(0))
        } else {
            self.source.next()
        }
    }

    pub(crate) fn consume_if(
        &mut self,
        condition: impl FnOnce(&I::Item) -> bool,
    ) -> Option<I::Item> {
        if condition(self.peek()?) {
            Some(self.consume().unwrap())
        } else {
            None
        }
    }
}

impl<I: Iterator<Item: Eq + fmt::Debug>> PeekableNth<I> {
    pub(crate) fn consume_eq(&mut self, expected: &I::Item) -> Option<I::Item> {
        self.consume_if(|item| item == expected)
    }

    pub(crate) fn consume_exact(&mut self, expected: &I::Item) -> I::Item {
        match self.peek() {
            Some(actual) if actual == expected => self.consume().unwrap(),
            Some(actual) => unreachable!("expected {:?} but was {:?}", expected, actual),
            None => unreachable!("expected {:?} but was <end>", expected),
        }
    }
}

impl<I: Iterator<Item = char>> PeekableNth<I> {
    pub(crate) fn peek_str(&mut self, expected: &str) -> bool {
        self.peek_str_n(0, expected)
    }

    pub(crate) fn peek_str_n(&mut self, offset: usize, expected: &str) -> bool {
        for (idx, ch) in expected.char_indices() {
            if self.peek_n(idx + offset) != Some(&ch) {
                return false;
            }
        }
        true
    }

    pub(crate) fn consume_str(&mut self, expected: &str) -> bool {
        if self.peek_str(expected) {
            self.advance_n(expected.len());
            true
        } else {
            false
        }
    }

    pub(crate) fn consume_until_string(&mut self, condition: impl Fn(&char) -> bool) -> String {
        self.consume_while_string(|ch| !condition(ch))
    }

    pub(crate) fn consume_while_string(&mut self, condition: impl Fn(&char) -> bool) -> String {
        // Optimisation: Check first char outside of the loop to avoid alloc on mismatch.
        if let Some(ch0) = self.consume_if(|ch| condition(ch)) {
            let mut out = String::new();
            out.push(ch0);
            while let Some(ch) = self.consume_if(|ch| condition(ch)) {
                out.push(ch);
            }
            out
        } else {
            String::with_capacity(0)
        }
    }
}

impl<I: Iterator> Iterator for PeekableNth<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume()
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

        assert_eq!(iter.peek_n(4), Some(&'o'));
        assert_eq!(iter.peek_n(3), Some(&'l'));
        assert_eq!(iter.peek_n(2), Some(&'l'));
        assert_eq!(iter.peek_n(1), Some(&'e'));
        assert_eq!(iter.peek_n(0), Some(&'H'));
        assert_eq!(iter.peek_n(7), Some(&'w'));
        assert_eq!(iter.next(), Some('H'));
        assert_eq!(iter.next(), Some('e'));
        assert_eq!(iter.peek(), Some(&'l'));
        assert_eq!(iter.peek_n(10), Some(&'!'));
        assert_eq!(iter.peek_n(11), None);
        assert_eq!(iter.peek_n(usize::MAX), None);
        assert_eq!(iter.collect::<String>(), "llo, world!");
    }
}
