use super::error::AllowToken::{Exactly, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::lexer;
use crate::token::{Punctuator, Token};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(super) enum Braces {
    Allow,
    Require,
}

impl<I: Iterator<Item = lexer::Result<Token>>> Parser<I> {
    pub(super) fn parse_block(&mut self, braces: Braces) -> Result<Block> {
        match self.tokens.try_peek()? {
            Some(Token::Punctuator(Punctuator::OpenBrace)) => match braces {
                Braces::Allow | Braces::Require => {
                    self.expect_punctuator(Punctuator::OpenBrace)?;
                    let block = self.parse_multi_statement_block_body();
                    self.expect_punctuator(Punctuator::CloseBrace)?;
                    block
                }
            },
            Some(actual) => match braces {
                Braces::Allow => self.parse_single_statement_block_body(),
                Braces::Require => Err(Error::unexpected_token(
                    Exactly(Token::Punctuator(Punctuator::OpenBrace)),
                    actual.clone(),
                )),
            },
            None => Err(Error::unexpected_eoi(Unspecified)),
        }
    }

    fn parse_single_statement_block_body(&mut self) -> Result<Block> {
        Ok(match self.parse_statement()? {
            Statement::Declaration(decl) if decl.is_hoisted() => match decl {
                decl_stmt @ DeclarationStatement::Function(_) => {
                    Block::new(vec![decl_stmt], vec![])
                }
                DeclarationStatement::Variable(var_decl) => {
                    let (var_decl, initialisers) = var_decl.into_declaration_and_initialiser();
                    let decl_stmt = DeclarationStatement::Variable(var_decl);
                    let init_stmts = initialisers
                        .into_iter()
                        .map(Statement::Expression)
                        .collect();
                    Block::new(vec![decl_stmt], init_stmts)
                }
            },
            stmt => Block::new(vec![], vec![stmt]),
        })
    }

    pub(super) fn parse_multi_statement_block_body(&mut self) -> Result<Block> {
        let mut hoisted_decls = Vec::new();
        let mut stmts = Vec::new();
        while !matches!(
            self.tokens.try_peek()?,
            Some(Token::Punctuator(Punctuator::CloseBrace)) | None
        ) {
            match self.parse_statement()? {
                Statement::Declaration(decl) if decl.is_hoisted() => match decl {
                    decl_stmt @ DeclarationStatement::Function(_) => {
                        hoisted_decls.push(decl_stmt);
                    }
                    DeclarationStatement::Variable(var_decl) => {
                        let (var_decl, initialisers) = var_decl.into_declaration_and_initialiser();
                        stmts.extend(initialisers.into_iter().map(Statement::Expression));
                        hoisted_decls.push(DeclarationStatement::Variable(var_decl));
                    }
                },
                stmt => stmts.push(stmt),
            }
        }
        Ok(Block::new(hoisted_decls, stmts))
    }
}