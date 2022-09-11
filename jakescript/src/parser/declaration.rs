use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::parser::Expected;
use crate::token::Keyword::{Const, Function, Let, Var};
use crate::token::Punctuator::{CloseParen, Comma, Eq, OpenParen, Semi};
use crate::token::{Element, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_declaration(&mut self) -> Result<Declaration> {
        let elem = self.source.peek()?;
        match elem.and_then(Element::token) {
            Some(Token::Keyword(Function)) => {
                self.parse_function_declaration().map(Declaration::Function)
            }
            Some(Token::Keyword(Var)) => {
                let decl = self.parse_variable_declaration()?;
                self.skip_non_tokens()?;
                self.expect_punctuator(Semi)?;
                Ok(Declaration::Variable(decl))
            }
            Some(Token::Keyword(Const | Let)) => {
                let decl = self.parse_lexical_declaration()?;
                self.skip_non_tokens()?;
                self.expect_punctuator(Semi)?;
                Ok(Declaration::Lexical(decl))
            }
            _ => Err(Error::unexpected(
                (
                    Expected::Keyword(Const),
                    Expected::Keyword(Function),
                    Expected::Keyword(Let),
                    Expected::Keyword(Var),
                ),
                elem.cloned(),
            )),
        }
    }

    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration> {
        let loc = self.expect_keyword(Function)?;
        self.skip_non_tokens()?;
        let (binding, _) = self.expect_identifier("function_name")?;
        self.skip_non_tokens()?;
        let formal_parameters = self.parse_fn_parameters()?;
        self.skip_non_tokens()?;
        let body = self.parse_block()?;
        Ok(FunctionDeclaration {
            loc,
            binding,
            formal_parameters,
            body,
        })
    }

    pub(super) fn parse_fn_parameters(&mut self) -> Result<Vec<Identifier>> {
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;
        if self
            .source
            .next_if(|elem| elem.punctuator() == Some(CloseParen))?
            .is_some()
        {
            return Ok(vec![]);
        }

        let mut params = Vec::new();
        loop {
            self.skip_non_tokens()?;
            let (param, _) = self.expect_identifier("parameter_name")?;
            params.push(param);
            self.skip_non_tokens()?;
            match self.source.next()? {
                Some(elem) if elem.punctuator() == Some(Comma) => {}
                Some(elem) if elem.punctuator() == Some(CloseParen) => {
                    break Ok(params);
                }
                elem => {
                    return Err(Error::unexpected(
                        (
                            Expected::Punctuator(Comma),
                            Expected::Punctuator(CloseParen),
                        ),
                        elem,
                    ));
                }
            }
        }
    }

    pub(super) fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let loc = self.expect_keyword(Var)?;
        self.skip_non_tokens()?;
        let bindings = self.parse_variable_bindings()?;
        Ok(VariableDeclaration { loc, bindings })
    }

    pub(super) fn parse_lexical_declaration(&mut self) -> Result<LexicalDeclaration> {
        let (kind, loc) = match self.source.next()? {
            Some(elem) if elem.keyword() == Some(Const) => (
                LexicalDeclarationKind::Const,
                elem.source_location().clone(),
            ),
            Some(elem) if elem.keyword() == Some(Let) => {
                (LexicalDeclarationKind::Let, elem.source_location().clone())
            }
            elem => {
                return Err(Error::unexpected(
                    (Expected::Keyword(Const), Expected::Keyword(Let)),
                    elem,
                ));
            }
        };
        self.skip_non_tokens()?;
        let bindings = self.parse_variable_bindings()?;
        Ok(LexicalDeclaration {
            loc,
            kind,
            bindings,
        })
    }

    fn parse_variable_bindings(&mut self) -> Result<Vec<VariableBinding>> {
        let mut bindings = Vec::new();
        loop {
            self.skip_non_tokens()?;
            bindings.push(self.parse_variable_binding()?);
            self.skip_non_tokens()?;

            match self.source.peek()? {
                Some(elem) if elem.punctuator() == Some(Comma) => {
                    self.source.next()?.unwrap();
                }
                Some(elem) if elem.punctuator() == Some(Semi) => break Ok(bindings),
                elem => {
                    return Err(Error::unexpected(
                        (Expected::Punctuator(Comma), Expected::Punctuator(Semi)),
                        elem.cloned(),
                    ))
                }
            }
        }
    }

    fn parse_variable_binding(&mut self) -> Result<VariableBinding> {
        let (identifier, loc) = self.expect_identifier("variable_name")?;
        self.skip_non_tokens()?;
        let initialiser = match self.source.peek()? {
            Some(elem) if elem.punctuator() == Some(Eq) => {
                self.source.next()?.unwrap();
                self.skip_non_tokens()?;
                Some(self.parse_expression()?)
            }
            Some(elem) if matches!(elem.punctuator(), Some(Comma | Semi)) => None,
            elem => {
                return Err(Error::unexpected(
                    (
                        Expected::Punctuator(Eq),
                        Expected::Punctuator(Comma),
                        Expected::Punctuator(Semi),
                    ),
                    elem.cloned(),
                ))
            }
        };
        Ok(VariableBinding {
            loc,
            identifier,
            initialiser,
        })
    }
}
