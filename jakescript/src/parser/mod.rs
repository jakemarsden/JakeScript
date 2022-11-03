use crate::ast::*;
use crate::iter::peek_fallible::{
    IntoPeekableNthFallible, PeekableNthFallible, PeekableNthFallibleIterator,
};
use crate::lexer::{self, Lexer};
use crate::token::{self, Element, Keyword, Punctuator, SourceLocation};
pub use error::*;
use fallible_iterator::FallibleIterator;
use std::{io, iter};

mod block;
mod declaration;
mod error;
mod expression;
mod literal;
mod statement;
#[cfg(test)]
mod test;

type Fallible<I> = fallible_iterator::Convert<iter::Map<I, fn(Element) -> lexer::Result<Element>>>;

pub struct Parser<I: FallibleIterator<Item = Element, Error = lexer::Error>> {
    source: PeekableNthFallible<I>,
}

impl<I: FallibleIterator<Item = char, Error = io::Error>> Parser<Lexer<I>> {
    pub fn for_lexer(source: Lexer<I>) -> Self {
        Self::for_elements_fallible(source)
    }
}

impl<I: Iterator<Item = Element>> Parser<Fallible<I>> {
    pub fn for_elements(source: I) -> Self {
        Self::for_elements_fallible(fallible_iterator::convert(source.map(Ok)))
    }
}

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub fn for_elements_fallible(source: I) -> Self {
        Self {
            source: source.peekable_nth_fallible(),
        }
    }

    pub fn execute(mut self) -> Result {
        // FIXME: Store the script path in the `Script`, not in every single AST node!
        // The path also wouldn't be lost for empty scripts.
        let loc = self
            .source
            .peek()?
            .map(Element::source_location)
            .cloned()
            .unwrap_or_default();

        self.skip_non_tokens()?;
        let body = self.parse_block_body()?;
        Ok(Script::new(loc, body))
    }

    fn skip_non_tokens(&mut self) -> lexer::Result<()> {
        self.source.advance_while(|elem| elem.token().is_none())?;
        Ok(())
    }

    fn expect_keyword(&mut self, expected: Keyword) -> Result<SourceLocation> {
        match self.source.next()? {
            Some(elem) if elem.keyword() == Some(expected) => Ok(elem.source_location().clone()),
            actual => Err(Error::unexpected(expected, actual)),
        }
    }

    fn expect_punctuator(&mut self, expected: Punctuator) -> Result<SourceLocation> {
        match self.source.next()? {
            Some(elem) if elem.punctuator() == Some(expected) => Ok(elem.source_location().clone()),
            actual => Err(Error::unexpected(expected, actual)),
        }
    }

    fn expect_identifier(
        &mut self,
        placeholder: &'static str,
    ) -> Result<(Identifier, SourceLocation)> {
        match self.source.next()? {
            Some(elem) if elem.identifier().is_some() => {
                let loc = elem.source_location().clone();
                Ok((Identifier::from(elem.into_identifier().unwrap()), loc))
            }
            elem => Err(Error::unexpected(Expected::Identifier(placeholder), elem)),
        }
    }

    fn expect_literal(&mut self) -> Result<(token::Literal, SourceLocation)> {
        match self.source.next()? {
            Some(elem) if elem.literal().is_some() => {
                let loc = elem.source_location().clone();
                Ok((elem.into_literal().unwrap(), loc))
            }
            elem => Err(Error::unexpected(Expected::Literal, elem)),
        }
    }
}
