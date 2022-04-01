use super::block::Braces;
use super::error::AllowToken::{AnyOf, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::{self, *};
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::Keyword::{
    Break, Catch, Continue, Else, Finally, For, If, Return, Throw, Try, While,
};
use crate::token::Punctuator::{CloseParen, OpenParen, Semi};
use crate::token::{Element, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_statement(&mut self) -> Result<Statement> {
        match self.source.peek()? {
            Some(Element::Token(Token::Keyword(If))) => {
                self.parse_if_statement().map(Statement::If)
            }
            Some(Element::Token(Token::Keyword(Try))) => {
                self.parse_try_statement().map(Statement::Try)
            }
            Some(Element::Token(Token::Keyword(For))) => {
                self.parse_for_loop().map(Statement::ForLoop)
            }
            Some(Element::Token(Token::Keyword(While))) => {
                self.parse_while_loop().map(Statement::WhileLoop)
            }

            Some(elem) => {
                let stmt = match elem {
                    Element::Token(Token::Keyword(Break)) => {
                        self.parse_break_statement().map(Statement::Break)
                    }
                    Element::Token(Token::Keyword(Continue)) => {
                        self.parse_continue_statement().map(Statement::Continue)
                    }
                    Element::Token(Token::Keyword(Return)) => {
                        self.parse_return_statement().map(Statement::Return)
                    }
                    Element::Token(Token::Keyword(Throw)) => {
                        self.parse_throw_statement().map(Statement::Throw)
                    }
                    _ => self.parse_expression().map(|expr| {
                        Statement::Expression(ExpressionStatement { expression: expr })
                    }),
                }?;
                self.skip_non_tokens()?;
                self.expect_punctuator(Semi)?;
                Ok(stmt)
            }
            None => Err(Error::unexpected_eoi(Unspecified)),
        }
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement> {
        self.expect_keyword(If)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;
        let condition = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Allow)?;
        self.skip_non_tokens()?;
        let else_body = if self
            .source
            .next_if_eq(&Element::Token(Token::Keyword(Else)))?
            .is_some()
        {
            self.skip_non_tokens()?;
            if matches!(
                self.source.peek()?,
                Some(Element::Token(Token::Keyword(If)))
            ) {
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
        self.expect_keyword(For)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;

        let initialiser = match self.source.peek()? {
            Some(Element::Token(Token::Punctuator(Semi))) => None,
            _ => Some(self.parse_variable_declaration()?),
        };
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        self.skip_non_tokens()?;

        let condition = match self.source.peek()? {
            Some(Element::Token(Token::Punctuator(Semi))) => None,
            _ => Some(self.parse_expression()?),
        };
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        self.skip_non_tokens()?;

        let incrementor = match self.source.peek()? {
            Some(Element::Token(Token::Punctuator(CloseParen))) => None,
            _ => Some(self.parse_expression()?),
        };
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        self.skip_non_tokens()?;

        let body = self.parse_block(Braces::Allow)?;
        Ok(ForStatement {
            initialiser,
            condition,
            incrementor,
            body,
        })
    }

    fn parse_while_loop(&mut self) -> Result<WhileStatement> {
        self.expect_keyword(While)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;
        let condition = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Allow)?;
        Ok(WhileStatement { condition, body })
    }

    fn parse_break_statement(&mut self) -> Result<BreakStatement> {
        self.expect_keyword(Break)?;
        Ok(BreakStatement {})
    }

    fn parse_continue_statement(&mut self) -> Result<ContinueStatement> {
        self.expect_keyword(Continue)?;
        Ok(ContinueStatement {})
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement> {
        self.expect_keyword(Return)?;
        self.skip_non_tokens()?;
        let value = match self.source.peek()? {
            Some(Element::Token(Token::Punctuator(Semi))) => None,
            _ => Some(self.parse_expression()?),
        };
        Ok(ReturnStatement { value })
    }

    fn parse_throw_statement(&mut self) -> Result<ThrowStatement> {
        self.expect_keyword(Throw)?;
        self.skip_non_tokens()?;
        let exception = self.parse_expression()?;
        Ok(ThrowStatement { exception })
    }

    fn parse_try_statement(&mut self) -> Result<TryStatement> {
        self.expect_keyword(Try)?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Require)?;

        self.skip_non_tokens()?;
        let catch = if matches!(
            self.source.peek()?,
            Some(Element::Token(Token::Keyword(Catch)))
        ) {
            Some(self.parse_catch()?)
        } else {
            None
        };

        self.skip_non_tokens()?;
        let finally = if matches!(
            self.source.peek()?,
            Some(Element::Token(Token::Keyword(Finally)))
        ) {
            Some(self.parse_finally()?)
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
                AnyOf(Token::Keyword(Catch), Token::Keyword(Finally), vec![]),
                self.source.peek()?.cloned(),
            ))
        }
    }

    fn parse_catch(&mut self) -> Result<ast::Catch> {
        self.expect_keyword(Catch)?;
        self.skip_non_tokens()?;
        let parameter = if self
            .source
            .next_if_eq(&Element::Token(Token::Punctuator(OpenParen)))?
            .is_some()
        {
            self.skip_non_tokens()?;
            let parameter = self.expect_identifier(non_empty_str!("catch_parameter"))?;
            self.skip_non_tokens()?;
            self.expect_punctuator(CloseParen)?;
            Some(parameter)
        } else {
            None
        };
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Require)?;
        Ok(ast::Catch { parameter, body })
    }

    fn parse_finally(&mut self) -> Result<ast::Finally> {
        self.expect_keyword(Finally)?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Require)?;
        Ok(ast::Finally { body })
    }
}
