use crate::ast::{
    self, AssertStatement, AssignmentExpression, AssignmentOperator, BinaryExpression,
    BinaryOperator, Block, BreakStatement, CatchBlock, ComputedPropertyAccessExpression,
    ComputedPropertyAccessOperator, ContinueStatement, DeclarationStatement, Expression,
    FinallyBlock, ForLoop, FunctionCallExpression, FunctionCallOperator, FunctionDeclaration,
    GroupingExpression, GroupingOperator, Identifier, IfStatement, LiteralExpression, Op, Operator,
    Precedence, Program, PropertyAccessExpression, PropertyAccessOperator, ReturnStatement,
    Statement, TernaryExpression, TernaryOperator, ThrowStatement, TryStatement, UnaryExpression,
    UnaryOperator, VariableAccessExpression, VariableDeclaration, VariableDeclarationEntry,
    VariableDeclarationKind, WhileLoop,
};
use crate::iter::{IntoPeekableNth, PeekableNth};
use crate::lexer::{
    self, Keyword, Lexer, Literal, NumericLiteral, Punctuator, StringLiteral, Token, Tokens,
};
use crate::non_empty_str;
use crate::str::NonEmptyString;
use error::AllowToken::{AnyOf, Exactly, Unspecified};
use std::collections::HashMap;
use std::io;
use std::iter::Map;

pub use error::*;

mod error;

type Fallible<I> = Map<I, fn(Token) -> lexer::Result<Token>>;

pub struct Parser<I: Iterator<Item = lexer::Result<Token>>> {
    tokens: PeekableNth<I>,
}

impl<I: Iterator<Item = io::Result<char>>> Parser<Tokens<Lexer<I>>> {
    pub fn for_lexer(source: Lexer<I>) -> Self {
        Self::for_tokens_fallible(source.tokens())
    }
}

impl<I: Iterator<Item = Token>> Parser<Fallible<I>> {
    pub fn for_tokens(source: I) -> Self {
        Self::for_tokens_fallible(source.map(Ok))
    }
}

impl<I: Iterator<Item = lexer::Result<Token>>> Parser<I> {
    pub fn for_tokens_fallible(source: I) -> Self {
        Self {
            tokens: source.peekable_nth(),
        }
    }

    pub fn execute(mut self) -> Result {
        let body = self.parse_block(false)?;
        Ok(Program::new(body))
    }

    fn parse_block(&mut self, braces: bool) -> Result<Block> {
        if braces {
            self.expect_punctuator(Punctuator::OpenBrace)?;
        }
        let mut hoisted_decls = Vec::new();
        let mut stmts = Vec::new();
        loop {
            match self.tokens.try_peek()? {
                Some(Token::Punctuator(Punctuator::CloseBrace)) if braces => break,
                Some(_) => {}
                None if !braces => break,
                None => {
                    return Err(Error::unexpected_eoi(Exactly(Token::Punctuator(
                        Punctuator::CloseBrace,
                    ))))
                }
            }
            match self.parse_statement()? {
                Statement::Declaration(decl) if decl.is_hoisted() => match decl {
                    DeclarationStatement::Function(fn_decl) => {
                        hoisted_decls.push(DeclarationStatement::Function(fn_decl));
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
        if braces {
            self.expect_punctuator(Punctuator::CloseBrace)?;
        }
        Ok(Block::new(hoisted_decls, stmts))
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.tokens.try_peek()? {
            Some(Token::Keyword(Keyword::If)) => self.parse_if_statement().map(Statement::If),
            Some(Token::Keyword(Keyword::Function)) => self
                .parse_function_declaration()
                .map(DeclarationStatement::Function)
                .map(Statement::Declaration),
            Some(Token::Keyword(Keyword::Try)) => self.parse_try_statement().map(Statement::Try),
            Some(Token::Keyword(Keyword::For)) => self.parse_for_loop().map(Statement::ForLoop),
            Some(Token::Keyword(Keyword::While)) => {
                self.parse_while_loop().map(Statement::WhileLoop)
            }

            Some(token) => {
                let stmt = match token {
                    Token::Keyword(Keyword::Assert) => {
                        self.parse_assert_statement().map(Statement::Assert)
                    }
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
                    Token::Keyword(Keyword::Const | Keyword::Let | Keyword::Var) => self
                        .parse_variable_declaration()
                        .map(DeclarationStatement::Variable)
                        .map(Statement::Declaration),
                    _ => self.parse_expression().map(Statement::Expression),
                }?;
                self.expect_punctuator(Punctuator::Semicolon)?;
                Ok(stmt)
            }
            None => Err(Error::unexpected_eoi(Unspecified)),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_expression_impl(Precedence::MIN)
    }

    fn parse_expression_impl(&mut self, min_precedence: Precedence) -> Result<Expression> {
        let mut expression = self.parse_primary_expression()?;
        while let Some(&Token::Punctuator(punctuator)) = self.tokens.try_peek()? {
            if let Some(op_kind) = Operator::try_parse(punctuator, Position::PostfixOrInfix) {
                if op_kind.precedence() > min_precedence {
                    self.tokens.try_next().unwrap().unwrap();
                    expression = self.parse_secondary_expression(expression, op_kind)?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression> {
        Ok(match self.tokens.try_next()? {
            Some(Token::Identifier(var_name)) => {
                Expression::VariableAccess(VariableAccessExpression {
                    var_name: Identifier::from(var_name),
                })
            }
            Some(Token::Punctuator(Punctuator::OpenBracket)) => {
                let elems = self.parse_array_literal_elements()?;
                self.expect_punctuator(Punctuator::CloseBracket)?;
                Expression::Literal(LiteralExpression {
                    value: ast::Literal::Array(elems),
                })
            }
            Some(Token::Punctuator(Punctuator::OpenBrace)) => {
                let props = self.parse_object_properties()?;
                self.expect_punctuator(Punctuator::CloseBrace)?;
                Expression::Literal(LiteralExpression {
                    value: ast::Literal::Object(props),
                })
            }
            Some(Token::Punctuator(punc)) => {
                if let Some(op_kind) = UnaryOperator::try_parse(punc, Position::Prefix) {
                    let operand = self.parse_expression_impl(op_kind.precedence())?;
                    Expression::Unary(UnaryExpression {
                        op: op_kind,
                        operand: Box::new(operand),
                    })
                } else if GroupingOperator::try_parse(punc, Position::Prefix).is_some() {
                    let inner = self.parse_expression()?;
                    self.expect_punctuator(Punctuator::CloseParen)?;
                    Expression::Grouping(GroupingExpression {
                        inner: Box::new(inner),
                    })
                } else {
                    return Err(Error::unexpected_token(
                        Unspecified,
                        Token::Punctuator(punc),
                    ));
                }
            }
            Some(Token::Literal(literal)) => Expression::Literal(LiteralExpression {
                value: match literal {
                    Literal::Boolean(value) => ast::Literal::Boolean(value),
                    Literal::Numeric(
                        NumericLiteral::BinInt(value)
                        | NumericLiteral::OctInt(value)
                        | NumericLiteral::DecInt(value)
                        | NumericLiteral::HexInt(value),
                    ) => ast::Literal::Numeric(ast::NumericLiteral::Int(value)),
                    Literal::Numeric(NumericLiteral::Decimal(value)) => {
                        todo!("NumericLiteral::Decimal: {}", value)
                    }
                    Literal::String(
                        StringLiteral::SingleQuoted(value) | StringLiteral::DoubleQuoted(value),
                    ) => ast::Literal::String(value),
                    Literal::RegEx(value) => {
                        // FIXME: Support Literal::RegEx properly"
                        ast::Literal::String(value.to_string())
                    }
                    Literal::Null => ast::Literal::Null,
                },
            }),
            Some(Token::Keyword(Keyword::Function)) => {
                let name = match self.tokens.try_peek()? {
                    Some(Token::Identifier(_)) => {
                        let name = self
                            .expect_identifier(non_empty_str!("function_name"))
                            .unwrap();
                        Some(Identifier::from(name))
                    }
                    Some(Token::Punctuator(Punctuator::OpenParen)) => None,
                    token => {
                        return Err(Error::unexpected(
                            AnyOf(
                                Token::Punctuator(Punctuator::OpenParen),
                                Token::Identifier(non_empty_str!("function_name")),
                                vec![],
                            ),
                            token.cloned(),
                        ))
                    }
                };
                let param_names = self.parse_fn_parameters()?;
                let body = self.parse_block(true)?;
                Expression::Literal(LiteralExpression {
                    value: ast::Literal::Function {
                        name,
                        param_names,
                        body,
                    },
                })
            }
            actual => return Err(Error::unexpected(Unspecified, actual)),
        })
    }

    fn parse_secondary_expression(
        &mut self,
        lhs: Expression,
        op_kind: Operator,
    ) -> Result<Expression> {
        Ok(match op_kind {
            Operator::Assignment(kind) => Expression::Assignment(AssignmentExpression {
                op: kind,
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_expression_impl(op_kind.precedence())?),
            }),
            Operator::Binary(kind) => Expression::Binary(BinaryExpression {
                op: kind,
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_expression_impl(op_kind.precedence())?),
            }),
            Operator::Unary(kind) => Expression::Unary(UnaryExpression {
                op: kind,
                operand: Box::new(lhs),
            }),
            Operator::Ternary => {
                let condition = lhs;
                let lhs = self.parse_expression_impl(op_kind.precedence())?;
                self.expect_punctuator(Punctuator::Colon)?;
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Ternary(TernaryExpression {
                    condition: Box::new(condition),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            Operator::Grouping => Expression::Grouping(GroupingExpression {
                inner: Box::new(lhs),
            }),
            Operator::FunctionCall => Expression::FunctionCall(FunctionCallExpression {
                function: Box::new(lhs),
                arguments: self.parse_fn_arguments(false)?,
            }),
            Operator::PropertyAccess => {
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::PropertyAccess(PropertyAccessExpression {
                    base: Box::new(lhs),
                    property_name: match rhs {
                        Expression::VariableAccess(VariableAccessExpression { var_name }) => {
                            var_name
                        }
                        rhs_expr => todo!(
                            "Unsupported property access expression (only simple `a.b` \
                             expressions are currently supported): {:?}",
                            rhs_expr
                        ),
                    },
                })
            }
            Operator::ComputedPropertyAccess => {
                let rhs = self.parse_expression()?;
                self.expect_punctuator(Punctuator::CloseBracket)?;
                Expression::ComputedPropertyAccess(ComputedPropertyAccessExpression {
                    base: Box::new(lhs),
                    property: Box::new(rhs),
                })
            }
        })
    }

    fn parse_array_literal_elements(&mut self) -> Result<Vec<Expression>> {
        if matches!(
            self.tokens.try_peek()?,
            Some(Token::Punctuator(Punctuator::CloseBracket))
        ) {
            return Ok(Vec::with_capacity(0));
        }
        let mut elems = Vec::new();
        loop {
            elems.push(self.parse_expression()?);
            match self.tokens.try_peek()? {
                Some(&Token::Punctuator(Punctuator::Comma)) => {
                    self.tokens
                        .try_next_exact(&Token::Punctuator(Punctuator::Comma))?;
                }
                Some(&Token::Punctuator(Punctuator::CloseBracket)) => break Ok(elems),
                token => {
                    break Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::CloseBracket),
                            vec![],
                        ),
                        token.cloned(),
                    ))
                }
            }
        }
    }

    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration> {
        self.expect_keyword(Keyword::Function)?;
        let fn_name = Identifier::from(self.expect_identifier(non_empty_str!("function_name"))?);
        let param_names = self.parse_fn_parameters()?;
        let body = self.parse_block(true)?;
        Ok(FunctionDeclaration {
            fn_name,
            param_names,
            body,
        })
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let kind = match self.tokens.try_next()? {
            Some(Token::Keyword(Keyword::Const)) => VariableDeclarationKind::Const,
            Some(Token::Keyword(Keyword::Let)) => VariableDeclarationKind::Let,
            Some(Token::Keyword(Keyword::Var)) => VariableDeclarationKind::Var,
            actual => {
                return Err(Error::unexpected(
                    AnyOf(
                        Token::Keyword(Keyword::Const),
                        Token::Keyword(Keyword::Let),
                        vec![Token::Keyword(Keyword::Var)],
                    ),
                    actual,
                ))
            }
        };
        let mut entries = Vec::new();
        loop {
            entries.push(self.parse_variable_declaration_entry()?);

            match self.tokens.try_peek()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {
                    self.tokens.try_next().unwrap().unwrap();
                }
                Some(Token::Punctuator(Punctuator::Semicolon)) => break,
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::Semicolon),
                            vec![],
                        ),
                        actual.cloned(),
                    ))
                }
            }
        }
        Ok(VariableDeclaration { kind, entries })
    }

    fn parse_variable_declaration_entry(&mut self) -> Result<VariableDeclarationEntry> {
        let var_name = Identifier::from(self.expect_identifier(non_empty_str!("variable_name"))?);
        let initialiser =
            if let Some(Token::Punctuator(Punctuator::Equal)) = self.tokens.try_peek()? {
                self.tokens.try_next().unwrap().unwrap();
                Some(self.parse_expression()?)
            } else {
                None
            };
        Ok(VariableDeclarationEntry {
            var_name,
            initialiser,
        })
    }

    fn parse_assert_statement(&mut self) -> Result<AssertStatement> {
        self.expect_keyword(Keyword::Assert)?;
        let condition = self.parse_expression()?;
        Ok(AssertStatement { condition })
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement> {
        self.expect_keyword(Keyword::If)?;
        self.expect_punctuator(Punctuator::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect_punctuator(Punctuator::CloseParen)?;
        let success_block = self.parse_block(true)?;
        let else_block = if self
            .tokens
            .try_next_if_eq(&Token::Keyword(Keyword::Else))?
            .is_some()
        {
            if matches!(self.tokens.try_peek()?, Some(Token::Keyword(Keyword::If))) {
                // Recursively parse `else if { .. }` blocks
                Some(Block::new(
                    vec![],
                    vec![Statement::If(self.parse_if_statement()?)],
                ))
            } else {
                // Parse `else { .. }` blocks
                Some(self.parse_block(true)?)
            }
        } else {
            None
        };
        Ok(IfStatement {
            condition,
            success_block,
            else_block,
        })
    }

    fn parse_for_loop(&mut self) -> Result<ForLoop> {
        self.expect_keyword(Keyword::For)?;
        self.expect_punctuator(Punctuator::OpenParen)?;

        let initialiser = match self.tokens.try_peek()? {
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            _ => Some(self.parse_variable_declaration()?),
        };
        self.expect_punctuator(Punctuator::Semicolon)?;

        let condition = match self.tokens.try_peek()? {
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            _ => Some(self.parse_expression()?),
        };
        self.expect_punctuator(Punctuator::Semicolon)?;

        let incrementor = match self.tokens.try_peek()? {
            Some(Token::Punctuator(Punctuator::CloseParen)) => None,
            _ => Some(self.parse_expression()?),
        };
        self.expect_punctuator(Punctuator::CloseParen)?;

        let body = self.parse_block(true)?;
        Ok(ForLoop {
            initialiser,
            condition,
            incrementor,
            body,
        })
    }

    fn parse_while_loop(&mut self) -> Result<WhileLoop> {
        self.expect_keyword(Keyword::While)?;
        self.expect_punctuator(Punctuator::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect_punctuator(Punctuator::CloseParen)?;
        let body = self.parse_block(true)?;
        Ok(WhileLoop { condition, body })
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
        let expr = match self.tokens.try_peek()? {
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            _ => Some(self.parse_expression()?),
        };
        Ok(ReturnStatement { expr })
    }

    fn parse_throw_statement(&mut self) -> Result<ThrowStatement> {
        self.expect_keyword(Keyword::Throw)?;
        let exception = self.parse_expression()?;
        Ok(ThrowStatement { exception })
    }

    fn parse_try_statement(&mut self) -> Result<TryStatement> {
        self.expect_keyword(Keyword::Try)?;
        let body = self.parse_block(true)?;
        let catch_block = if matches!(
            self.tokens.try_peek()?,
            Some(Token::Keyword(Keyword::Catch))
        ) {
            Some(self.parse_catch_block()?)
        } else {
            None
        };
        let finally_block = if matches!(
            self.tokens.try_peek()?,
            Some(Token::Keyword(Keyword::Finally))
        ) {
            self.expect_keyword(Keyword::Finally)?;
            Some(FinallyBlock {
                inner: self.parse_block(true)?,
            })
        } else {
            None
        };
        if catch_block.is_some() || finally_block.is_some() {
            Ok(TryStatement {
                body,
                catch_block,
                finally_block,
            })
        } else {
            Err(Error::unexpected(
                AnyOf(
                    Token::Keyword(Keyword::Catch),
                    Token::Keyword(Keyword::Finally),
                    vec![],
                ),
                self.tokens.try_peek()?.cloned(),
            ))
        }
    }

    fn parse_catch_block(&mut self) -> Result<CatchBlock> {
        self.expect_keyword(Keyword::Catch)?;
        let exception_identifier = if matches!(
            self.tokens.try_peek()?,
            Some(Token::Punctuator(Punctuator::OpenParen))
        ) {
            self.expect_punctuator(Punctuator::OpenParen)?;
            let identifier = match self.tokens.try_next()? {
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
        let body = self.parse_block(true)?;
        Ok(CatchBlock {
            exception_identifier,
            body,
        })
    }

    fn parse_fn_parameters(&mut self) -> Result<Vec<Identifier>> {
        self.expect_punctuator(Punctuator::OpenParen)?;
        if self
            .tokens
            .try_next_if_eq(&Token::Punctuator(Punctuator::CloseParen))?
            .is_some()
        {
            return Ok(Vec::with_capacity(0));
        }

        let mut params = Vec::new();
        loop {
            match self.tokens.try_next()? {
                Some(Token::Identifier(param)) => {
                    params.push(Identifier::from(param));
                }
                actual => {
                    return Err(Error::unexpected(
                        Exactly(Token::Identifier(non_empty_str!("function_parameter"))),
                        actual,
                    ));
                }
            }
            match self.tokens.try_next()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {}
                Some(Token::Punctuator(Punctuator::CloseParen)) => break Ok(params),
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::CloseParen),
                            vec![],
                        ),
                        actual,
                    ))
                }
            }
        }
    }

    fn parse_fn_arguments(&mut self, consume_open_paren: bool) -> Result<Vec<Expression>> {
        if consume_open_paren {
            self.expect_punctuator(Punctuator::OpenParen)?;
        }
        if self
            .tokens
            .try_next_if_eq(&Token::Punctuator(Punctuator::CloseParen))?
            .is_some()
        {
            return Ok(Vec::with_capacity(0));
        }

        let mut args = Vec::new();
        loop {
            args.push(self.parse_expression()?);
            match self.tokens.try_next()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {}
                Some(Token::Punctuator(Punctuator::CloseParen)) => break Ok(args),
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::CloseParen),
                            vec![],
                        ),
                        actual,
                    ))
                }
            }
        }
    }

    fn parse_object_properties(&mut self) -> Result<HashMap<Identifier, Expression>> {
        let mut props = HashMap::default();
        Ok(loop {
            match self.tokens.try_peek()? {
                Some(Token::Punctuator(Punctuator::CloseBrace)) => break props,
                Some(Token::Identifier(_)) => {
                    let (key, value) = self.parse_object_property()?;
                    props.insert(key, value);
                }
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::CloseBrace),
                            Token::Identifier(non_empty_str!("property_key")),
                            vec![],
                        ),
                        actual.cloned(),
                    ))
                }
            }
            match self.tokens.try_peek()? {
                Some(Token::Punctuator(Punctuator::CloseBrace)) => break props,
                Some(Token::Punctuator(Punctuator::Comma)) => self
                    .tokens
                    .try_next_exact(&Token::Punctuator(Punctuator::Comma))?,
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::CloseBrace),
                            Token::Punctuator(Punctuator::Comma),
                            vec![],
                        ),
                        actual.cloned(),
                    ))
                }
            }
        })
    }

    fn parse_object_property(&mut self) -> Result<(Identifier, Expression)> {
        let key = match self.tokens.try_next()? {
            Some(Token::Identifier(id)) => Identifier::from(id),
            actual => {
                return Err(Error::unexpected(
                    Exactly(Token::Identifier(non_empty_str!("property_key"))),
                    actual,
                ))
            }
        };
        self.expect_punctuator(Punctuator::Colon)?;
        let value = self.parse_expression()?;
        Ok((key, value))
    }

    fn expect_keyword(&mut self, expected: Keyword) -> Result<()> {
        self.expect_token(Token::Keyword(expected))
    }

    fn expect_punctuator(&mut self, expected: Punctuator) -> Result<()> {
        self.expect_token(Token::Punctuator(expected))
    }

    fn expect_identifier(&mut self, placeholder: NonEmptyString) -> Result<NonEmptyString> {
        match self.tokens.try_next()? {
            Some(Token::Identifier(id)) => Ok(id),
            actual => Err(Error::unexpected(
                Exactly(Token::Identifier(placeholder)),
                actual,
            )),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<()> {
        match self.tokens.try_next()? {
            Some(actual) if actual == expected => Ok(()),
            actual => Err(Error::unexpected(Exactly(expected), actual)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Position {
    /// For example:
    /// - `++a`
    Prefix,
    /// For example:
    /// - `a++`
    /// - `a + b`
    PostfixOrInfix,
}

trait TryParse {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self>
    where
        Self: Sized;
}

impl TryParse for Operator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        AssignmentOperator::try_parse(punc, pos)
            .map(Self::Assignment)
            .or_else(|| BinaryOperator::try_parse(punc, pos).map(Self::Binary))
            .or_else(|| UnaryOperator::try_parse(punc, pos).map(Self::Unary))
            .or_else(|| TernaryOperator::try_parse(punc, pos).map(|_| Self::Ternary))
            .or_else(|| GroupingOperator::try_parse(punc, pos).map(|_| Self::Grouping))
            .or_else(|| FunctionCallOperator::try_parse(punc, pos).map(|_| Self::FunctionCall))
            .or_else(|| PropertyAccessOperator::try_parse(punc, pos).map(|_| Self::PropertyAccess))
            .or_else(|| {
                ComputedPropertyAccessOperator::try_parse(punc, pos)
                    .map(|_| Self::ComputedPropertyAccess)
            })
    }
}

impl TryParse for AssignmentOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::PostfixOrInfix {
            return None;
        }
        Some(match punc {
            Punctuator::Equal => Self::Assign,
            Punctuator::PlusEqual => Self::AddAssign,
            Punctuator::SlashEqual => Self::DivAssign,
            Punctuator::PercentEqual => Self::ModAssign,
            Punctuator::AsteriskEqual => Self::MulAssign,
            Punctuator::DoubleAsteriskEqual => Self::PowAssign,
            Punctuator::MinusEqual => Self::SubAssign,
            Punctuator::DoubleLessThanEqual => Self::ShiftLeftAssign,
            Punctuator::DoubleMoreThanEqual => Self::ShiftRightAssign,
            Punctuator::TripleMoreThanEqual => Self::ShiftRightUnsignedAssign,
            Punctuator::AmpersandEqual => Self::BitwiseAndAssign,
            Punctuator::PipeEqual => Self::BitwiseOrAssign,
            Punctuator::CaretEqual => Self::BitwiseXOrAssign,
            _ => return None,
        })
    }
}

impl TryParse for BinaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::PostfixOrInfix {
            return None;
        }
        Some(match punc {
            Punctuator::Plus => Self::Add,
            Punctuator::Slash => Self::Div,
            Punctuator::Percent => Self::Mod,
            Punctuator::Asterisk => Self::Mul,
            Punctuator::DoubleAsterisk => Self::Pow,
            Punctuator::Minus => Self::Sub,
            Punctuator::DoubleEqual => Self::Equal,
            Punctuator::BangEqual => Self::NotEqual,
            Punctuator::TripleEqual => Self::Identical,
            Punctuator::BangDoubleEqual => Self::NotIdentical,
            Punctuator::LessThan => Self::LessThan,
            Punctuator::LessThanEqual => Self::LessThanOrEqual,
            Punctuator::MoreThan => Self::MoreThan,
            Punctuator::MoreThanEqual => Self::MoreThanOrEqual,
            Punctuator::DoubleLessThan => Self::ShiftLeft,
            Punctuator::DoubleMoreThan => Self::ShiftRight,
            Punctuator::TripleMoreThan => Self::ShiftRightUnsigned,
            Punctuator::Ampersand => Self::BitwiseAnd,
            Punctuator::Pipe => Self::BitwiseOr,
            Punctuator::Caret => Self::BitwiseXOr,
            Punctuator::DoubleAmpersand => Self::LogicalAnd,
            Punctuator::DoublePipe => Self::LogicalOr,
            _ => return None,
        })
    }
}

impl TryParse for UnaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (Punctuator::DoubleMinus, Position::Prefix) => Self::DecrementPrefix,
            (Punctuator::DoubleMinus, Position::PostfixOrInfix) => Self::DecrementPostfix,
            (Punctuator::DoublePlus, Position::Prefix) => Self::IncrementPrefix,
            (Punctuator::DoublePlus, Position::PostfixOrInfix) => Self::IncrementPostfix,
            (Punctuator::Tilde, Position::Prefix) => Self::BitwiseNot,
            (Punctuator::Bang, Position::Prefix) => Self::LogicalNot,
            (Punctuator::Minus, Position::Prefix) => Self::NumericNegate,
            (Punctuator::Plus, Position::Prefix) => Self::NumericPlus,
            (_, _) => return None,
        })
    }
}

impl TryParse for TernaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!(
            (punc, pos),
            (Punctuator::Question, Position::PostfixOrInfix)
        )
        .then_some(Self)
    }
}

impl TryParse for GroupingOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::OpenParen, Position::Prefix)).then_some(Self)
    }
}

impl TryParse for FunctionCallOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!(
            (punc, pos),
            (Punctuator::OpenParen, Position::PostfixOrInfix)
        )
        .then_some(Self)
    }
}

impl TryParse for PropertyAccessOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::Dot, Position::PostfixOrInfix)).then_some(Self)
    }
}

impl TryParse for ComputedPropertyAccessOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!(
            (punc, pos),
            (Punctuator::OpenBracket, Position::PostfixOrInfix)
        )
        .then_some(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::{Keyword, Literal, NumericLiteral, Punctuator, Token};
    use crate::parser::error::ErrorKind::{UnexpectedEoi, UnexpectedToken};
    use std::assert_matches::assert_matches;

    #[test]
    fn parse_unclosed_block() {
        let tokens = vec![
            Token::Keyword(Keyword::While),
            Token::Punctuator(Punctuator::OpenParen),
            Token::Literal(Literal::Boolean(true)),
            Token::Punctuator(Punctuator::CloseParen),
            Token::Punctuator(Punctuator::OpenBrace),
        ];
        let parser = Parser::for_tokens(tokens.into_iter());
        assert_matches!(
            parser.execute(),
            Err(err) if matches!(
                err.kind(),
                UnexpectedEoi(Exactly(Token::Punctuator(Punctuator::CloseBrace)))
            )
        );
    }

    #[test]
    fn parse_unclosed_paren() {
        let tokens = vec![
            Token::Keyword(Keyword::While),
            Token::Punctuator(Punctuator::OpenParen),
            Token::Literal(Literal::Boolean(true)),
            Token::Punctuator(Punctuator::OpenBrace),
        ];
        let parser = Parser::for_tokens(tokens.into_iter());
        assert_matches!(
            parser.execute(),
            Err(err) if matches!(
                err.kind(),
                UnexpectedToken(
                    Exactly(Token::Punctuator(Punctuator::CloseParen)),
                    Token::Punctuator(Punctuator::OpenBrace)
                )
            )
        );
    }

    #[test]
    fn parse_unfinished_variable_decl() {
        let tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Punctuator(Punctuator::Semicolon),
        ];
        let parser = Parser::for_tokens(tokens.into_iter());
        assert_matches!(
            parser.execute(),
            Err(err) if matches!(
                err.kind(),
                UnexpectedToken(
                    Exactly(Token::Identifier(_)),
                    Token::Punctuator(Punctuator::Semicolon)
                )
            )
        );
    }

    #[test]
    fn parse_unfinished_binary_expression() {
        let tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Identifier(non_empty_str!("a")),
            Token::Punctuator(Punctuator::Equal),
            Token::Literal(Literal::Numeric(NumericLiteral::DecInt(1))),
            Token::Punctuator(Punctuator::Plus),
            Token::Literal(Literal::Numeric(NumericLiteral::DecInt(2))),
            Token::Punctuator(Punctuator::Plus),
            Token::Punctuator(Punctuator::Semicolon),
        ];
        let parser = Parser::for_tokens(tokens.into_iter());
        assert_matches!(
            parser.execute(),
            Err(err) if matches!(
                err.kind(),
                UnexpectedToken(Unspecified, Token::Punctuator(Punctuator::Semicolon))
            )
        );
    }
}
