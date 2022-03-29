use super::error::AllowToken::{Exactly, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::token::{Keyword, Punctuator, Token};
use fallible_iterator::FallibleIterator;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(super) enum Braces {
    Allow,
    Require,
}

impl<I: FallibleIterator<Item = Token, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_block(&mut self, braces: Braces) -> Result<Block> {
        match self.tokens.peek()? {
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
        Ok(match self.parse_declaration_or_statement()? {
            BlockItem::Declaration(decl) if decl.is_hoisted() => {
                let (decl, initialisers) = decl.into_declaration_and_initialiser();
                let initialisers = initialisers
                    .into_iter()
                    .map(|expr| BlockItem::from(Statement::Expression(expr)))
                    .collect();
                Block::new(vec![decl], initialisers)
            }
            node => Block::new(vec![], vec![node]),
        })
    }

    pub(super) fn parse_multi_statement_block_body(&mut self) -> Result<Block> {
        let mut hoisted_decls = Vec::new();
        let mut body = Vec::new();
        while !matches!(
            self.tokens.peek()?,
            Some(Token::Punctuator(Punctuator::CloseBrace)) | None
        ) {
            match self.parse_declaration_or_statement()? {
                BlockItem::Declaration(decl) if decl.is_hoisted() => {
                    let (decl, initialisers) = decl.into_declaration_and_initialiser();
                    let initialisers = initialisers
                        .into_iter()
                        .map(|expr| BlockItem::from(Statement::Expression(expr)));
                    hoisted_decls.push(decl);
                    body.extend(initialisers);
                }
                node => body.push(node),
            }
        }
        Ok(Block::new(hoisted_decls, body))
    }

    fn parse_declaration_or_statement(&mut self) -> Result<BlockItem> {
        match self.tokens.peek()? {
            Some(Token::Keyword(
                Keyword::Const | Keyword::Function | Keyword::Let | Keyword::Var,
            )) => self.parse_declaration().map(BlockItem::Declaration),
            _ => self.parse_statement().map(BlockItem::from),
        }
    }
}
