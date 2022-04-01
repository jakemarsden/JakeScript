pub use error::*;

use crate::ast::*;
use crate::iter::peek_fallible::{
    IntoPeekableNthFallible, PeekableNthFallible, PeekableNthFallibleIterator,
};
use crate::lexer::{self, Lexer};
use crate::str::NonEmptyString;
use crate::token::{self, Element, Keyword, Punctuator, Token};
use error::AllowToken::{Exactly, Unspecified};
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
        self.skip_non_tokens()?;
        let body = self.parse_multi_statement_block_body()?;
        Ok(Script::new(body))
    }

    fn skip_non_tokens(&mut self) -> lexer::Result<()> {
        self.source.advance_while(|elem| elem.token().is_none())?;
        Ok(())
    }

    fn expect_keyword(&mut self, expected: Keyword) -> Result<()> {
        match self.source.next()? {
            Some(elem) if elem.keyword() == Some(expected) => Ok(()),
            elem => Err(Error::unexpected(Exactly(Token::Keyword(expected)), elem)),
        }
    }

    fn expect_punctuator(&mut self, expected: Punctuator) -> Result<()> {
        match self.source.next()? {
            Some(elem) if elem.punctuator() == Some(expected) => Ok(()),
            elem => Err(Error::unexpected(
                Exactly(Token::Punctuator(expected)),
                elem,
            )),
        }
    }

    fn expect_identifier(&mut self, placeholder: NonEmptyString) -> Result<Identifier> {
        match self.source.next()? {
            Some(elem) if elem.identifier().is_some() => {
                Ok(Identifier::from(elem.into_identifier().unwrap()))
            }
            elem => Err(Error::unexpected(
                Exactly(Token::Identifier(placeholder)),
                elem,
            )),
        }
    }

    fn expect_literal(&mut self) -> Result<token::Literal> {
        match self.source.next()? {
            Some(elem) if elem.literal().is_some() => Ok(elem.into_literal().unwrap()),
            elem => Err(Error::unexpected(Unspecified, elem)),
        }
    }
}
