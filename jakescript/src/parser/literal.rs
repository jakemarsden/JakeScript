use super::error::AllowToken::{AnyOf, Exactly};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::{Element, Punctuator, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_array_elements(&mut self) -> Result<Vec<Expression>> {
        if matches!(
            self.source.peek()?,
            Some(Element::Token(Token::Punctuator(Punctuator::CloseBracket)))
        ) {
            return Ok(vec![]);
        }

        let mut elems = Vec::new();
        loop {
            self.skip_non_tokens()?;
            elems.push(self.parse_expression()?);
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(Element::Token(Token::Punctuator(Punctuator::Comma))) => {
                    self.source.next()?.unwrap();
                }
                Some(Element::Token(Token::Punctuator(Punctuator::CloseBracket))) => {
                    break Ok(elems);
                }
                actual => {
                    break Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::CloseBracket),
                            vec![],
                        ),
                        actual.cloned(),
                    ))
                }
            }
        }
    }

    pub(super) fn parse_object_properties(&mut self) -> Result<Vec<DeclaredProperty>> {
        let mut props = Vec::new();
        Ok(loop {
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(Element::Token(Token::Punctuator(Punctuator::CloseBrace))) => break props,
                Some(Element::Token(Token::Identifier(_))) => {
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
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(Element::Token(Token::Punctuator(Punctuator::CloseBrace))) => break props,
                Some(Element::Token(Token::Punctuator(Punctuator::Comma))) => {
                    self.source.next()?.unwrap();
                }
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
        let name = match self.source.next()? {
            // TODO: Parse non-identifier declared property names.
            Some(Element::Token(Token::Identifier(id))) => {
                DeclaredPropertyName::Identifier(Identifier::from(id))
            }
            actual => {
                return Err(Error::unexpected(
                    Exactly(Token::Identifier(non_empty_str!("property_key"))),
                    actual,
                ))
            }
        };
        self.skip_non_tokens()?;
        self.expect_punctuator(Punctuator::Colon)?;
        self.skip_non_tokens()?;
        let initialiser = self.parse_expression()?;
        Ok(DeclaredProperty { name, initialiser })
    }
}
