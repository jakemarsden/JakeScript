use super::block::Braces;
use super::error::AllowToken::{AnyOf, Exactly};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::{Keyword, Punctuator, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Token, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration> {
        self.expect_keyword(Keyword::Function)?;
        let fn_name = Identifier::from(self.expect_identifier(non_empty_str!("function_name"))?);
        let param_names = self.parse_fn_parameters()?;
        let body = self.parse_block(Braces::Require)?;
        Ok(FunctionDeclaration {
            fn_name,
            param_names,
            body,
        })
    }

    pub(super) fn parse_fn_parameters(&mut self) -> Result<Vec<Identifier>> {
        self.expect_punctuator(Punctuator::OpenParen)?;
        if self
            .tokens
            .next_if_eq(&Token::Punctuator(Punctuator::CloseParen))?
            .is_some()
        {
            return Ok(Vec::with_capacity(0));
        }

        let mut params = Vec::new();
        loop {
            match self.tokens.next()? {
                Some(Token::Identifier(param)) => {
                    params.push(Identifier::from(param));
                }
                actual => {
                    return Err(Error::unexpected(
                        Exactly(Token::Identifier(non_empty_str!("function_parameter"))),
                        actual,
                    ));
                }
            }
            match self.tokens.next()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {}
                Some(Token::Punctuator(Punctuator::CloseParen)) => break Ok(params),
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::CloseParen),
                            vec![],
                        ),
                        actual,
                    ))
                }
            }
        }
    }

    pub(super) fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let kind = match self.tokens.next()? {
            Some(Token::Keyword(Keyword::Const)) => VariableDeclarationKind::Const,
            Some(Token::Keyword(Keyword::Let)) => VariableDeclarationKind::Let,
            Some(Token::Keyword(Keyword::Var)) => VariableDeclarationKind::Var,
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
        let mut entries = Vec::new();
        loop {
            entries.push(self.parse_variable_declaration_entry()?);

            match self.tokens.peek()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {
                    self.tokens.next().unwrap().unwrap();
                }
                Some(Token::Punctuator(Punctuator::Semi)) => break,
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
        Ok(VariableDeclaration { kind, entries })
    }

    fn parse_variable_declaration_entry(&mut self) -> Result<VariableDeclarationEntry> {
        let var_name = Identifier::from(self.expect_identifier(non_empty_str!("variable_name"))?);
        let initialiser = if let Some(Token::Punctuator(Punctuator::Eq)) = self.tokens.peek()? {
            self.tokens.next().unwrap().unwrap();
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(VariableDeclarationEntry {
            var_name,
            initialiser,
        })
    }
}
