use crate::ast::{self, *};
use crate::iter::{IntoPeekableNth, PeekableNth};
use crate::lexer::{self, *};
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
                None => panic!("Block not closed before end of input"),
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
            None => panic!("Expected statement but was <end>"),
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
                    expression = self
                        .parse_secondary_expression(expression, op_kind)
                        .expect("Expected secondary expression but was <end>");
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
                    let operand = self
                        .parse_expression_impl(op_kind.precedence())
                        .expect("Expected expression but was <end>");
                    Expression::Unary(UnaryExpression {
                        kind: op_kind,
                        operand: Box::new(operand),
                    })
                } else if GroupingOp::try_parse(punc, Position::Prefix).is_some() {
                    let inner = self
                        .parse_expression()
                        .expect("Expected expression but was <end>");
                    self.expect_punctuator(Punctuator::CloseParen)?;
                    Expression::Grouping(GroupingExpression {
                        inner: Box::new(inner),
                    })
                } else {
                    todo!(
                        "Parser::parse_primary_expression: token={}",
                        Token::Punctuator(punc)
                    )
                }
            }
            Some(Token::Literal(literal)) => Expression::Literal(LiteralExpression {
                value: match literal {
                    lexer::Literal::Boolean(value) => ast::Literal::Boolean(value),
                    lexer::Literal::Numeric(value) => ast::Literal::Numeric(value),
                    lexer::Literal::String(value) => ast::Literal::String(value),
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
            Some(token) => todo!("Parser::parse_primary_expression: token={}", token),
            None => panic!("Expected primary expression but was <end>"),
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
                rhs: Box::new(
                    self.parse_expression_impl(op_kind.precedence())
                        .expect("Expected right-hand-side of assignment expression but was <end>"),
                ),
            }),
            Operator::Binary(kind) => Expression::Binary(BinaryExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(
                    self.parse_expression_impl(op_kind.precedence())
                        .expect("Expected right-hand-side of binary expression but was <end>"),
                ),
            }),
            Operator::Unary(kind) => Expression::Unary(UnaryExpression {
                kind,
                operand: Box::new(lhs),
            }),
            Operator::Ternary => {
                let condition = lhs;
                let lhs = self.parse_expression_impl(op_kind.precedence()).expect(
                    "Expected left-hand-side expression of ternary expression but was <end>",
                );
                match self.tokens.try_next()? {
                    Some(Token::Punctuator(Punctuator::Colon)) => {}
                    Some(token) => panic!(
                        "Expected colon between ternary expressions but was {}",
                        token
                    ),
                    None => panic!("Expected colon between ternary expressions but was <end>"),
                }
                let rhs = self.parse_expression_impl(op_kind.precedence()).expect(
                    "Expected right-hand-side expression of ternary expression but was <end>",
                );
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
                let rhs = self
                    .parse_expression_impl(op_kind.precedence())
                    .expect("Expected right-hand-side of property access expression but was <end>");
                Expression::PropertyAccess(PropertyAccessExpression {
                    base: Box::new(lhs),
                    property_name: match rhs {
                        Expression::VariableAccess(VariableAccessExpression { var_name }) => {
                            var_name
                        }
                        rhs_expr => panic!("Expected property name but was {:#?}", rhs_expr),
                    },
                })
            }
        })
    }

    fn parse_function_declaration(&mut self) -> ParseResult<FunctionDeclaration> {
        self.expect_keyword(Keyword::Function)?;
        if let Some(Token::Identifier(fn_name)) = self.tokens.try_next()? {
            let fn_name = self.constants.allocate_if_absent(fn_name);
            let param_names = self.parse_fn_parameters()?;
            let body = self.parse_block()?;
            Ok(FunctionDeclaration {
                fn_name,
                param_names,
                body,
            })
        } else {
            panic!("Expected function name")
        }
    }

    fn parse_variable_declaration(&mut self) -> ParseResult<VariableDeclaration> {
        let kind = match self.tokens.try_next()? {
            Some(Token::Keyword(Keyword::Const)) => VariableDeclarationKind::Const,
            Some(Token::Keyword(Keyword::Let)) => VariableDeclarationKind::Let,
            Some(Token::Keyword(Keyword::Var)) => VariableDeclarationKind::Var,
            Some(token) => panic!("Expected variable declaration but was {}", token),
            None => panic!("Expected variable declaration but was <end>"),
        };
        let mut entries = Vec::new();
        loop {
            entries.push(self.parse_variable_declaration_entry()?);

            match self.tokens.try_peek()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {
                    self.tokens.try_next().unwrap().unwrap();
                }
                Some(Token::Punctuator(Punctuator::Semicolon)) => break,
                Some(token) => panic!("Expected comma or semicolon but was {}", token),
                None => panic!("Expected comma or semicolon but was <end>"),
            }
        }
        Ok(VariableDeclaration { kind, entries })
    }

    fn parse_variable_declaration_entry(&mut self) -> ParseResult<VariableDeclarationEntry> {
        let var_name = match self.tokens.try_next()? {
            Some(Token::Identifier(var_name)) => self.constants.allocate_if_absent(var_name),
            Some(token) => unreachable!("Expected variable name but was {}", token),
            None => unreachable!("Expected variable name but was <end>"),
        };
        let initialiser =
            if let Some(Token::Punctuator(Punctuator::Equal)) = self.tokens.try_peek()? {
                self.tokens.try_next().unwrap().unwrap();
                let expr = self
                    .parse_expression()
                    .expect("Expected expression but was <end>");
                Some(expr)
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
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        Ok(Assertion { condition })
    }

    fn parse_print_statement(&mut self) -> ParseResult<PrintStatement> {
        let new_line = match self.tokens.try_next()? {
            Some(Token::Keyword(Keyword::Print)) => false,
            Some(Token::Keyword(Keyword::PrintLn)) => true,
            Some(token) => unreachable!(
                "Expected `{}` or `{}` but was {}",
                Keyword::Print,
                Keyword::PrintLn,
                token
            ),
            None => unreachable!(
                "Expected `{}` or `{}` but was <end>",
                Keyword::Print,
                Keyword::PrintLn
            ),
        };
        let argument = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        Ok(PrintStatement { argument, new_line })
    }

    fn parse_if_statement(&mut self) -> ParseResult<IfStatement> {
        self.expect_keyword(Keyword::If)?;
        self.expect_punctuator(Punctuator::OpenParen)?;
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
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
            Some(_) => Some(
                self.parse_variable_declaration()
                    .expect("Expected variable declaration but was <end>"),
            ),
            None => panic!("Expected variable declaration or <end>"),
        };
        self.expect_punctuator(Punctuator::Semicolon)?;

        let condition = match self.tokens.try_peek()? {
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            Some(_) => Some(
                self.parse_expression()
                    .expect("Expected expression but was <end>"),
            ),
            None => panic!("Expected expression or semicolon but was <end>"),
        };
        self.expect_punctuator(Punctuator::Semicolon)?;

        let incrementor = match self.tokens.try_peek()? {
            Some(Token::Punctuator(Punctuator::CloseParen)) => None,
            Some(_) => Some(
                self.parse_expression()
                    .expect("Expected expression but was <end>"),
            ),
            None => panic!("Expected expression or close paren but was <end>"),
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
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
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
            Some(_) => Some(
                self.parse_expression()
                    .expect("Expected expression but was <end>"),
            ),
            None => panic!("Expected expression or semicolon but was <end>"),
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
                Some(token) => panic!("Expected function parameter identifier but was {}", token),
                None => panic!("Expected function parameter identifier but was <end>"),
            }
            match self.tokens.try_next()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {}
                Some(Token::Punctuator(Punctuator::CloseParen)) => break Ok(params),
                Some(token) => panic!("Expected comma or closing parenthesis but was {:?}", token),
                None => panic!("Expected comma or closing parenthesis but was <end>"),
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
                Some(token) => panic!("Expected comma or closing parenthesis but was {:?}", token),
                None => panic!("Expected comma or closing parenthesis but was <end>"),
            }
        }
    }

    fn expect_keyword(&mut self, expected: Keyword) -> LexicalResult<()> {
        self.tokens.try_next_exact(&Token::Keyword(expected))
    }

    fn expect_punctuator(&mut self, expected: Punctuator) -> LexicalResult<()> {
        self.tokens.try_next_exact(&Token::Punctuator(expected))
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
