use crate::iter::peek_fallible::{
    IntoPeekableNthFallible, PeekableNthFallible, PeekableNthFallibleIterator,
};
use crate::token::symbol::{is_line_terminator, CR, LF};
use crate::token::SourceLocation;
use fallible_iterator::{self, FallibleIterator};
use std::{io, iter, str};

pub(super) type Fallible<I> =
    fallible_iterator::Convert<iter::Map<I, fn(char) -> io::Result<char>>>;

pub(super) struct SourceCode<I>
where
    I: FallibleIterator<Item = char, Error = io::Error>,
{
    source: PeekableNthFallible<I>,
    location: SourceLocation,
    crlf_sequence: bool,
}

impl<'a> SourceCode<Fallible<str::Chars<'a>>> {
    pub(super) fn for_str(source: &'a str, start_loc: SourceLocation) -> Self {
        Self::for_chars(source.chars(), start_loc)
    }
}

impl<I> SourceCode<Fallible<I>>
where
    I: Iterator<Item = char>,
{
    pub(super) fn for_chars(source: I, start_loc: SourceLocation) -> Self {
        Self::for_chars_fallible(fallible_iterator::convert(source.map(Ok)), start_loc)
    }
}

impl<I> SourceCode<I>
where
    I: FallibleIterator<Item = char, Error = io::Error>,
{
    pub(super) fn for_chars_fallible(source: I, start_loc: SourceLocation) -> Self {
        Self::new(source, start_loc)
    }

    fn new(source: I, start_loc: SourceLocation) -> Self {
        Self {
            source: source.peekable_nth_fallible(),
            location: start_loc,
            crlf_sequence: false,
        }
    }

    pub(super) fn location(&self) -> &SourceLocation {
        &self.location
    }
}

impl<I> FallibleIterator for SourceCode<I>
where
    I: FallibleIterator<Item = char, Error = io::Error>,
{
    type Error = (io::Error, SourceLocation);
    type Item = char;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        let ch = self
            .source
            .next()
            .map_err(|err| (err, self.location().clone()))?;
        match ch {
            Some(LF) if self.crlf_sequence => {
                // This is the LF of a CRLF line terminator sequence. The line
                // was already advanced at the start of the sequence so don't
                // advance it now.
            }
            Some(ch) if is_line_terminator(ch) => {
                self.location.advance_line();
            }
            Some(_) => {
                self.location.advance_column();
            }
            None => {}
        }
        self.crlf_sequence = matches!(ch, Some(CR));
        Ok(ch)
    }
}

impl<I> PeekableNthFallibleIterator for SourceCode<I>
where
    I: FallibleIterator<Item = char, Error = io::Error>,
{
    fn peek_nth(&mut self, n: usize) -> Result<Option<&Self::Item>, Self::Error> {
        let loc = self.location().clone();
        self.source.peek_nth(n).map_err(|err| (err, loc))
    }
}
