use super::error::AllowToken::{Exactly, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::token::{Element, Keyword, Punctuator, Token};
use fallible_iterator::FallibleIterator;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(super) enum Braces {
    Allow,
    Require,
}

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_block(&mut self, braces: Braces) -> Result<Block> {
        match self.source.peek()? {
            Some(Element::Token(Token::Punctuator(Punctuator::OpenBrace))) => match braces {
                Braces::Allow | Braces::Require => {
                    self.expect_punctuator(Punctuator::OpenBrace)?;
                    self.skip_non_tokens()?;
                    let block = self.parse_multi_statement_block_body()?;
                    self.skip_non_tokens()?;
                    self.expect_punctuator(Punctuator::CloseBrace)?;
                    Ok(block)
                }
            },
            Some(actual) => match braces {
                Braces::Allow => self.parse_single_statement_block_body(),
                Braces::Require => Err(Error::unexpected_token(
                    Exactly(Token::Punctuator(Punctuator::OpenBrace)),
                    actual.clone(),
                )),
            },
            None => match braces {
                Braces::Allow => Err(Error::unexpected_eoi(Unspecified)),
                Braces::Require => Err(Error::unexpected_eoi(Exactly(Token::Punctuator(
                    Punctuator::OpenBrace,
                )))),
            },
        }
    }

    fn parse_single_statement_block_body(&mut self) -> Result<Block> {
        Ok(match self.parse_declaration_or_statement()? {
            BlockItem::Declaration(decl) if decl.is_hoisted() => {
                let (decl, initialisers) = decl.into_declaration_and_initialiser();
                let initialisers = initialisers
                    .into_iter()
                    .map(|expr| {
                        BlockItem::Statement(Statement::Expression(ExpressionStatement {
                            expression: expr,
                        }))
                    })
                    .collect();
                Block::new(vec![decl], initialisers)
            }
            node => Block::new(vec![], vec![node]),
        })
    }

    pub(super) fn parse_multi_statement_block_body(&mut self) -> Result<Block> {
        let mut hoisted_decls = Vec::new();
        let mut body = Vec::new();
        loop {
            self.skip_non_tokens()?;
            if matches!(
                self.source.peek()?,
                Some(Element::Token(Token::Punctuator(Punctuator::CloseBrace))) | None
            ) {
                break;
            }
            match self.parse_declaration_or_statement()? {
                BlockItem::Declaration(decl) if decl.is_hoisted() => {
                    let (decl, initialisers) = decl.into_declaration_and_initialiser();
                    let initialisers = initialisers.into_iter().map(|expr| {
                        BlockItem::Statement(Statement::Expression(ExpressionStatement {
                            expression: expr,
                        }))
                    });
                    hoisted_decls.push(decl);
                    body.extend(initialisers);
                }
                node => body.push(node),
            }
        }
        Ok(Block::new(hoisted_decls, body))
    }

    fn parse_declaration_or_statement(&mut self) -> Result<BlockItem> {
        match self.source.peek()? {
            Some(Element::Token(Token::Keyword(
                Keyword::Const | Keyword::Function | Keyword::Let | Keyword::Var,
            ))) => self.parse_declaration().map(BlockItem::Declaration),
            _ => self.parse_statement().map(BlockItem::Statement),
        }
    }
}
