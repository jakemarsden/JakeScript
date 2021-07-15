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
///     Token::Punctuator(Punctuator::Semicolon),
/// ];
/// let mut parser = Parser::for_tokens(source);
/// assert_eq!(
///     parser.execute(),
///     Program(vec![BlockItem::Statement(Statement::Expression(
///         Expression::BinaryOp(
///             BinaryOp::Add,
///             Box::new(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(100)
///             ))),
///             Box::new(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(50)
///             )))
///         )
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
///         BlockItem::Declaration(Declaration::Variable(
///             VariableDeclKind::Let,
///             "a".to_owned(),
///             Some(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(100)
///             )))
///         )),
///         BlockItem::Declaration(Declaration::Variable(
///             VariableDeclKind::Let,
///             "b".to_owned(),
///             Some(Expression::Member(MemberExpression::Literal(
///                 ast::Literal::Numeric(50)
///             )))
///         )),
///         BlockItem::Statement(Statement::Expression(Expression::BinaryOp(
///             BinaryOp::Add,
///             Box::new(Expression::Member(MemberExpression::Identifier(
///                 "a".to_owned()
///             ))),
///             Box::new(Expression::Member(MemberExpression::Identifier(
///                 "b".to_owned()
///             )))
///         )))
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
            None => Some(lhs),
            Some(_) => self
                .parse_binary_expression(lhs)
                .map(|(kind, lhs, rhs)| Expression::BinaryOp(kind, lhs, rhs)),
        }
    }

    fn parse_binary_expression(
        &mut self,
        lhs: Expression,
    ) -> Option<(BinaryOp, Box<Expression>, Box<Expression>)> {
        let op = match self.0.consume()? {
            Token::Punctuator(punctuator) => match punctuator {
                Punctuator::Plus => BinaryOp::Add,
                Punctuator::Slash => BinaryOp::Div,
                Punctuator::Percent => BinaryOp::Mod,
                Punctuator::Asterisk => BinaryOp::Mul,
                Punctuator::DoubleAsterisk => BinaryOp::Pow,
                Punctuator::Minus => BinaryOp::Sub,
                Punctuator::DoubleEqual => BinaryOp::Equal,
                Punctuator::BangEqual => BinaryOp::NotEqual,
                Punctuator::TripleEqual => BinaryOp::Identical,
                Punctuator::BangDoubleEqual => BinaryOp::NotIdentical,
                Punctuator::LessThan => BinaryOp::LessThan,
                Punctuator::LessThanEqual => BinaryOp::LessThanOrEqual,
                Punctuator::MoreThan => BinaryOp::MoreThan,
                Punctuator::MoreThanEqual => BinaryOp::MoreThanOrEqual,
                Punctuator::DoubleLessThan => BinaryOp::ShiftLeft,
                Punctuator::DoubleMoreThan => BinaryOp::ShiftRight,
                Punctuator::TripleMoreThan => BinaryOp::ShiftRightUnsigned,
                Punctuator::Ampersand => BinaryOp::BitwiseAnd,
                Punctuator::Pipe => BinaryOp::BitwiseOr,
                Punctuator::Caret => BinaryOp::BitwiseXOr,
                Punctuator::DoubleAmpersand => BinaryOp::LogicalAnd,
                Punctuator::DoublePipe => BinaryOp::LogicalOr,
                token => panic!("Expected punctuator but was {}", token),
            },
            token => panic!("Expected punctuator but was {}", token),
        };

        let rhs = self
            .parse_expression()
            .expect("Expected rhs of binary expression but was <end>");
        Some((op, Box::new(lhs), Box::new(rhs)))
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        match self.0.peek()? {
            Token::Keyword(Keyword::Const | Keyword::Let) => self
                .parse_variable_decl()
                .map(|(kind, name, initialiser)| Declaration::Variable(kind, name, initialiser)),
            token => todo!("parse_declaration: {}", token),
        }
    }

    fn parse_variable_decl(
        &mut self,
    ) -> Option<(VariableDeclKind, IdentifierName, Option<Expression>)> {
        let decl_kind = match self.0.consume() {
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
            Some((decl_kind, var_name, initialiser))
        } else {
            panic!("Expected variable name");
        }
    }
}
