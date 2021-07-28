use crate::ast::{self, *};
use crate::lexer::{Literal, *};
use crate::util::Stream;
use std::convert::TryFrom;
use std::iter::Iterator;

pub struct Parser(Stream<Token>);

impl Parser {
    pub fn for_lexer(source: Lexer) -> Self {
        Self::new(source)
    }

    pub fn for_tokens(source: Vec<Token>) -> Self {
        Self::new(source.into_iter())
    }

    pub fn new(source: impl Iterator<Item = Token>) -> Self {
        Self(Stream::new(source.filter(Token::is_significant)))
    }
}

/// ```rust
/// # use jakescript::ast::{self, *};
/// # use jakescript::lexer::{Literal, *};
/// # use jakescript::parser::*;
/// let source = vec![
///     Token::Literal(Literal::Numeric(100)),
///     Token::Punctuator(Punctuator::Plus),
///     Token::Literal(Literal::Numeric(50)),
///     Token::Punctuator(Punctuator::Plus),
///     Token::Literal(Literal::Numeric(17)),
///     Token::Punctuator(Punctuator::Semicolon),
/// ];
/// let mut parser = Parser::for_tokens(source);
/// assert_eq!(
///     parser.execute(),
///     Program::new(Block::new(vec![Statement::Expression(Expression::Binary(
///         BinaryExpression {
///             kind: BinaryOp::Add,
///             lhs: Box::new(Expression::Binary(BinaryExpression {
///                 kind: BinaryOp::Add,
///                 lhs: Box::new(Expression::Literal(ast::Literal::Numeric(100))),
///                 rhs: Box::new(Expression::Literal(ast::Literal::Numeric(50))),
///             })),
///             rhs: Box::new(Expression::Literal(ast::Literal::Numeric(17))),
///         }
///     )),]))
/// );
/// ```
///
/// ```rust
/// # use jakescript::ast::{self, *};
/// # use jakescript::lexer::{Literal, *};
/// # use jakescript::parser::*;
/// let source = vec![
///     Token::Keyword(Keyword::Let),
///     Token::Identifier("a".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Literal(Literal::Numeric(100)),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Keyword(Keyword::Let),
///     Token::Identifier("b".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Literal(Literal::Numeric(50)),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Identifier("a".to_owned()),
///     Token::Punctuator(Punctuator::Plus),
///     Token::Identifier("b".to_owned()),
///     Token::Punctuator(Punctuator::Semicolon),
/// ];
/// let mut parser = Parser::for_tokens(source);
/// assert_eq!(
///     parser.execute(),
///     Program::new(Block::new(vec![
///         Statement::VariableDeclaration {
///             kind: VariableDeclKind::Let,
///             var_name: "a".to_owned(),
///             initialiser: Some(Expression::Literal(ast::Literal::Numeric(100)))
///         },
///         Statement::VariableDeclaration {
///             kind: VariableDeclKind::Let,
///             var_name: "b".to_owned(),
///             initialiser: Some(Expression::Literal(ast::Literal::Numeric(50)))
///         },
///         Statement::Expression(Expression::Binary(BinaryExpression {
///             kind: BinaryOp::Add,
///             lhs: Box::new(Expression::VariableAccess("a".to_owned())),
///             rhs: Box::new(Expression::VariableAccess("b".to_owned()))
///         }))
///     ]))
/// );
/// ```
///
/// ```rust
/// # use jakescript::ast::{self, *};
/// # use jakescript::lexer::{Literal, *};
/// # use jakescript::parser::*;
/// let source = vec![
///     Token::Keyword(Keyword::Let),
///     Token::Identifier("a".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Literal(Literal::Numeric(100)),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Keyword(Keyword::Let),
///     Token::Identifier("b".to_owned()),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Keyword(Keyword::If),
///     Token::Punctuator(Punctuator::OpenParen),
///     Token::Identifier("a".to_owned()),
///     Token::Punctuator(Punctuator::MoreThanEqual),
///     Token::Literal(Literal::Numeric(3)),
///     Token::Punctuator(Punctuator::CloseParen),
///     Token::Punctuator(Punctuator::OpenBrace),
///     Token::Identifier("b".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Literal(Literal::String("success block!".to_owned())),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Punctuator(Punctuator::CloseBrace),
///     Token::Keyword(Keyword::Else),
///     Token::Punctuator(Punctuator::OpenBrace),
///     Token::Identifier("b".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Literal(Literal::String("else block!".to_owned())),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Punctuator(Punctuator::CloseBrace),
/// ];
/// let mut parser = Parser::for_tokens(source);
/// assert_eq!(
///     parser.execute(),
///     Program::new(Block::new(vec![
///         Statement::VariableDeclaration {
///             kind: VariableDeclKind::Let,
///             var_name: "a".to_owned(),
///             initialiser: Some(Expression::Literal(ast::Literal::Numeric(100)))
///         },
///         Statement::VariableDeclaration {
///             kind: VariableDeclKind::Let,
///             var_name: "b".to_owned(),
///             initialiser: None,
///         },
///         Statement::IfStatement(IfStatement {
///             condition: Expression::Binary(BinaryExpression {
///                 kind: BinaryOp::MoreThanOrEqual,
///                 lhs: Box::new(Expression::VariableAccess("a".to_owned())),
///                 rhs: Box::new(Expression::Literal(ast::Literal::Numeric(3)))
///             }),
///             success_block: Block::new(vec![Statement::Expression(Expression::Assignment(
///                 AssignmentExpression {
///                     kind: AssignmentOp::Assign,
///                     lhs: Box::new(Expression::VariableAccess("b".to_owned())),
///                     rhs: Box::new(Expression::Literal(ast::Literal::String(
///                         "success block!".to_owned()
///                     ))),
///                 }
///             )),]),
///             else_block: Some(Block::new(vec![Statement::Expression(
///                 Expression::Assignment(AssignmentExpression {
///                     kind: AssignmentOp::Assign,
///                     lhs: Box::new(Expression::VariableAccess("b".to_owned())),
///                     rhs: Box::new(Expression::Literal(ast::Literal::String(
///                         "else block!".to_owned()
///                     ))),
///                 })
///             ),]))
///         }),
///     ]))
/// );
/// ```
///
/// ```rust
/// # use jakescript::ast::{self, *};
/// # use jakescript::lexer::{Literal, *};
/// # use jakescript::parser::*;
/// let source = vec![
///     Token::Keyword(Keyword::Let),
///     Token::Identifier("a".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Literal(Literal::Numeric(3)),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Keyword(Keyword::While),
///     Token::Punctuator(Punctuator::OpenParen),
///     Token::Identifier("a".to_owned()),
///     Token::Punctuator(Punctuator::BangDoubleEqual),
///     Token::Literal(Literal::Numeric(0)),
///     Token::Punctuator(Punctuator::CloseParen),
///     Token::Punctuator(Punctuator::OpenBrace),
///     Token::Identifier("a".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Identifier("a".to_owned()),
///     Token::Punctuator(Punctuator::Minus),
///     Token::Literal(Literal::Numeric(1)),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Punctuator(Punctuator::CloseBrace),
/// ];
/// let mut parser = Parser::for_tokens(source);
/// assert_eq!(
///     parser.execute(),
///     Program::new(Block::new(vec![
///         Statement::VariableDeclaration {
///             kind: VariableDeclKind::Let,
///             var_name: "a".to_owned(),
///             initialiser: Some(Expression::Literal(ast::Literal::Numeric(3)))
///         },
///         Statement::WhileLoop(WhileLoop {
///             condition: Expression::Binary(BinaryExpression {
///                 kind: BinaryOp::NotIdentical,
///                 lhs: Box::new(Expression::VariableAccess("a".to_owned())),
///                 rhs: Box::new(Expression::Literal(ast::Literal::Numeric(0)))
///             }),
///             block: Block::new(vec![Statement::Expression(Expression::Assignment(
///                 AssignmentExpression {
///                     kind: AssignmentOp::Assign,
///                     lhs: Box::new(Expression::VariableAccess("a".to_owned())),
///                     rhs: Box::new(Expression::Binary(BinaryExpression {
///                         kind: BinaryOp::Sub,
///                         lhs: Box::new(Expression::VariableAccess("a".to_string())),
///                         rhs: Box::new(Expression::Literal(ast::Literal::Numeric(1))),
///                     })),
///                 }
///             )),]),
///         }),
///     ]))
/// );
/// ```
impl Parser {
    pub fn execute(mut self) -> Program {
        let mut stmts = Vec::new();
        while let Some(stmt) = self.parse_statement() {
            stmts.push(stmt);
        }
        Program::new(Block::new(stmts))
    }

    fn parse_block(&mut self) -> Block {
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::OpenBrace));
        let mut stmts = Vec::new();
        while !matches!(
            self.0.peek(),
            Some(Token::Punctuator(Punctuator::CloseBrace))
        ) {
            if let Some(stmt) = self.parse_statement() {
                stmts.push(stmt);
            } else {
                // Block not closed before end of input
                break;
            }
        }
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::CloseBrace));
        Block::new(stmts)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.0.peek()? {
            Token::Punctuator(Punctuator::OpenBrace) => Some(Statement::Block(self.parse_block())),
            Token::Keyword(Keyword::Assert) => self.parse_assertion().map(Statement::Assertion),
            Token::Keyword(Keyword::If) => self.parse_if_statement().map(Statement::IfStatement),
            Token::Keyword(Keyword::Const | Keyword::Let) => {
                self.parse_variable_declaration()
                    .map(
                        |(kind, var_name, initialiser)| Statement::VariableDeclaration {
                            kind,
                            var_name,
                            initialiser,
                        },
                    )
            }
            Token::Keyword(Keyword::While) => self.parse_while_loop().map(Statement::WhileLoop),
            _ => self.parse_expression().map(Statement::Expression),
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_expression_impl(Precedence::MIN)
    }

    fn parse_expression_impl(&mut self, min_precedence: Precedence) -> Option<Expression> {
        let mut expression = self.parse_primary_expression()?;
        loop {
            match self.0.peek() {
                Some(&Token::Punctuator(Punctuator::Semicolon)) => {
                    self.0.advance();
                }
                Some(&Token::Punctuator(punctuator)) => {
                    if let Ok(op_kind) = Op::try_from(punctuator) {
                        if op_kind.precedence() > min_precedence {
                            self.0.advance();
                            expression = self
                                .parse_secondary_expression(expression, op_kind)
                                .expect("Expected secondary expression but was <end>");
                            continue;
                        }
                    }
                }
                Some(_) | None => {}
            }
            break;
        }
        Some(expression)
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        Some(match self.0.consume()? {
            Token::Identifier(name) => Expression::VariableAccess(name),
            Token::Literal(literal) => Expression::Literal(match literal {
                Literal::Boolean(value) => ast::Literal::Boolean(value),
                Literal::Null => ast::Literal::Null,
                Literal::Numeric(value) => ast::Literal::Numeric(value),
                Literal::String(value) => ast::Literal::String(value),
            }),
            token => todo!("Parser::parse_primary_expression: token={}", token),
        })
    }

    fn parse_secondary_expression(&mut self, lhs: Expression, op_kind: Op) -> Option<Expression> {
        let rhs = self
            .parse_expression_impl(op_kind.precedence())
            .expect("Expected RHS of binary expression but was <end>");

        Some(match op_kind {
            Op::Assignment(kind) => Expression::Assignment(AssignmentExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Op::Binary(kind) => Expression::Binary(BinaryExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
        })
    }

    fn parse_variable_declaration(
        &mut self,
    ) -> Option<(VariableDeclKind, IdentifierName, Option<Expression>)> {
        let kind = match self.0.consume() {
            Some(Token::Keyword(Keyword::Const)) => VariableDeclKind::Const,
            Some(Token::Keyword(Keyword::Let)) => VariableDeclKind::Let,
            token => panic!("Expected variable declaration but was: {:?}", token),
        };

        if let Some(Token::Identifier(var_name)) = self.0.consume() {
            let initialiser = match self.0.consume() {
                Some(Token::Punctuator(Punctuator::Equal)) => Some(
                    self.parse_expression()
                        .expect("Expected initialiser or semicolon but was <end>"),
                ),
                Some(Token::Punctuator(Punctuator::Semicolon)) => None,
                Some(token) => panic!("Expected initialiser or semicolon but was {}", token),
                None => panic!("Expected initialiser or semicolon but was <end>"),
            };
            Some((kind, var_name, initialiser))
        } else {
            panic!("Expected variable name");
        }
    }

    fn parse_assertion(&mut self) -> Option<Assertion> {
        self.0.consume_exact(&Token::Keyword(Keyword::Assert));
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        Some(Assertion { condition })
    }

    fn parse_if_statement(&mut self) -> Option<IfStatement> {
        self.0.consume_exact(&Token::Keyword(Keyword::If));
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::OpenParen));
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::CloseParen));
        let success_block = self.parse_block();
        let else_block = if self.0.consume_eq(&Token::Keyword(Keyword::Else)).is_some() {
            Some(self.parse_block())
        } else {
            None
        };
        Some(IfStatement {
            condition,
            success_block,
            else_block,
        })
    }

    fn parse_while_loop(&mut self) -> Option<WhileLoop> {
        self.0.consume_exact(&Token::Keyword(Keyword::While));
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::OpenParen));
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::CloseParen));
        let block = self.parse_block();
        Some(WhileLoop { condition, block })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    Assignment(AssignmentOp),
    Binary(BinaryOp),
}

impl Operator for Op {
    fn associativity(&self) -> Associativity {
        match self {
            Self::Assignment(op) => op.associativity(),
            Self::Binary(op) => op.associativity(),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Assignment(op) => op.precedence(),
            Self::Binary(op) => op.precedence(),
        }
    }
}

impl TryFrom<Punctuator> for Op {
    type Error = ();

    fn try_from(punc: Punctuator) -> Result<Self, Self::Error> {
        Ok(match punc {
            Punctuator::Equal => Self::Assignment(AssignmentOp::Assign),
            Punctuator::PlusEqual => Self::Assignment(AssignmentOp::AddAssign),
            Punctuator::SlashEqual => Self::Assignment(AssignmentOp::DivAssign),
            Punctuator::PercentEqual => Self::Assignment(AssignmentOp::ModAssign),
            Punctuator::AsteriskEqual => Self::Assignment(AssignmentOp::MulAssign),
            Punctuator::DoubleAsteriskEqual => Self::Assignment(AssignmentOp::PowAssign),
            Punctuator::MinusEqual => Self::Assignment(AssignmentOp::SubAssign),
            Punctuator::DoubleLessThanEqual => Self::Assignment(AssignmentOp::ShiftLeftAssign),
            Punctuator::DoubleMoreThanEqual => Self::Assignment(AssignmentOp::ShiftRightAssign),
            Punctuator::TripleMoreThanEqual => {
                Self::Assignment(AssignmentOp::ShiftRightUnsignedAssign)
            }
            Punctuator::AmpersandEqual => Self::Assignment(AssignmentOp::BitwiseAndAssign),
            Punctuator::PipeEqual => Self::Assignment(AssignmentOp::BitwiseOrAssign),
            Punctuator::CaretEqual => Self::Assignment(AssignmentOp::BitwiseXOrAssign),

            Punctuator::Plus => Self::Binary(BinaryOp::Add),
            Punctuator::Slash => Self::Binary(BinaryOp::Div),
            Punctuator::Percent => Self::Binary(BinaryOp::Mod),
            Punctuator::Asterisk => Self::Binary(BinaryOp::Mul),
            Punctuator::DoubleAsterisk => Self::Binary(BinaryOp::Pow),
            Punctuator::Minus => Self::Binary(BinaryOp::Sub),
            Punctuator::DoubleEqual => Self::Binary(BinaryOp::Equal),
            Punctuator::BangEqual => Self::Binary(BinaryOp::NotEqual),
            Punctuator::TripleEqual => Self::Binary(BinaryOp::Identical),
            Punctuator::BangDoubleEqual => Self::Binary(BinaryOp::NotIdentical),
            Punctuator::LessThan => Self::Binary(BinaryOp::LessThan),
            Punctuator::LessThanEqual => Self::Binary(BinaryOp::LessThanOrEqual),
            Punctuator::MoreThan => Self::Binary(BinaryOp::MoreThan),
            Punctuator::MoreThanEqual => Self::Binary(BinaryOp::MoreThanOrEqual),
            Punctuator::DoubleLessThan => Self::Binary(BinaryOp::ShiftLeft),
            Punctuator::DoubleMoreThan => Self::Binary(BinaryOp::ShiftRight),
            Punctuator::TripleMoreThan => Self::Binary(BinaryOp::ShiftRightUnsigned),
            Punctuator::Ampersand => Self::Binary(BinaryOp::BitwiseAnd),
            Punctuator::Pipe => Self::Binary(BinaryOp::BitwiseOr),
            Punctuator::Caret => Self::Binary(BinaryOp::BitwiseXOr),
            Punctuator::DoubleAmpersand => Op::Binary(BinaryOp::LogicalAnd),
            Punctuator::DoublePipe => Self::Binary(BinaryOp::LogicalOr),

            _ => return Err(()),
        })
    }
}
