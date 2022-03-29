use super::error::AllowToken::{AnyOf, Exactly};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::{Punctuator, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Token, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_array_literal_elements(&mut self) -> Result<Vec<Expression>> {
        if matches!(
            self.tokens.peek()?,
            Some(Token::Punctuator(Punctuator::CloseBracket))
        ) {
            return Ok(Vec::with_capacity(0));
        }
        let mut elems = Vec::new();
        loop {
            elems.push(self.parse_expression()?);
            match self.tokens.peek()? {
                Some(&Token::Punctuator(Punctuator::Comma)) => {
                    assert!(self
                        .tokens
                        .next_if_eq(&Token::Punctuator(Punctuator::Comma))?
                        .is_some());
                }
                Some(&Token::Punctuator(Punctuator::CloseBracket)) => break Ok(elems),
                token => {
                    break Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::CloseBracket),
                            vec![],
                        ),
                        token.cloned(),
                    ))
                }
            }
        }
    }

    pub(super) fn parse_object_properties(&mut self) -> Result<Vec<DeclaredProperty>> {
        let mut props = Vec::new();
        Ok(loop {
            match self.tokens.peek()? {
                Some(Token::Punctuator(Punctuator::CloseBrace)) => break props,
                Some(Token::Identifier(_)) => {
                    props.push(self.parse_object_property()?);
                }
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::CloseBrace),
                            Token::Identifier(non_empty_str!("property_key")),
                            vec![],
                        ),
                        actual.cloned(),
                    ))
                }
            }
            match self.tokens.peek()? {
                Some(Token::Punctuator(Punctuator::CloseBrace)) => break props,
                Some(Token::Punctuator(Punctuator::Comma)) => assert!(self
                    .tokens
                    .next_if_eq(&Token::Punctuator(Punctuator::Comma))?
                    .is_some()),
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::CloseBrace),
                            Token::Punctuator(Punctuator::Comma),
                            vec![],
                        ),
                        actual.cloned(),
                    ))
                }
            }
        })
    }

    fn parse_object_property(&mut self) -> Result<DeclaredProperty> {
        let name = match self.tokens.next()? {
            // TODO: Parse non-identifier declared property names.
            Some(Token::Identifier(id)) => DeclaredPropertyName::Identifier(Identifier::from(id)),
            actual => {
                return Err(Error::unexpected(
                    Exactly(Token::Identifier(non_empty_str!("property_key"))),
                    actual,
                ))
            }
        };
        self.expect_punctuator(Punctuator::Colon)?;
        let initialiser = self.parse_expression()?;
        Ok(DeclaredProperty { name, initialiser })
    }
}
