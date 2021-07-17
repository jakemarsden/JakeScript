use crate::ast::{self, *};
use crate::lexer::{Literal, *};
use crate::util::Stream;
use std::iter::Iterator;

pub struct Parser(Stream<Token>);

impl Parser {
    pub fn for_lexer(source: Lexer) -> Self {
        Self::new(source.tokens())
    }

    pub fn for_tokens(source: Vec<Token>) -> Self {
        Self::new(source.into_iter())
    }

    pub fn new(source: impl Iterator<Item = Token>) -> Self {
        Self(Stream::new(source))
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
///     Program(vec![BlockItem::Statement(Statement::Expression(
///         Expression::BinaryOp {
///             kind: BinaryOp::Add,
///             lhs: Box::new(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(100)
///             ))),
///             rhs: Box::new(Expression::BinaryOp {
///                 kind: BinaryOp::Add,
///                 lhs: Box::new(Expression::Member(MemberExpression::Literal(
///                     ast::Literal::Numeric(50)
///                 ))),
///                 rhs: Box::new(Expression::Member(MemberExpression::Literal(
///                     ast::Literal::Numeric(17)
///                 ))),
///             }),
///         }
///     )),])
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
///     Program(vec![
///         BlockItem::Declaration(Declaration::Variable {
///             kind: VariableDeclKind::Let,
///             var_name: "a".to_owned(),
///             initialiser: Some(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(100)
///             )))
///         }),
///         BlockItem::Declaration(Declaration::Variable {
///             kind: VariableDeclKind::Let,
///             var_name: "b".to_owned(),
///             initialiser: Some(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(50)
///             )))
///         }),
///         BlockItem::Statement(Statement::Expression(Expression::BinaryOp {
///             kind: BinaryOp::Add,
///             lhs: Box::new(Expression::Member(MemberExpression::Identifier(
///                 "a".to_owned()
///             ))),
///             rhs: Box::new(Expression::Member(MemberExpression::Identifier(
///                 "b".to_owned()
///             )))
///         }))
///     ])
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
///     Program(vec![
///         BlockItem::Declaration(Declaration::Variable {
///             kind: VariableDeclKind::Let,
///             var_name: "a".to_owned(),
///             initialiser: Some(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(100)
///             )))
///         }),
///         BlockItem::Declaration(Declaration::Variable {
///             kind: VariableDeclKind::Let,
///             var_name: "b".to_owned(),
///             initialiser: None,
///         }),
///         BlockItem::Statement(Statement::If {
///             condition: Expression::BinaryOp {
///                 kind: BinaryOp::MoreThanOrEqual,
///                 lhs: Box::new(Expression::Member(MemberExpression::Identifier(
///                     "a".to_owned()
///                 ))),
///                 rhs: Box::new(Expression::Member(MemberExpression::Literal(
///                     ast::Literal::Numeric(3)
///                 )))
///             },
///             success_block: vec![BlockItem::Statement(Statement::Expression(
///                 Expression::AssignmentOp {
///                     kind: AssignmentOp::Assign,
///                     lhs: MemberExpression::Identifier("b".to_owned()),
///                     rhs: Box::new(Expression::Member(MemberExpression::Literal(
///                         ast::Literal::String("success block!".to_owned())
///                     ))),
///                 }
///             )),],
///             else_block: Some(vec![BlockItem::Statement(Statement::Expression(
///                 Expression::AssignmentOp {
///                     kind: AssignmentOp::Assign,
///                     lhs: MemberExpression::Identifier("b".to_owned()),
///                     rhs: Box::new(Expression::Member(MemberExpression::Literal(
///                         ast::Literal::String("else block!".to_owned())
///                     ))),
///                 }
///             )),])
///         }),
///     ])
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
///     Program(vec![
///         BlockItem::Declaration(Declaration::Variable {
///             kind: VariableDeclKind::Let,
///             var_name: "a".to_owned(),
///             initialiser: Some(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(3)
///             )))
///         }),
///         BlockItem::Statement(Statement::WhileLoop {
///             condition: Expression::BinaryOp {
///                 kind: BinaryOp::NotIdentical,
///                 lhs: Box::new(Expression::Member(MemberExpression::Identifier(
///                     "a".to_owned()
///                 ))),
///                 rhs: Box::new(Expression::Member(MemberExpression::Literal(
///                     ast::Literal::Numeric(0)
///                 )))
///             },
///             block: vec![BlockItem::Statement(Statement::Expression(
///                 Expression::AssignmentOp {
///                     kind: AssignmentOp::Assign,
///                     lhs: MemberExpression::Identifier("a".to_owned()),
///                     rhs: Box::new(Expression::BinaryOp {
///                         kind: BinaryOp::Sub,
///                         lhs: Box::new(Expression::Member(MemberExpression::Identifier(
///                             "a".to_string()
///                         ))),
///                         rhs: Box::new(Expression::Member(MemberExpression::Literal(
///                             ast::Literal::Numeric(1)
///                         ))),
///                     }),
///                 }
///             )),],
///         }),
///     ])
/// );
/// ```
impl Parser {
    pub fn execute(mut self) -> Program {
        let mut block = Vec::new();
        while let Some(block_item) = self.parse_block_item() {
            block.push(block_item);
        }
        Program(block)
    }

    fn parse_block(&mut self) -> Vec<BlockItem> {
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::OpenBrace));
        let mut block = Vec::new();
        while !matches!(
            self.0.peek(),
            Some(Token::Punctuator(Punctuator::CloseBrace))
        ) {
            if let Some(block_item) = self.parse_block_item() {
                block.push(block_item);
            } else {
                // Block not closed before end of input
                break;
            }
        }
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::CloseBrace));
        block
    }

    fn parse_block_item(&mut self) -> Option<BlockItem> {
        match self.0.peek()? {
            Token::Keyword(Keyword::Const | Keyword::Let) => {
                self.parse_declaration().map(BlockItem::Declaration)
            }
            _ => self.parse_statement().map(BlockItem::Statement),
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.0.peek()? {
            Token::Punctuator(Punctuator::OpenBrace) => Some(Statement::Block(self.parse_block())),
            Token::Keyword(Keyword::If) => {
                self.parse_if_statement()
                    .map(|(condition, success_block, else_block)| Statement::If {
                        condition,
                        success_block,
                        else_block,
                    })
            }
            Token::Keyword(Keyword::While) => self
                .parse_while_statement()
                .map(|(condition, block)| Statement::WhileLoop { condition, block }),
            _ => self.parse_expression().map(Statement::Expression),
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        let lhs = match self.0.consume()? {
            Token::Identifier(name) => Expression::Member(MemberExpression::Identifier(name)),
            Token::Literal(literal) => {
                Expression::Member(MemberExpression::Literal(match literal {
                    Literal::Boolean(value) => ast::Literal::Boolean(value),
                    Literal::Null => ast::Literal::Null,
                    Literal::Numeric(value) => ast::Literal::Numeric(value),
                    Literal::String(value) => ast::Literal::String(value),
                }))
            }
            token => todo!("parse_expression: {}", token),
        };
        match self.0.peek() {
            Some(&Token::Punctuator(Punctuator::Semicolon)) => {
                self.0.advance();
                Some(lhs)
            }
            Some(&Token::Punctuator(Punctuator::CloseParen)) => {
                // TODO: Added to get if statement conditions to parse, but seems a bit
                // arbitrary/hackish
                Some(lhs)
            }
            None => Some(lhs),
            Some(_) => self.parse_secondary_expression(lhs),
        }
    }

    fn parse_secondary_expression(&mut self, lhs: Expression) -> Option<Expression> {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        enum Op {
            Assignment(AssignmentOp),
            Binary(BinaryOp),
        }

        let kind = match self.0.consume()? {
            Token::Punctuator(punctuator) => match punctuator {
                Punctuator::Equal => Op::Assignment(AssignmentOp::Assign),
                Punctuator::PlusEqual => Op::Assignment(AssignmentOp::AddAssign),
                Punctuator::SlashEqual => Op::Assignment(AssignmentOp::DivAssign),
                Punctuator::PercentEqual => Op::Assignment(AssignmentOp::ModAssign),
                Punctuator::AsteriskEqual => Op::Assignment(AssignmentOp::MulAssign),
                Punctuator::DoubleAsteriskEqual => Op::Assignment(AssignmentOp::PowAssign),
                Punctuator::MinusEqual => Op::Assignment(AssignmentOp::SubAssign),
                Punctuator::DoubleLessThanEqual => Op::Assignment(AssignmentOp::ShiftLeftAssign),
                Punctuator::DoubleMoreThanEqual => Op::Assignment(AssignmentOp::ShiftRightAssign),
                Punctuator::TripleMoreThanEqual => {
                    Op::Assignment(AssignmentOp::ShiftRightUnsignedAssign)
                }
                Punctuator::AmpersandEqual => Op::Assignment(AssignmentOp::BitwiseAndAssign),
                Punctuator::PipeEqual => Op::Assignment(AssignmentOp::BitwiseOrAssign),
                Punctuator::CaretEqual => Op::Assignment(AssignmentOp::BitwiseXOrAssign),

                Punctuator::Plus => Op::Binary(BinaryOp::Add),
                Punctuator::Slash => Op::Binary(BinaryOp::Div),
                Punctuator::Percent => Op::Binary(BinaryOp::Mod),
                Punctuator::Asterisk => Op::Binary(BinaryOp::Mul),
                Punctuator::DoubleAsterisk => Op::Binary(BinaryOp::Pow),
                Punctuator::Minus => Op::Binary(BinaryOp::Sub),
                Punctuator::DoubleEqual => Op::Binary(BinaryOp::Equal),
                Punctuator::BangEqual => Op::Binary(BinaryOp::NotEqual),
                Punctuator::TripleEqual => Op::Binary(BinaryOp::Identical),
                Punctuator::BangDoubleEqual => Op::Binary(BinaryOp::NotIdentical),
                Punctuator::LessThan => Op::Binary(BinaryOp::LessThan),
                Punctuator::LessThanEqual => Op::Binary(BinaryOp::LessThanOrEqual),
                Punctuator::MoreThan => Op::Binary(BinaryOp::MoreThan),
                Punctuator::MoreThanEqual => Op::Binary(BinaryOp::MoreThanOrEqual),
                Punctuator::DoubleLessThan => Op::Binary(BinaryOp::ShiftLeft),
                Punctuator::DoubleMoreThan => Op::Binary(BinaryOp::ShiftRight),
                Punctuator::TripleMoreThan => Op::Binary(BinaryOp::ShiftRightUnsigned),
                Punctuator::Ampersand => Op::Binary(BinaryOp::BitwiseAnd),
                Punctuator::Pipe => Op::Binary(BinaryOp::BitwiseOr),
                Punctuator::Caret => Op::Binary(BinaryOp::BitwiseXOr),
                Punctuator::DoubleAmpersand => Op::Binary(BinaryOp::LogicalAnd),
                Punctuator::DoublePipe => Op::Binary(BinaryOp::LogicalOr),

                token => panic!("Expected punctuator but was {}", token),
            },
            token => panic!("Expected punctuator but was {}", token),
        };

        let rhs = self
            .parse_expression()
            .expect("Expected rhs of binary expression but was <end>");
        Some(match kind {
            Op::Assignment(kind) => Expression::AssignmentOp {
                kind,
                lhs: if let Expression::Member(lhs) = lhs {
                    lhs
                } else {
                    panic!("Expected member expression but was {:?}", lhs)
                },
                rhs: Box::new(rhs),
            },
            Op::Binary(kind) => Expression::BinaryOp {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        })
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        match self.0.peek()? {
            Token::Keyword(Keyword::Const | Keyword::Let) => {
                self.parse_variable_decl()
                    .map(|(kind, var_name, initialiser)| Declaration::Variable {
                        kind,
                        var_name,
                        initialiser,
                    })
            }
            token => todo!("parse_declaration: {}", token),
        }
    }

    fn parse_variable_decl(
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

    fn parse_if_statement(
        &mut self,
    ) -> Option<(Expression, Vec<BlockItem>, Option<Vec<BlockItem>>)> {
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
        Some((condition, success_block, else_block))
    }

    fn parse_while_statement(&mut self) -> Option<(Expression, Vec<BlockItem>)> {
        self.0.consume_exact(&Token::Keyword(Keyword::While));
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::OpenParen));
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        self.0
            .consume_exact(&Token::Punctuator(Punctuator::CloseParen));
        let block = self.parse_block();
        Some((condition, block))
    }
}
