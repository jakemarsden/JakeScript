use super::block::Braces;
use super::error::AllowToken::{AnyOf, Exactly, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::{Keyword, Punctuator, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Token, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_statement(&mut self) -> Result<Statement> {
        match self.tokens.peek()? {
            Some(Token::Keyword(Keyword::If)) => self.parse_if_statement().map(Statement::If),
            Some(Token::Keyword(Keyword::Try)) => self.parse_try_statement().map(Statement::Try),
            Some(Token::Keyword(Keyword::For)) => self.parse_for_loop().map(Statement::ForLoop),
            Some(Token::Keyword(Keyword::While)) => {
                self.parse_while_loop().map(Statement::WhileLoop)
            }

            Some(token) => {
                let stmt = match token {
                    Token::Keyword(Keyword::Break) => {
                        self.parse_break_statement().map(Statement::Break)
                    }
                    Token::Keyword(Keyword::Continue) => {
                        self.parse_continue_statement().map(Statement::Continue)
                    }
                    Token::Keyword(Keyword::Return) => {
                        self.parse_return_statement().map(Statement::Return)
                    }
                    Token::Keyword(Keyword::Throw) => {
                        self.parse_throw_statement().map(Statement::Throw)
                    }
                    _ => self.parse_expression().map(|expr| {
                        Statement::Expression(ExpressionStatement { expression: expr })
                    }),
                }?;
                self.expect_punctuator(Punctuator::Semi)?;
                Ok(stmt)
            }
            None => Err(Error::unexpected_eoi(Unspecified)),
        }
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement> {
        self.expect_keyword(Keyword::If)?;
        self.expect_punctuator(Punctuator::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect_punctuator(Punctuator::CloseParen)?;
        let body = self.parse_block(Braces::Allow)?;
        let else_body = if self
            .tokens
            .next_if_eq(&Token::Keyword(Keyword::Else))?
            .is_some()
        {
            if matches!(self.tokens.peek()?, Some(Token::Keyword(Keyword::If))) {
                // Recursively parse `else if { .. }` blocks
                Some(Block::new(
                    vec![],
                    vec![BlockItem::Statement(Statement::If(
                        self.parse_if_statement()?,
                    ))],
                ))
            } else {
                // Parse `else { .. }` blocks
                Some(self.parse_block(Braces::Allow)?)
            }
        } else {
            None
        };
        Ok(IfStatement {
            condition,
            body,
            else_body,
        })
    }

    fn parse_for_loop(&mut self) -> Result<ForStatement> {
        self.expect_keyword(Keyword::For)?;
        self.expect_punctuator(Punctuator::OpenParen)?;

        let initialiser = match self.tokens.peek()? {
            Some(Token::Punctuator(Punctuator::Semi)) => None,
            _ => Some(self.parse_variable_declaration()?),
        };
        self.expect_punctuator(Punctuator::Semi)?;

        let condition = match self.tokens.peek()? {
            Some(Token::Punctuator(Punctuator::Semi)) => None,
            _ => Some(self.parse_expression()?),
        };
        self.expect_punctuator(Punctuator::Semi)?;

        let incrementor = match self.tokens.peek()? {
            Some(Token::Punctuator(Punctuator::CloseParen)) => None,
            _ => Some(self.parse_expression()?),
        };
        self.expect_punctuator(Punctuator::CloseParen)?;

        let body = self.parse_block(Braces::Allow)?;
        Ok(ForStatement {
            initialiser,
            condition,
            incrementor,
            body,
        })
    }

    fn parse_while_loop(&mut self) -> Result<WhileStatement> {
        self.expect_keyword(Keyword::While)?;
        self.expect_punctuator(Punctuator::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect_punctuator(Punctuator::CloseParen)?;
        let body = self.parse_block(Braces::Allow)?;
        Ok(WhileStatement { condition, body })
    }

    fn parse_break_statement(&mut self) -> Result<BreakStatement> {
        self.expect_keyword(Keyword::Break)?;
        Ok(BreakStatement {})
    }

    fn parse_continue_statement(&mut self) -> Result<ContinueStatement> {
        self.expect_keyword(Keyword::Continue)?;
        Ok(ContinueStatement {})
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement> {
        self.expect_keyword(Keyword::Return)?;
        let value = match self.tokens.peek()? {
            Some(Token::Punctuator(Punctuator::Semi)) => None,
            _ => Some(self.parse_expression()?),
        };
        Ok(ReturnStatement { value })
    }

    fn parse_throw_statement(&mut self) -> Result<ThrowStatement> {
        self.expect_keyword(Keyword::Throw)?;
        let exception = self.parse_expression()?;
        Ok(ThrowStatement { exception })
    }

    fn parse_try_statement(&mut self) -> Result<TryStatement> {
        self.expect_keyword(Keyword::Try)?;
        let body = self.parse_block(Braces::Require)?;
        let catch = if matches!(self.tokens.peek()?, Some(Token::Keyword(Keyword::Catch))) {
            Some(self.parse_catch_block()?)
        } else {
            None
        };
        let finally = if matches!(self.tokens.peek()?, Some(Token::Keyword(Keyword::Finally))) {
            self.expect_keyword(Keyword::Finally)?;
            Some(Finally {
                body: self.parse_block(Braces::Require)?,
            })
        } else {
            None
        };
        if catch.is_some() || finally.is_some() {
            Ok(TryStatement {
                body,
                catch,
                finally,
            })
        } else {
            Err(Error::unexpected(
                AnyOf(
                    Token::Keyword(Keyword::Catch),
                    Token::Keyword(Keyword::Finally),
                    vec![],
                ),
                self.tokens.peek()?.cloned(),
            ))
        }
    }

    fn parse_catch_block(&mut self) -> Result<Catch> {
        self.expect_keyword(Keyword::Catch)?;
        let parameter = if matches!(
            self.tokens.peek()?,
            Some(Token::Punctuator(Punctuator::OpenParen))
        ) {
            self.expect_punctuator(Punctuator::OpenParen)?;
            let identifier = match self.tokens.next()? {
                Some(Token::Identifier(id)) => Identifier::from(id),
                token => {
                    return Err(Error::unexpected(
                        Exactly(Token::Identifier(non_empty_str!("exception_identifier"))),
                        token,
                    ))
                }
            };
            self.expect_punctuator(Punctuator::CloseParen)?;
            Some(identifier)
        } else {
            None
        };
        let body = self.parse_block(Braces::Require)?;
        Ok(Catch { parameter, body })
    }
}
