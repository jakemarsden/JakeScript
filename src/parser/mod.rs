use crate::ast::{self, *};
use crate::iter::{IntoPeekableNth, PeekableNth};
use crate::lexer::{self, *};
use error::AllowToken::*;
use std::io;
use std::iter::Map;

pub use error::*;

mod error;

type Fallible<I> = Map<I, fn(Token) -> LexicalResult<Token>>;

pub struct Parser<I: Iterator<Item = LexicalResult<Token>>> {
    tokens: PeekableNth<I>,
    constants: ConstantPool,
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

impl<I: Iterator<Item = LexicalResult<Token>>> Parser<I> {
    pub fn for_tokens_fallible(source: I) -> Self {
        Self {
            tokens: source.peekable_nth(),
            constants: ConstantPool::default(),
        }
    }

    pub fn execute(mut self) -> ParseResult {
        let mut stmts = Vec::new();
        while self.tokens.try_peek()?.is_some() {
            stmts.push(self.parse_statement()?);
        }
        Ok(Program::new(Block::new(stmts), self.constants))
    }

    fn parse_block(&mut self) -> ParseResult<Block> {
        self.expect_punctuator(Punctuator::OpenBrace)?;
        let mut stmts = Vec::new();
        loop {
            match self.tokens.try_peek()? {
                Some(Token::Punctuator(Punctuator::CloseBrace)) => break,
                Some(_) => {}
                None => {
                    return Err(ParseError::unexpected_eoi(Exactly(Token::Punctuator(
                        Punctuator::CloseBrace,
                    ))))
                }
            }
            stmts.push(self.parse_statement()?);
        }
        self.expect_punctuator(Punctuator::CloseBrace)?;
        Ok(Block::new(stmts))
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.tokens.try_peek()? {
            Some(Token::Keyword(Keyword::If)) => {
                self.parse_if_statement().map(Statement::IfStatement)
            }
            Some(Token::Keyword(Keyword::Function)) => self
                .parse_function_declaration()
                .map(Statement::FunctionDeclaration),
            Some(Token::Keyword(Keyword::For)) => self.parse_for_loop().map(Statement::ForLoop),
            Some(Token::Keyword(Keyword::While)) => {
                self.parse_while_loop().map(Statement::WhileLoop)
            }

            Some(token) => {
                let stmt = match token {
                    Token::Keyword(Keyword::Assert) => {
                        self.parse_assertion().map(Statement::Assertion)
                    }
                    Token::Keyword(Keyword::Print | Keyword::PrintLn) => {
                        self.parse_print_statement().map(Statement::Print)
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
                    Token::Keyword(Keyword::Const | Keyword::Let | Keyword::Var) => self
                        .parse_variable_declaration()
                        .map(Statement::VariableDeclaration),
                    _ => self.parse_expression().map(Statement::Expression),
                }?;
                self.expect_punctuator(Punctuator::Semicolon)?;
                Ok(stmt)
            }
            None => Err(ParseError::unexpected_eoi(Unspecified)),
        }
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_expression_impl(Precedence::MIN)
    }

    fn parse_expression_impl(&mut self, min_precedence: Precedence) -> ParseResult<Expression> {
        let mut expression = self.parse_primary_expression()?;
        while let Some(&Token::Punctuator(punctuator)) = self.tokens.try_peek()? {
            if let Some(op_kind) = Operator::try_parse(punctuator, Position::Postfix) {
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

    fn parse_primary_expression(&mut self) -> ParseResult<Expression> {
        Ok(match self.tokens.try_next()? {
            Some(Token::Identifier(identifier)) => {
                let var_name = self.constants.allocate_if_absent(identifier);
                Expression::VariableAccess(VariableAccessExpression { var_name })
            }
            Some(Token::Punctuator(Punctuator::OpenBrace)) => {
                Expression::Literal(LiteralExpression {
                    value: if self
                        .tokens
                        .try_next_if_eq(&Token::Punctuator(Punctuator::CloseBrace))?
                        .is_some()
                    {
                        ast::Literal::Object
                    } else {
                        todo!(
                            "Parser::parse_primary_expression: Only empty object literals are \
                             supported"
                        )
                    },
                })
            }
            Some(Token::Punctuator(punc)) => {
                if let Some(op_kind) = UnaryOperator::try_parse(punc, Position::Prefix) {
                    let operand = self.parse_expression_impl(op_kind.precedence())?;
                    Expression::Unary(UnaryExpression {
                        kind: op_kind,
                        operand: Box::new(operand),
                    })
                } else if GroupingOp::try_parse(punc, Position::Prefix).is_some() {
                    let inner = self.parse_expression()?;
                    self.expect_punctuator(Punctuator::CloseParen)?;
                    Expression::Grouping(GroupingExpression {
                        inner: Box::new(inner),
                    })
                } else {
                    return Err(ParseError::unexpected_token(
                        Unspecified,
                        Token::Punctuator(punc),
                    ));
                }
            }
            Some(Token::Literal(literal)) => Expression::Literal(LiteralExpression {
                value: match literal {
                    lexer::Literal::Boolean(value) => ast::Literal::Boolean(value),
                    lexer::Literal::Numeric(
                        NumericLiteral::BinInt(value)
                        | NumericLiteral::OctInt(value)
                        | NumericLiteral::DecInt(value)
                        | NumericLiteral::HexInt(value),
                    ) => ast::Literal::Numeric(value),
                    lexer::Literal::Numeric(NumericLiteral::Decimal(value)) => {
                        todo!("NumericLiteral::Decimal: {}", value)
                    }
                    lexer::Literal::String(
                        StringLiteral::SingleQuoted(value) | StringLiteral::DoubleQuoted(value),
                    ) => ast::Literal::String(value),
                    lexer::Literal::RegEx(_) => {
                        // FIXME: Support Literal::RegEx properly
                        ast::Literal::Undefined
                    }
                    lexer::Literal::Null => ast::Literal::Null,
                    lexer::Literal::Undefined => ast::Literal::Undefined,
                },
            }),
            Some(Token::Keyword(Keyword::Function)) => {
                let param_names = self.parse_fn_parameters()?;
                let body = self.parse_block()?;
                Expression::Literal(LiteralExpression {
                    value: ast::Literal::AnonFunction { param_names, body },
                })
            }
            actual => return Err(ParseError::unexpected(Unspecified, actual)),
        })
    }

    fn parse_secondary_expression(
        &mut self,
        lhs: Expression,
        op_kind: Operator,
    ) -> ParseResult<Expression> {
        Ok(match op_kind {
            Operator::Assignment(kind) => Expression::Assignment(AssignmentExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_expression_impl(op_kind.precedence())?),
            }),
            Operator::Binary(kind) => Expression::Binary(BinaryExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_expression_impl(op_kind.precedence())?),
            }),
            Operator::Unary(kind) => Expression::Unary(UnaryExpression {
                kind,
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
        })
    }

    fn parse_function_declaration(&mut self) -> ParseResult<FunctionDeclaration> {
        self.expect_keyword(Keyword::Function)?;
        match self.tokens.try_next()? {
            Some(Token::Identifier(fn_name)) => {
                let fn_name = self.constants.allocate_if_absent(fn_name);
                let param_names = self.parse_fn_parameters()?;
                let body = self.parse_block()?;
                Ok(FunctionDeclaration {
                    fn_name,
                    param_names,
                    body,
                })
            }
            actual => Err(ParseError::unexpected(
                Exactly(Token::Identifier("function_name".to_owned())),
                actual,
            )),
        }
    }

    fn parse_variable_declaration(&mut self) -> ParseResult<VariableDeclaration> {
        let kind = match self.tokens.try_next()? {
            Some(Token::Keyword(Keyword::Const)) => VariableDeclarationKind::Const,
            Some(Token::Keyword(Keyword::Let)) => VariableDeclarationKind::Let,
            Some(Token::Keyword(Keyword::Var)) => VariableDeclarationKind::Var,
            actual => {
                return Err(ParseError::unexpected(
                    AnyOf(
                        Token::Keyword(Keyword::Const),
                        Token::Keyword(Keyword::Let),
                        vec![Token::Keyword(Keyword::Var)],
                    ),
                    actual,
                ));
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
                    return Err(ParseError::unexpected(
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

    fn parse_variable_declaration_entry(&mut self) -> ParseResult<VariableDeclarationEntry> {
        let var_name = match self.tokens.try_next()? {
            Some(Token::Identifier(var_name)) => self.constants.allocate_if_absent(var_name),
            actual => {
                return Err(ParseError::unexpected(
                    Exactly(Token::Identifier("variable_name".to_owned())),
                    actual,
                ))
            }
        };
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

    fn parse_assertion(&mut self) -> ParseResult<Assertion> {
        self.expect_keyword(Keyword::Assert)?;
        let condition = self.parse_expression()?;
        Ok(Assertion { condition })
    }

    fn parse_print_statement(&mut self) -> ParseResult<PrintStatement> {
        let new_line = match self.tokens.try_next()? {
            Some(Token::Keyword(Keyword::Print)) => false,
            Some(Token::Keyword(Keyword::PrintLn)) => true,
            actual => {
                return Err(ParseError::unexpected(
                    AnyOf(
                        Token::Keyword(Keyword::Print),
                        Token::Keyword(Keyword::PrintLn),
                        vec![],
                    ),
                    actual,
                ))
            }
        };
        let argument = self.parse_expression()?;
        Ok(PrintStatement { argument, new_line })
    }

    fn parse_if_statement(&mut self) -> ParseResult<IfStatement> {
        self.expect_keyword(Keyword::If)?;
        self.expect_punctuator(Punctuator::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect_punctuator(Punctuator::CloseParen)?;
        let success_block = self.parse_block()?;
        let else_block = if self
            .tokens
            .try_next_if_eq(&Token::Keyword(Keyword::Else))?
            .is_some()
        {
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(IfStatement {
            condition,
            success_block,
            else_block,
        })
    }

    fn parse_for_loop(&mut self) -> ParseResult<ForLoop> {
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

        let block = self.parse_block()?;
        Ok(ForLoop {
            initialiser,
            condition,
            incrementor,
            block,
        })
    }

    fn parse_while_loop(&mut self) -> ParseResult<WhileLoop> {
        self.expect_keyword(Keyword::While)?;
        self.expect_punctuator(Punctuator::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect_punctuator(Punctuator::CloseParen)?;
        let block = self.parse_block()?;
        Ok(WhileLoop { condition, block })
    }

    fn parse_break_statement(&mut self) -> ParseResult<BreakStatement> {
        self.expect_keyword(Keyword::Break)?;
        Ok(BreakStatement {})
    }

    fn parse_continue_statement(&mut self) -> ParseResult<ContinueStatement> {
        self.expect_keyword(Keyword::Continue)?;
        Ok(ContinueStatement {})
    }

    fn parse_return_statement(&mut self) -> ParseResult<ReturnStatement> {
        self.expect_keyword(Keyword::Return)?;
        let expr = match self.tokens.try_peek()? {
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            _ => Some(self.parse_expression()?),
        };
        Ok(ReturnStatement { expr })
    }

    fn parse_fn_parameters(&mut self) -> ParseResult<Vec<ConstantId>> {
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
                    let param_constant_id = self.constants.allocate_if_absent(param);
                    params.push(param_constant_id);
                }
                actual => {
                    return Err(ParseError::unexpected(
                        Exactly(Token::Identifier("function_parameter".to_owned())),
                        actual,
                    ))
                }
            }
            match self.tokens.try_next()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {}
                Some(Token::Punctuator(Punctuator::CloseParen)) => break Ok(params),
                actual => {
                    return Err(ParseError::unexpected(
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

    fn parse_fn_arguments(&mut self, consume_open_paren: bool) -> ParseResult<Vec<Expression>> {
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
                    return Err(ParseError::unexpected(
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

    fn expect_keyword(&mut self, expected: Keyword) -> ParseResult<()> {
        self.expect_token(Token::Keyword(expected))
    }

    fn expect_punctuator(&mut self, expected: Punctuator) -> ParseResult<()> {
        self.expect_token(Token::Punctuator(expected))
    }

    fn expect_token(&mut self, expected: Token) -> ParseResult<()> {
        match self.tokens.try_next()? {
            Some(actual) if actual == expected => Ok(()),
            actual => Err(ParseError::unexpected(Exactly(expected), actual)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Position {
    Prefix,
    Postfix,
}

trait TryParse {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self>
    where
        Self: Sized;
}

impl TryParse for Operator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(if let Some(op) = AssignmentOperator::try_parse(punc, pos) {
            Self::Assignment(op)
        } else if let Some(op) = BinaryOperator::try_parse(punc, pos) {
            Self::Binary(op)
        } else if let Some(op) = UnaryOperator::try_parse(punc, pos) {
            Self::Unary(op)
        } else if TernaryOp::try_parse(punc, pos).is_some() {
            Self::Ternary
        } else if GroupingOp::try_parse(punc, pos).is_some() {
            Self::Grouping
        } else if FunctionCallOp::try_parse(punc, pos).is_some() {
            Self::FunctionCall
        } else if PropertyAccessOp::try_parse(punc, pos).is_some() {
            Self::PropertyAccess
        } else {
            return None;
        })
    }
}

impl TryParse for AssignmentOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::Postfix {
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
        if pos != Position::Postfix {
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
            (Punctuator::DoubleMinus, Position::Postfix) => Self::DecrementPostfix,
            (Punctuator::DoublePlus, Position::Prefix) => Self::IncrementPrefix,
            (Punctuator::DoublePlus, Position::Postfix) => Self::IncrementPostfix,

            (Punctuator::Tilde, Position::Prefix) => Self::BitwiseNot,
            (Punctuator::Bang, Position::Prefix) => Self::LogicalNot,
            (Punctuator::Minus, Position::Prefix) => Self::NumericNegate,
            (Punctuator::Plus, Position::Prefix) => Self::NumericPlus,

            (_, _) => return None,
        })
    }
}

impl TryParse for TernaryOp {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::Question, Position::Postfix)).then_some(Self)
    }
}

impl TryParse for GroupingOp {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::OpenParen, Position::Prefix)).then_some(Self)
    }
}

impl TryParse for FunctionCallOp {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::OpenParen, Position::Postfix)).then_some(Self)
    }
}

impl TryParse for PropertyAccessOp {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::Dot, Position::Postfix)).then_some(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::{Keyword, Literal, Punctuator, Token};
    use crate::parser::error::ParseErrorKind::*;
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
            Token::Identifier("a".to_owned()),
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
