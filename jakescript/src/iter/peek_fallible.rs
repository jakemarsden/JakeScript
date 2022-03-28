use fallible_iterator::{DoubleEndedFallibleIterator, FallibleIterator};
use std::collections::VecDeque;

pub trait IntoPeekableNthFallible: FallibleIterator {
    fn peekable_nth_fallible(self) -> PeekableNthFallible<Self>
    where
        Self: Sized;
}

pub trait PeekableNthFallibleIterator: FallibleIterator {
    fn peek(&mut self) -> Result<Option<&Self::Item>, Self::Error> {
        self.peek_nth(0)
    }

    fn peek_nth(&mut self, n: usize) -> Result<Option<&Self::Item>, Self::Error>;

    fn try_next_if<E>(
        &mut self,
        f: impl FnOnce(&Self::Item) -> Result<bool, E>,
    ) -> Result<Option<Self::Item>, Self::Error>
    where
        Self::Error: From<E>,
    {
        if let Some(item) = self.peek()? && f(item)? {
            Ok(Some(self.next()?.unwrap()))
        } else {
            Ok(None)
        }
    }

    fn next_if(
        &mut self,
        f: impl FnOnce(&Self::Item) -> bool,
    ) -> Result<Option<Self::Item>, Self::Error> {
        self.try_next_if(|item| Ok(f(item)))
    }

    fn next_if_eq<T>(&mut self, expected: &T) -> Result<Option<Self::Item>, Self::Error>
    where
        T: ?Sized,
        Self::Item: PartialEq<T>,
    {
        self.next_if(|item| item == expected)
    }

    fn advance_by(&mut self, n: usize) -> Result<Result<(), usize>, Self::Error> {
        for idx in 0..n {
            if self.next()?.is_none() {
                return Ok(Err(idx));
            }
        }
        Ok(Ok(()))
    }

    fn try_advance_while<E>(
        &mut self,
        mut f: impl FnMut(&Self::Item) -> Result<bool, E>,
    ) -> Result<usize, Self::Error>
    where
        Self::Error: From<E>,
    {
        let mut idx = 0;
        while let Some(item) = self.peek()? && f(item)? {
            self.advance_by(1)?.unwrap();
            idx += 1;
        }
        Ok(idx)
    }

    fn advance_while(
        &mut self,
        mut f: impl FnMut(&Self::Item) -> bool,
    ) -> Result<usize, Self::Error> {
        self.try_advance_while(|item| Ok(f(item)))
    }

    fn try_advance_over_if_eq<T>(&mut self, expected: T) -> Result<Result<(), usize>, Self::Error>
    where
        T: FallibleIterator,
        Self::Item: PartialEq<T::Item>,
        Self::Error: From<T::Error>,
    {
        let mut matching_len = 0;
        let mut expected = expected.enumerate();
        while let Some((idx, ref expected_item)) = expected.next()? {
            match self.peek_nth(idx)? {
                Some(item) if item == expected_item => {}
                Some(_) | None => return Ok(Err(matching_len)),
            }
            matching_len += 1;
        }
        self.advance_by(matching_len)?.unwrap();
        Ok(Ok(()))
    }

    fn advance_over_if_eq<T>(&mut self, expected: T) -> Result<Result<(), usize>, Self::Error>
    where
        T: Iterator,
        Self::Item: PartialEq<T::Item>,
    {
        self.try_advance_over_if_eq(fallible_iterator::convert(expected.map(Ok)))
    }
}

pub struct PeekableNthFallible<I: FallibleIterator> {
    peeked: VecDeque<I::Item>,
    iter: I,
}

impl<I: FallibleIterator> IntoPeekableNthFallible for I {
    fn peekable_nth_fallible(self) -> PeekableNthFallible<I> {
        PeekableNthFallible::new(self)
    }
}

impl<I: FallibleIterator> PeekableNthFallible<I> {
    fn new(iter: I) -> Self {
        Self {
            peeked: VecDeque::new(),
            iter,
        }
    }
}

impl<I: FallibleIterator> FallibleIterator for PeekableNthFallible<I> {
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        match self.peeked.pop_front() {
            Some(item) => Ok(Some(item)),
            None => self.iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let peeked_count = self.peeked.len();
        let (iter_min, iter_max) = self.iter.size_hint();
        (
            iter_min.saturating_add(peeked_count),
            iter_max.and_then(|count| count.checked_add(peeked_count)),
        )
    }
}

impl<I: FallibleIterator> PeekableNthFallibleIterator for PeekableNthFallible<I> {
    fn peek_nth(&mut self, n: usize) -> Result<Option<&Self::Item>, Self::Error> {
        for _ in self.peeked.len()..=n {
            match self.iter.next()? {
                Some(item) => self.peeked.push_back(item),
                None => return Ok(None),
            }
        }
        Ok(Some(self.peeked.get(n).unwrap()))
    }
}

impl<I: DoubleEndedFallibleIterator> DoubleEndedFallibleIterator for PeekableNthFallible<I> {
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        match self.iter.next_back()? {
            Some(item) => Ok(Some(item)),
            None => Ok(self.peeked.pop_back()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{IntoPeekableNthFallible, PeekableNthFallibleIterator};
    use fallible_iterator::{DoubleEndedFallibleIterator, FallibleIterator};

    #[test]
    fn peek() {
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.peek(), Ok(Some(&"foo")));
        assert_eq!(it.next(), Ok(Some("foo")));
        assert_eq!(it.peek(), Ok(Some(&"bar")));
        assert_eq!(it.next(), Ok(Some("bar")));
        assert_eq!(it.peek(), Ok(Some(&"baz")));
        assert_eq!(it.next(), Ok(Some("baz")));
        assert_eq!(it.peek(), Ok(None));
    }

    #[test]
    fn peek_nth() {
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.peek_nth(0), Ok(Some(&"foo")));
        assert_eq!(it.peek_nth(1), Ok(Some(&"bar")));
        assert_eq!(it.peek_nth(2), Ok(Some(&"baz")));
        assert_eq!(it.peek_nth(3), Ok(None));
        assert_eq!(it.peek_nth(usize::MAX), Ok(None));
        assert_eq!(it.next(), Ok(Some("foo")));
        assert_eq!(it.peek_nth(0), Ok(Some(&"bar")));
        assert_eq!(it.peek_nth(1), Ok(Some(&"baz")));
        assert_eq!(it.peek_nth(2), Ok(None));
        assert_eq!(it.next(), Ok(Some("bar")));
        assert_eq!(it.peek_nth(0), Ok(Some(&"baz")));
        assert_eq!(it.peek_nth(1), Ok(None));
        assert_eq!(it.next(), Ok(Some("baz")));
        assert_eq!(it.peek_nth(0), Ok(None));
        assert_eq!(it.peek_nth(usize::MAX), Ok(None));
    }

    #[test]
    fn next() {
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.next(), Ok(Some("foo")));
        assert_eq!(it.peek_nth(1), Ok(Some(&"baz")));
        assert_eq!(it.next(), Ok(Some("bar")));
        assert_eq!(it.next(), Ok(Some("baz")));
        assert_eq!(it.next(), Ok(None));
    }

    #[test]
    fn try_next_if() {
        let predicate1: fn(&&str) -> Result<bool, ()> = |item| {
            assert_eq!(item, &"foo");
            Ok(false)
        };
        let predicate2: fn(&&str) -> Result<bool, ()> = |item| {
            assert_eq!(item, &"foo");
            Ok(true)
        };
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.try_next_if(predicate1), Ok(None));
        assert_eq!(it.try_next_if(predicate2), Ok(Some("foo")));
    }

    #[test]
    fn next_if() {
        let predicate1: fn(&&str) -> bool = |item| {
            assert_eq!(item, &"foo");
            false
        };
        let predicate2: fn(&&str) -> bool = |item| {
            assert_eq!(item, &"foo");
            true
        };
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.next_if(predicate1), Ok(None));
        assert_eq!(it.next_if(predicate2), Ok(Some("foo")));
    }

    #[test]
    fn next_if_eq() {
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.next_if_eq(&"bar"), Ok(None));
        assert_eq!(it.next(), Ok(Some("foo")));
        assert_eq!(it.next_if_eq(&"bar"), Ok(Some("bar")));
        assert_eq!(it.next(), Ok(Some("baz")));
        assert_eq!(it.next_if_eq(&"baz"), Ok(None));
    }

    #[test]
    fn next_back() {
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.next_back(), Ok(Some("baz")));
        assert_eq!(it.next_back(), Ok(Some("bar")));
        assert_eq!(it.next_back(), Ok(Some("foo")));
        assert_eq!(it.next_back(), Ok(None));

        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.peek_nth(2), Ok(Some(&"baz")));
        assert_eq!(it.next_back(), Ok(Some("baz")));
        assert_eq!(it.next_back(), Ok(Some("bar")));
        assert_eq!(it.next_back(), Ok(Some("foo")));
        assert_eq!(it.next_back(), Ok(None));
    }

    #[test]
    fn advance_over_if_eq() {
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.advance_over_if_eq(["bar"].into_iter()), Ok(Err(0)));
        assert_eq!(
            it.advance_over_if_eq(["foo", "baz"].into_iter()),
            Ok(Err(1))
        );
        assert_eq!(it.next(), Ok(Some("foo")));
        assert_eq!(it.advance_over_if_eq(["bar"].into_iter()), Ok(Ok(())));
        assert_eq!(it.next(), Ok(Some("baz")));
        assert_eq!(it.advance_over_if_eq(["baz"].into_iter()), Ok(Err(0)));

        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(
            it.advance_over_if_eq(["foo", "baz"].into_iter()),
            Ok(Err(1))
        );
        assert_eq!(
            it.advance_over_if_eq(["foo", "bar"].into_iter()),
            Ok(Ok(()))
        );
        assert_eq!(it.next(), Ok(Some("baz")));

        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.next(), Ok(Some("foo")));
    }

    #[test]
    fn size_hint() {
        let mut it =
            fallible_iterator::convert::<_, (), _>([Ok("foo"), Ok("bar"), Ok("baz")].into_iter())
                .peekable_nth_fallible();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next(), Ok(Some("foo")));
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.peek_nth(1), Ok(Some(&"baz")));
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next(), Ok(Some("bar")));
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next(), Ok(Some("baz")));
        assert_eq!(it.size_hint(), (0, Some(0)));
    }
}
