use std::collections::LinkedList;
use std::fmt;
use std::iter::Iterator;

pub(crate) struct Stream<T> {
    items: LinkedList<T>,
}

impl<T> Stream<T> {
    pub(crate) fn new(items: impl Iterator<Item = T>) -> Self {
        Self {
            items: items.collect(),
        }
    }

    pub(crate) fn peek(&self) -> Option<&T> {
        self.peek_n(0)
    }

    pub(crate) fn peek_n(&self, offset: usize) -> Option<&T> {
        self.items.iter().nth(offset)
    }

    pub(crate) fn advance(&mut self) {
        self.advance_n(1);
    }

    pub(crate) fn advance_n(&mut self, count: usize) {
        for _ in 0..count {
            self.consume();
        }
    }

    pub(crate) fn consume(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub(crate) fn consume_if(&mut self, condition: impl FnOnce(&T) -> bool) -> Option<T> {
        if condition(self.peek()?) {
            Some(self.consume().unwrap())
        } else {
            None
        }
    }

    pub(crate) fn consume_until(&mut self, condition: impl Fn(&T) -> bool) -> Vec<T> {
        self.consume_while(|item| !condition(item))
    }

    pub(crate) fn consume_while(&mut self, condition: impl Fn(&T) -> bool) -> Vec<T> {
        let mut items = Vec::new();
        while let Some(item) = self.consume_if(|item| condition(item)) {
            items.push(item);
        }
        items
    }
}

impl<T> Stream<T>
where
    T: Eq + fmt::Debug,
{
    pub(crate) fn consume_exact(&mut self, expected: &T) -> T {
        match self.peek() {
            Some(actual) if actual == expected => self.consume().unwrap(),
            Some(actual) => unreachable!("expected {:?} but was {:?}", expected, actual),
            None => unreachable!("expected {:?} but was <end>", expected),
        }
    }
}

impl Stream<char> {
    pub(crate) fn consume_str(&mut self, expected: &str) -> bool {
        for (idx, ch) in expected.char_indices() {
            if self.peek_n(idx) != Some(&ch) {
                return false;
            }
        }
        self.advance_n(expected.len());
        true
    }
}
