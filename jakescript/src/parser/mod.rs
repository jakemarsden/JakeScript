pub use error::*;

use crate::ast::*;
use crate::iter::{IntoPeekableNth, PeekableNth};
use crate::lexer::{self, Keyword, Lexer, Punctuator, Token, Tokens};
use crate::str::NonEmptyString;
use error::AllowToken::Exactly;
use std::{io, iter};

mod block;
mod declaration;
mod error;
mod expression;
mod literal;
mod statement;
#[cfg(test)]
mod test;

type Fallible<I> = iter::Map<I, fn(Token) -> lexer::Result<Token>>;

pub struct Parser<I: Iterator<Item = lexer::Result<Token>>> {
    tokens: PeekableNth<I>,
}

impl<I: Iterator<Item = io::Result<char>>> Parser<Tokens<Lexer<I>>> {
    pub fn for_lexer(source: Lexer<I>) -> Self {
        Self::for_tokens_fallible(source.tokens())
    }
}

impl<I: Iterator<Item = Token>> Parser<Fallible<I>> {
    pub fn for_tokens(source: I) -> Self {
        Self::for_tokens_fallible(source.map(Ok))
    }
}

impl<I: Iterator<Item = lexer::Result<Token>>> Parser<I> {
    pub fn for_tokens_fallible(source: I) -> Self {
        Self {
            tokens: source.peekable_nth(),
        }
    }

    pub fn execute(mut self) -> Result {
        let body = self.parse_multi_statement_block_body()?;
        Ok(Program::new(body))
    }

    fn expect_keyword(&mut self, expected: Keyword) -> Result<()> {
        self.expect_token(Token::Keyword(expected))
    }

    fn expect_punctuator(&mut self, expected: Punctuator) -> Result<()> {
        self.expect_token(Token::Punctuator(expected))
    }

    fn expect_identifier(&mut self, placeholder: NonEmptyString) -> Result<NonEmptyString> {
        match self.tokens.try_next()? {
            Some(Token::Identifier(id)) => Ok(id),
            actual => Err(Error::unexpected(
                Exactly(Token::Identifier(placeholder)),
                actual,
            )),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<()> {
        match self.tokens.try_next()? {
            Some(actual) if actual == expected => Ok(()),
            actual => Err(Error::unexpected(Exactly(expected), actual)),
        }
    }
}
