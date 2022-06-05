use super::error::AllowToken::{AnyOf, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::{self, *};
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::Keyword::{
    Break, Case, Catch, Const, Continue, Default, Do, Else, Finally, For, Function, If, Let,
    Return, Switch, Throw, Try, Var, While,
};
use crate::token::Punctuator::{CloseBrace, CloseParen, Colon, OpenBrace, OpenParen, Semi};
use crate::token::{Element, Token};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_statement(&mut self) -> Result<Statement> {
        match self.source.peek()? {
            Some(elem) if let Some(punc) = elem.punctuator() => match punc {
                Semi => self.parse_empty_statement().map(Statement::Empty),
                OpenBrace => self.parse_block_statement().map(Statement::Block),
                _ => self.parse_expression_statement().map(Statement::Expression),
            }

            Some(elem) if let Some(kw) = elem.keyword() => match kw {
                Const | Function | Let | Var => {
                    self.parse_declaration_statement().map(Statement::Declaration)
                }

                If => self.parse_if_statement().map(Statement::If),
                Switch => self.parse_switch_statement().map(Statement::Switch),
                Do => self.parse_do_statement().map(Statement::Do),
                While => self.parse_while_statement().map(Statement::While),
                For => self.parse_for_statement().map(Statement::For),

                Continue => self.parse_continue_statement().map(Statement::Continue),
                Break => self.parse_break_statement().map(Statement::Break),
                Return => self.parse_return_statement().map(Statement::Return),
                Throw => self.parse_throw_statement().map(Statement::Throw),
                Try => self.parse_try_statement().map(Statement::Try),

                _ => self.parse_expression_statement().map(Statement::Expression),
            }

            Some(_) => self.parse_expression_statement().map(Statement::Expression),
            None => Err(Error::unexpected_eoi(Unspecified)),
        }
    }

    fn parse_empty_statement(&mut self) -> Result<EmptyStatement> {
        let loc = self.expect_punctuator(Semi)?;
        Ok(EmptyStatement { loc })
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement> {
        let block = self.parse_block()?;
        Ok(BlockStatement { block })
    }

    fn parse_declaration_statement(&mut self) -> Result<DeclarationStatement> {
        let declaration = self.parse_declaration()?;
        Ok(DeclarationStatement { declaration })
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement> {
        let expression = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(ExpressionStatement { expression })
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
        let body = self.parse_statement()?;
        self.skip_non_tokens()?;
        let else_body = if self
            .source
            .next_if(|elem| elem.keyword() == Some(Else))?
            .is_some()
        {
            self.skip_non_tokens()?;
            Some(self.parse_statement()?)
        } else {
            None
        };
        Ok(IfStatement {
            loc,
            condition,
            body: Box::new(body),
            else_body: else_body.map(Box::new),
        })
    }

    fn parse_switch_statement(&mut self) -> Result<SwitchStatement> {
        let loc = self.expect_keyword(Switch)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;
        let value = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenBrace)?;
        let mut cases = Vec::new();
        let mut default_case = None;
        loop {
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(elem) if elem.keyword() == Some(Case) => {
                    cases.push(self.parse_case_statement()?);
                }
                Some(elem) if elem.keyword() == Some(Default) => {
                    default_case = Some(self.parse_default_case_statement()?);
                    self.skip_non_tokens()?;
                    break;
                }
                Some(elem) if elem.punctuator() == Some(CloseBrace) => {
                    break;
                }
                elem => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Keyword(Case),
                            Token::Keyword(Default),
                            vec![Token::Punctuator(CloseBrace)],
                        ),
                        elem.cloned(),
                    ))
                }
            }
        }
        self.expect_punctuator(CloseBrace)?;
        Ok(SwitchStatement {
            loc,
            value,
            cases,
            default_case,
        })
    }

    fn parse_case_statement(&mut self) -> Result<CaseStatement> {
        let loc = self.expect_keyword(Case)?;
        self.skip_non_tokens()?;
        let expected = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Colon)?;
        self.skip_non_tokens()?;
        let body = self.parse_case_statement_body()?;
        Ok(CaseStatement {
            loc,
            expected,
            body,
        })
    }

    fn parse_default_case_statement(&mut self) -> Result<DefaultCaseStatement> {
        let loc = self.expect_keyword(Default)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Colon)?;
        self.skip_non_tokens()?;
        let body = self.parse_case_statement_body()?;
        Ok(DefaultCaseStatement { loc, body })
    }

    fn parse_case_statement_body(&mut self) -> Result<Vec<Statement>> {
        let mut stmts = Vec::new();
        loop {
            match self.source.peek()? {
                Some(elem) if matches!(elem.keyword(), Some(Case | Default)) => break,
                Some(elem) if elem.punctuator() == Some(CloseBrace) => break,
                None => break,
                _ => {
                    stmts.push(self.parse_statement()?);
                    self.skip_non_tokens()?;
                }
            }
        }
        Ok(stmts)
    }

    fn parse_do_statement(&mut self) -> Result<DoStatement> {
        let loc = self.expect_keyword(Do)?;
        self.skip_non_tokens()?;
        let body = self.parse_statement()?;
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
        Ok(DoStatement {
            loc,
            body: Box::new(body),
            condition,
        })
    }

    fn parse_while_statement(&mut self) -> Result<WhileStatement> {
        let loc = self.expect_keyword(While)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;
        let condition = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        self.skip_non_tokens()?;
        let body = self.parse_statement()?;
        Ok(WhileStatement {
            loc,
            condition,
            body: Box::new(body),
        })
    }

    fn parse_for_statement(&mut self) -> Result<ForStatement> {
        let loc = self.expect_keyword(For)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(OpenParen)?;
        self.skip_non_tokens()?;

        let initialiser = match self.source.peek()? {
            Some(elem) if elem.punctuator() == Some(Semi) => None,
            _ => Some(self.parse_loop_initialiser()?),
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

        let body = self.parse_statement()?;
        Ok(ForStatement {
            loc,
            initialiser,
            condition,
            incrementor,
            body: Box::new(body),
        })
    }

    fn parse_loop_initialiser(&mut self) -> Result<LoopInitialiser> {
        Ok(match self.source.peek()? {
            Some(elem) if matches!(elem.keyword(), Some(Const | Let | Var)) => self
                .parse_variable_declaration()
                .map(LoopInitialiser::VariableDeclaration)?,
            Some(_) => self.parse_expression().map(LoopInitialiser::Expression)?,
            None => return Err(Error::unexpected_eoi(Unspecified)),
        })
    }

    fn parse_continue_statement(&mut self) -> Result<ContinueStatement> {
        let loc = self.expect_keyword(Continue)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(ContinueStatement { loc })
    }

    fn parse_break_statement(&mut self) -> Result<BreakStatement> {
        let loc = self.expect_keyword(Break)?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(BreakStatement { loc })
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
        Ok(ReturnStatement { loc, value })
    }

    fn parse_throw_statement(&mut self) -> Result<ThrowStatement> {
        let loc = self.expect_keyword(Throw)?;
        self.skip_non_tokens()?;
        let exception = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Semi)?;
        Ok(ThrowStatement { loc, exception })
    }

    fn parse_try_statement(&mut self) -> Result<TryStatement> {
        let loc = self.expect_keyword(Try)?;
        self.skip_non_tokens()?;
        let body = self.parse_block()?;

        self.skip_non_tokens()?;
        let catch = match self.source.peek()? {
            Some(elem) if elem.keyword() == Some(Catch) => Some(self.parse_catch_statement()?),
            _ => None,
        };

        self.skip_non_tokens()?;
        let finally = match self.source.peek()? {
            Some(elem) if elem.keyword() == Some(Finally) => Some(self.parse_finally_statement()?),
            _ => None,
        };

        if catch.is_some() || finally.is_some() {
            Ok(TryStatement {
                loc,
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

    fn parse_catch_statement(&mut self) -> Result<ast::CatchStatement> {
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
        let body = self.parse_block()?;
        Ok(ast::CatchStatement {
            loc,
            parameter,
            body,
        })
    }

    fn parse_finally_statement(&mut self) -> Result<ast::FinallyStatement> {
        let loc = self.expect_keyword(Finally)?;
        self.skip_non_tokens()?;
        let body = self.parse_block()?;
        Ok(ast::FinallyStatement { loc, body })
    }
}
