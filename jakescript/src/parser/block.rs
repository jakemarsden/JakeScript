use super::error::AllowToken::{Exactly, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::token::Keyword::{Const, Function, Let, Var};
use crate::token::Punctuator::{CloseBrace, OpenBrace};
use crate::token::{Element, SourceLocation, Token};
use fallible_iterator::FallibleIterator;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(super) enum Braces {
    Allow,
    Require,
}

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_block(&mut self, braces: Braces) -> Result<Block> {
        match self.source.peek()? {
            Some(elem) if elem.punctuator() == Some(OpenBrace) => match braces {
                Braces::Allow | Braces::Require => {
                    let loc = self.expect_punctuator(OpenBrace)?;
                    self.skip_non_tokens()?;
                    let block = self.parse_multi_statement_block_body(loc)?;
                    self.skip_non_tokens()?;
                    self.expect_punctuator(CloseBrace)?;
                    Ok(block)
                }
            },
            Some(elem) => match braces {
                Braces::Allow => self.parse_single_statement_block_body(),
                Braces::Require => Err(Error::unexpected_token(
                    Exactly(Token::Punctuator(OpenBrace)),
                    elem.clone(),
                )),
            },
            None => Err(Error::unexpected_eoi(match braces {
                Braces::Allow => Unspecified,
                Braces::Require => Exactly(Token::Punctuator(OpenBrace)),
            })),
        }
    }

    fn parse_single_statement_block_body(&mut self) -> Result<Block> {
        Ok(match self.parse_declaration_or_statement()? {
            BlockItem::Declaration(decl) if decl.is_hoisted() => {
                let (decl, initialisers) = decl.into_declaration_and_initialiser();
                let loc = decl.source_location().clone();
                let initialisers = initialisers
                    .into_iter()
                    .map(|expr| {
                        BlockItem::Statement(Statement::Expression(ExpressionStatement {
                            expression: expr,
                        }))
                    })
                    .collect();
                Block::new(vec![decl], initialisers, loc)
            }
            node => {
                let loc = node.source_location().clone();
                Block::new(vec![], vec![node], loc)
            }
        })
    }

    /// - `loc` - Location of the opening brace.
    pub(super) fn parse_multi_statement_block_body(
        &mut self,
        loc: SourceLocation,
    ) -> Result<Block> {
        let mut hoisted_decls = Vec::new();
        let mut body = Vec::new();
        loop {
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(elem) if elem.punctuator() == Some(CloseBrace) => break,
                None => break,
                _ => {}
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
        Ok(Block::new(hoisted_decls, body, loc))
    }

    fn parse_declaration_or_statement(&mut self) -> Result<BlockItem> {
        if let Some(elem) = self.source.peek()?
            && matches!(elem.keyword(), Some(Const | Function | Let | Var))
        {
            self.parse_declaration().map(BlockItem::Declaration)
        } else {
            self.parse_statement().map(BlockItem::Statement)
        }
    }
}
