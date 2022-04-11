use super::block::Braces;
use super::error::AllowToken::{AnyOf, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::{self, *};
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::Keyword::{
    Break, Catch, Continue, Do, Else, Finally, For, If, Return, Throw, Try, While,
};
use crate::token::Punctuator::{CloseParen, OpenParen, Semi};
use crate::token::{Element, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_statement(&mut self) -> Result<Statement> {
        match self.source.peek()?.map(Element::keyword) {
            Some(Some(If)) => self.parse_if_statement().map(Statement::If),
            Some(Some(Try)) => self.parse_try_statement().map(Statement::Try),
            Some(Some(For)) => self.parse_for_loop().map(Statement::ForLoop),
            Some(Some(While)) => self.parse_while_loop().map(Statement::WhileLoop),
            Some(Some(Do)) => self.parse_do_while_loop().map(Statement::DoWhileLoop),

            Some(Some(Break)) => self.parse_break_statement().map(Statement::Break),
            Some(Some(Continue)) => self.parse_continue_statement().map(Statement::Continue),
            Some(Some(Return)) => self.parse_return_statement().map(Statement::Return),
            Some(Some(Throw)) => self.parse_throw_statement().map(Statement::Throw),
            Some(_) => self.parse_expression_statement().map(Statement::Expression),

            None => Err(Error::unexpected_eoi(Unspecified)),
        }
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement> {
        let loc = self.expect_keyword(If)?;
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
            .next_if(|elem| elem.keyword() == Some(Else))?
            .is_some()
        {
            self.skip_non_tokens()?;
            if let Some(elem) = self.source.peek()? && elem.keyword() == Some(If) {
                // Recursively parse `else if { .. }` blocks
                let else_if_stmt = self.parse_if_statement()?;
                let else_if_loc =else_if_stmt.source_location().clone();
                Some(Block::new(
                    vec![],
                    vec![BlockItem::Statement(Statement::If(else_if_stmt))],
                    else_if_loc,
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
            loc,
        })
    }

    fn parse_for_loop(&mut self) -> Result<ForStatement> {
        let loc = self.expect_keyword(For)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;

        let initialiser = match self.source.peek()? {
            Some(elem) if elem.punctuator() == Some(Semi) => None,
            _ => Some(self.parse_variable_declaration()?),
        };
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        self.skip_non_tokens()?;

        let condition = match self.source.peek()? {
            Some(elem) if elem.punctuator() == Some(Semi) => None,
            _ => Some(self.parse_expression()?),
        };
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        self.skip_non_tokens()?;

        let incrementor = match self.source.peek()? {
            Some(elem) if elem.punctuator() == Some(CloseParen) => None,
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
            loc,
        })
    }

    fn parse_while_loop(&mut self) -> Result<WhileStatement> {
        let loc = self.expect_keyword(While)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;
        let condition = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Allow)?;
        Ok(WhileStatement {
            condition,
            body,
            loc,
        })
    }

    fn parse_do_while_loop(&mut self) -> Result<DoWhileStatement> {
        let loc = self.expect_keyword(Do)?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Allow)?;
        self.skip_non_tokens()?;
        self.expect_keyword(While)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;
        let condition = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(DoWhileStatement {
            body,
            condition,
            loc,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement> {
        let expression = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(ExpressionStatement { expression })
    }

    fn parse_break_statement(&mut self) -> Result<BreakStatement> {
        let loc = self.expect_keyword(Break)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(BreakStatement { loc })
    }

    fn parse_continue_statement(&mut self) -> Result<ContinueStatement> {
        let loc = self.expect_keyword(Continue)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(ContinueStatement { loc })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement> {
        let loc = self.expect_keyword(Return)?;
        self.skip_non_tokens()?;
        let value = match self.source.peek()? {
            Some(elem) if elem.punctuator() == Some(Semi) => None,
            _ => Some(self.parse_expression()?),
        };
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(ReturnStatement { value, loc })
    }

    fn parse_throw_statement(&mut self) -> Result<ThrowStatement> {
        let loc = self.expect_keyword(Throw)?;
        self.skip_non_tokens()?;
        let exception = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(ThrowStatement { exception, loc })
    }

    fn parse_try_statement(&mut self) -> Result<TryStatement> {
        let loc = self.expect_keyword(Try)?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Require)?;

        self.skip_non_tokens()?;
        let catch = match self.source.peek()? {
            Some(elem) if elem.keyword() == Some(Catch) => Some(self.parse_catch()?),
            _ => None,
        };

        self.skip_non_tokens()?;
        let finally = match self.source.peek()? {
            Some(elem) if elem.keyword() == Some(Finally) => Some(self.parse_finally()?),
            _ => None,
        };

        if catch.is_some() || finally.is_some() {
            Ok(TryStatement {
                body,
                catch,
                finally,
                loc,
            })
        } else {
            Err(Error::unexpected(
                AnyOf(Token::Keyword(Catch), Token::Keyword(Finally), vec![]),
                self.source.peek()?.cloned(),
            ))
        }
    }

    fn parse_catch(&mut self) -> Result<ast::Catch> {
        let loc = self.expect_keyword(Catch)?;
        self.skip_non_tokens()?;
        let parameter = if self
            .source
            .next_if(|elem| elem.punctuator() == Some(OpenParen))?
            .is_some()
        {
            self.skip_non_tokens()?;
            let (parameter, _) = self.expect_identifier(non_empty_str!("catch_parameter"))?;
            self.skip_non_tokens()?;
            self.expect_punctuator(CloseParen)?;
            Some(parameter)
        } else {
            None
        };
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Require)?;
        Ok(ast::Catch {
            parameter,
            body,
            loc,
        })
    }

    fn parse_finally(&mut self) -> Result<ast::Finally> {
        let loc = self.expect_keyword(Finally)?;
        self.skip_non_tokens()?;
        let body = self.parse_block(Braces::Require)?;
        Ok(ast::Finally { body, loc })
    }
}
