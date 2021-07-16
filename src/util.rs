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
        self.advance_n(1)
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
}

impl<T> Stream<T>
where
    T: Eq + fmt::Debug,
{
    pub(crate) fn consume_eq(&mut self, expected: &T) -> Option<T> {
        self.consume_if(|item| item == expected)
    }

    pub(crate) fn consume_exact(&mut self, expected: &T) -> T {
        match self.peek() {
            Some(actual) if actual == expected => self.consume().unwrap(),
            Some(actual) => unreachable!("expected {:?} but was {:?}", expected, actual),
            None => unreachable!("expected {:?} but was <end>", expected),
        }
    }
}

impl Stream<char> {
    pub(crate) fn peek_str(&self, expected: &str) -> bool {
        self.peek_str_n(0, expected)
    }

    pub(crate) fn peek_str_n(&self, offset: usize, expected: &str) -> bool {
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
        // Optimisation: check first char outside of the loop to avoid alloc on mismatch
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
