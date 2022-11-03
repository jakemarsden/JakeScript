use super::error::Result;
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::token::Punctuator::{CloseBrace, OpenBrace};
use crate::token::{Element, SourceLocation};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_block(&mut self) -> Result<(SourceLocation, Block)> {
        let loc = self.expect_punctuator(OpenBrace)?;
        self.skip_non_tokens()?;
        let block = self.parse_block_body()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseBrace)?;
        Ok((loc, block))
    }

    /// - `loc` - Location of the opening brace.
    pub(super) fn parse_block_body(&mut self) -> Result<Block> {
        let mut hoisted_decls = Vec::new();
        let mut body = Vec::new();
        loop {
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(elem) if elem.punctuator() == Some(CloseBrace) => break,
                None => break,
                _ => {}
            }
            match self.parse_statement()? {
                Statement::Declaration(decl) if decl.is_hoisted() => {
                    let (decl, init_exprs) = decl.into_declaration_and_initialiser();
                    hoisted_decls.push(decl);
                    body.reserve(init_exprs.len());
                    body.extend(init_exprs.into_iter().map(Statement::Expression));
                }
                node => body.push(node),
            };
        }
        Ok(Block::new(hoisted_decls, body))
    }
}
