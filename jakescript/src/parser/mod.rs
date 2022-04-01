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
        self.source
            .advance_while(|elem| !matches!(elem, Element::Token(..)))
            .map(|_| ())
    }

    fn expect_keyword(&mut self, expected: Keyword) -> Result<()> {
        self.expect_token(Token::Keyword(expected))
    }

    fn expect_punctuator(&mut self, expected: Punctuator) -> Result<()> {
        self.expect_token(Token::Punctuator(expected))
    }

    fn expect_identifier(&mut self, placeholder: NonEmptyString) -> Result<Identifier> {
        match self.source.next()? {
            Some(Element::Token(Token::Identifier(id))) => Ok(Identifier::from(id)),
            actual => Err(Error::unexpected(
                Exactly(Token::Identifier(placeholder)),
                actual,
            )),
        }
    }

    fn expect_literal(&mut self) -> Result<token::Literal> {
        match self.source.next()? {
            Some(Element::Token(Token::Literal(literal))) => Ok(literal),
            actual => Err(Error::unexpected(Unspecified, actual)),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<()> {
        match self.source.next()? {
            Some(Element::Token(actual)) if actual == expected => Ok(()),
            actual => Err(Error::unexpected(Exactly(expected), actual)),
        }
    }
}
