use super::error::AllowToken::{AnyOf, Exactly};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::{self, *};
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::parser::block::Braces;
use crate::token::Keyword::Function;
use crate::token::Punctuator::{
    CloseBrace, CloseBracket, Colon, Comma, OpenBrace, OpenBracket, OpenParen,
};
use crate::token::{self, Element, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_literal_expression(&mut self) -> Result<LiteralExpression> {
        let value = match self.expect_literal()? {
            token::Literal::Boolean(value) => ast::Literal::Boolean(value),
            token::Literal::Numeric(
                token::NumericLiteral::BinInt(value)
                | token::NumericLiteral::OctInt(value)
                | token::NumericLiteral::DecInt(value)
                | token::NumericLiteral::HexInt(value),
            ) => ast::Literal::Numeric(ast::NumericLiteral::Int(value)),
            token::Literal::Numeric(token::NumericLiteral::Decimal(value)) => {
                ast::Literal::Numeric(ast::NumericLiteral::Float(value))
            }
            token::Literal::String(value) => {
                ast::Literal::String(ast::StringLiteral { value: value.value })
            }
            token::Literal::RegEx(value) => {
                // FIXME: Support Literal::RegEx properly.
                ast::Literal::String(ast::StringLiteral {
                    value: value.to_string(),
                })
            }
            token::Literal::Null => ast::Literal::Null,
        };
        Ok(LiteralExpression { value })
    }

    pub(super) fn parse_array_literal_expression(&mut self) -> Result<ArrayExpression> {
        self.expect_punctuator(OpenBracket)?;
        self.skip_non_tokens()?;
        let declared_elements = self.parse_array_elements()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseBracket)?;
        Ok(ArrayExpression { declared_elements })
    }

    fn parse_array_elements(&mut self) -> Result<Vec<Expression>> {
        if let Some(elem) = self.source.peek()? && elem.punctuator() == Some(CloseBracket) {
            return Ok(vec![]);
        }

        let mut elems = Vec::new();
        loop {
            self.skip_non_tokens()?;
            elems.push(self.parse_expression()?);
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(elem) if elem.punctuator() == Some(Comma) => {
                    self.source.next()?.unwrap();
                }
                Some(elem) if elem.punctuator() == Some(CloseBracket) => {
                    break Ok(elems);
                }
                elem => {
                    break Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Comma),
                            Token::Punctuator(CloseBracket),
                            vec![],
                        ),
                        elem.cloned(),
                    ))
                }
            }
        }
    }

    pub(super) fn parse_object_literal_expression(&mut self) -> Result<ObjectExpression> {
        self.expect_punctuator(OpenBrace)?;
        self.skip_non_tokens()?;
        let declared_properties = self.parse_object_properties()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseBrace)?;
        Ok(ObjectExpression {
            declared_properties,
        })
    }

    fn parse_object_properties(&mut self) -> Result<Vec<DeclaredProperty>> {
        let mut props = Vec::new();
        Ok(loop {
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(elem) if elem.punctuator() == Some(CloseBrace) => break props,
                Some(elem) if elem.identifier().is_some() => {
                    props.push(self.parse_object_property()?);
                }
                elem => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(CloseBrace),
                            Token::Identifier(non_empty_str!("property_key")),
                            vec![],
                        ),
                        elem.cloned(),
                    ))
                }
            }
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(elem) if elem.punctuator() == Some(CloseBrace) => break props,
                Some(elem) if elem.punctuator() == Some(Comma) => {
                    self.source.next()?.unwrap();
                }
                elem => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(CloseBrace),
                            Token::Punctuator(Comma),
                            vec![],
                        ),
                        elem.cloned(),
                    ))
                }
            }
        })
    }

    fn parse_object_property(&mut self) -> Result<DeclaredProperty> {
        let name = match self.source.next()? {
            // TODO: Parse non-identifier declared property names.
            Some(elem) if elem.identifier().is_some() => {
                DeclaredPropertyName::Identifier(Identifier::from(elem.into_identifier().unwrap()))
            }
            elem => {
                return Err(Error::unexpected(
                    Exactly(Token::Identifier(non_empty_str!("property_key"))),
                    elem,
                ))
            }
        };
        self.skip_non_tokens()?;
        self.expect_punctuator(Colon)?;
        self.skip_non_tokens()?;
        let initialiser = self.parse_expression()?;
        Ok(DeclaredProperty { name, initialiser })
    }

    pub(super) fn parse_function_literal_expression(&mut self) -> Result<FunctionExpression> {
        self.expect_keyword(Function)?;
        self.skip_non_tokens()?;
        let binding = match self.source.peek()? {
            Some(elem) if elem.identifier().is_some() => {
                Some(self.expect_identifier(non_empty_str!("function_name"))?)
            }
            Some(elem) if elem.punctuator() == Some(OpenParen) => None,
            elem => {
                return Err(Error::unexpected(
                    AnyOf(
                        Token::Punctuator(OpenParen),
                        Token::Identifier(non_empty_str!("function_name")),
                        vec![],
                    ),
                    elem.cloned(),
                ))
            }
        };
        self.skip_non_tokens()?;
        let formal_parameters = self.parse_fn_parameters()?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Require)?;
        Ok(FunctionExpression {
            binding,
            formal_parameters,
            body,
        })
    }
}
