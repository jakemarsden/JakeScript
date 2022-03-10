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
}

impl<T, E, I: Iterator<Item = Result<T, E>>> PeekableNth<I> {
    pub fn try_peek(&mut self) -> Result<Option<&T>, E>
    where
        E: 'static,
    {
        self.try_peek_nth(0)
    }

    pub fn try_peek_nth(&mut self, offset: usize) -> Result<Option<&T>, E>
    where
        E: 'static,
    {
        match self.peek_nth(offset) {
            Some(Ok(_)) => {
                if let Some(Ok(value)) = self.peek_nth(offset) {
                    Ok(Some(value))
                } else {
                    panic!()
                }
            }
            Some(Err(_)) => {
                self.advance_by(offset).unwrap();
                if let Some(Err(err)) = self.next() {
                    Err(err)
                } else {
                    panic!()
                }
            }
            None => Ok(None),
        }
    }

    pub fn try_next(&mut self) -> Result<Option<T>, E> {
        self.next().transpose()
    }

    pub fn try_next_if(&mut self, condition: impl FnOnce(&T) -> bool) -> Result<Option<T>, E>
    where
        E: 'static,
    {
        Ok(match self.try_peek()? {
            Some(next) if condition(next) => {
                if let Some(Ok(item)) = self.next() {
                    Some(item)
                } else {
                    panic!()
                }
            }
            Some(_) | None => None,
        })
    }

    pub fn try_next_if_eq<U>(&mut self, expected: &U) -> Result<Option<T>, E>
    where
        T: PartialEq<U>,
        E: 'static,
        U: ?Sized,
    {
        self.try_next_if(|next| next == expected)
    }

    pub fn try_next_exact<U>(&mut self, expected: &U) -> Result<(), E>
    where
        T: PartialEq<U> + fmt::Debug,
        E: 'static,
        U: ?Sized + fmt::Debug,
    {
        match self.try_peek()? {
            Some(actual) if actual == expected => {
                if let Some(Ok(_)) = self.next() {
                    Ok(())
                } else {
                    panic!()
                }
            }
            Some(actual) => unreachable!("expected {:?} but was {:?}", expected, actual),
            None => unreachable!("expected {:?} but was <end>", expected),
        }
    }

    pub fn try_collect_until<B>(&mut self, condition: impl Fn(&T) -> bool) -> Result<B, E>
    where
        E: 'static,
        B: FromIterator<T>,
    {
        self.try_collect_while(|next| !condition(next))
    }

    pub fn try_collect_while<B>(&mut self, condition: impl Fn(&T) -> bool) -> Result<B, E>
    where
        E: 'static,
        B: FromIterator<T>,
    {
        let mut idx = 0;
        loop {
            match self.try_peek_nth(idx)? {
                Some(item) if condition(item) => idx += 1,
                Some(_) | None => break,
            }
        }
        let partial_iter = self
            .take(idx)
            .map(|item| if let Ok(item) = item { item } else { panic!() });
        Ok(partial_iter.collect())
    }

    pub fn try_consume_str(&mut self, expected: &str) -> Result<bool, E>
    where
        T: PartialEq<char>,
        E: 'static,
    {
        let mut len = 0;
        for (idx, ch) in expected.chars().enumerate() {
            match self.try_peek_nth(idx)? {
                Some(item) if item == &ch => len += 1,
                Some(_) | None => return Ok(false),
            }
        }
        self.advance_by(len).unwrap();
        Ok(true)
    }
}

impl<I: Iterator> Iterator for PeekableNth<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            self.source.next()
        } else {
            Some(self.buf.remove(0))
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
    use super::IntoPeekableNth;
    use std::convert::Infallible;

    #[test]
    fn peek() {
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
    fn try_collect_while() {
        let mut iter = "Hello, world!"
            .chars()
            .map(Ok as fn(char) -> Result<char, Infallible>)
            .peekable_nth();

        let hello: Vec<_> = iter.try_collect_while(char::is_ascii_alphabetic).unwrap();
        assert_eq!(hello, ['H', 'e', 'l', 'l', 'o']);
        assert_eq!(iter.next(), Some(Ok(',')));

        let empty: Vec<_> = iter.try_collect_while(|_| false).unwrap();
        assert_eq!(empty, []);
        assert_eq!(iter.peek(), Some(&Ok(' ')));

        let space: Vec<_> = iter.try_collect_until(char::is_ascii_alphabetic).unwrap();
        assert_eq!(space, [' ']);
        assert_eq!(iter.peek(), Some(&Ok('w')));

        let world: String = iter.try_collect_while(|_| true).unwrap();
        assert_eq!(world, "world!");
        assert_eq!(iter.peek(), None);

        let empty: String = iter.try_collect_while(|_| true).unwrap();
        assert_eq!(empty, "");
        assert_eq!(iter.peek(), None);
    }
}
