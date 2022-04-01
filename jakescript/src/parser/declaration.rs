use super::block::Braces;
use super::error::AllowToken::AnyOf;
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::{Element, Keyword, Punctuator, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_declaration(&mut self) -> Result<Declaration> {
        match self.source.peek()? {
            Some(Element::Token(Token::Keyword(Keyword::Function))) => {
                self.parse_function_declaration().map(Declaration::Function)
            }
            Some(Element::Token(Token::Keyword(Keyword::Const | Keyword::Let | Keyword::Var))) => {
                let decl = self.parse_variable_declaration()?;
                self.skip_non_tokens()?;
                self.expect_punctuator(Punctuator::Semi)?;
                Ok(Declaration::Variable(decl))
            }
            actual => Err(Error::unexpected(
                AnyOf(
                    Token::Keyword(Keyword::Const),
                    Token::Keyword(Keyword::Function),
                    vec![Token::Keyword(Keyword::Let), Token::Keyword(Keyword::Var)],
                ),
                actual.cloned(),
            )),
        }
    }

    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration> {
        self.expect_keyword(Keyword::Function)?;
        self.skip_non_tokens()?;
        let binding = self.expect_identifier(non_empty_str!("function_name"))?;
        self.skip_non_tokens()?;
        let formal_parameters = self.parse_fn_parameters()?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Require)?;
        Ok(FunctionDeclaration {
            binding,
            formal_parameters,
            body,
        })
    }

    pub(super) fn parse_fn_parameters(&mut self) -> Result<Vec<Identifier>> {
        self.expect_punctuator(Punctuator::OpenParen)?;
        self.skip_non_tokens()?;
        if self
            .source
            .next_if_eq(&Element::Token(Token::Punctuator(Punctuator::CloseParen)))?
            .is_some()
        {
            return Ok(vec![]);
        }

        let mut params = Vec::new();
        loop {
            self.skip_non_tokens()?;
            params.push(self.expect_identifier(non_empty_str!("parameter_name"))?);
            self.skip_non_tokens()?;
            match self.source.next()? {
                Some(Element::Token(Token::Punctuator(Punctuator::Comma))) => {}
                Some(Element::Token(Token::Punctuator(Punctuator::CloseParen))) => {
                    break Ok(params);
                }
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::CloseParen),
                            vec![],
                        ),
                        actual,
                    ));
                }
            }
        }
    }

    pub(super) fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let kind = match self.source.next()? {
            Some(Element::Token(Token::Keyword(Keyword::Const))) => VariableDeclarationKind::Const,
            Some(Element::Token(Token::Keyword(Keyword::Let))) => VariableDeclarationKind::Let,
            Some(Element::Token(Token::Keyword(Keyword::Var))) => VariableDeclarationKind::Var,
            actual => {
                return Err(Error::unexpected(
                    AnyOf(
                        Token::Keyword(Keyword::Const),
                        Token::Keyword(Keyword::Let),
                        vec![Token::Keyword(Keyword::Var)],
                    ),
                    actual,
                ))
            }
        };
        let mut bindings = Vec::new();
        loop {
            self.skip_non_tokens()?;
            bindings.push(self.parse_variable_binding()?);
            self.skip_non_tokens()?;

            match self.source.peek()? {
                Some(Element::Token(Token::Punctuator(Punctuator::Comma))) => {
                    self.source.next()?.unwrap();
                }
                Some(Element::Token(Token::Punctuator(Punctuator::Semi))) => break,
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::Semi),
                            vec![],
                        ),
                        actual.cloned(),
                    ))
                }
            }
        }
        Ok(VariableDeclaration { kind, bindings })
    }

    fn parse_variable_binding(&mut self) -> Result<VariableBinding> {
        let identifier = self.expect_identifier(non_empty_str!("variable_name"))?;
        self.skip_non_tokens()?;
        let initialiser = match self.source.peek()? {
            Some(Element::Token(Token::Punctuator(Punctuator::Eq))) => {
                self.source.next()?.unwrap();
                self.skip_non_tokens()?;
                Some(self.parse_expression()?)
            }
            Some(Element::Token(Token::Punctuator(Punctuator::Comma | Punctuator::Semi))) => None,
            actual => {
                return Err(Error::unexpected(
                    AnyOf(
                        Token::Punctuator(Punctuator::Eq),
                        Token::Punctuator(Punctuator::Comma),
                        vec![Token::Punctuator(Punctuator::Semi)],
                    ),
                    actual.cloned(),
                ))
            }
        };
        Ok(VariableBinding {
            identifier,
            initialiser,
        })
    }
}
