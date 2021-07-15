use crate::ast::*;
use crate::lexer::*;
use crate::util::Stream;
use std::iter::Iterator;

pub struct Parser(Stream<Token>);

impl Parser {
    pub fn for_tokens(source: Vec<Token>) -> Self {
        Self::new(source.into_iter())
    }

    pub fn new(source: impl Iterator<Item = Token>) -> Self {
        Self(Stream::new(source))
    }
}

/// ```rust
/// # use jakescript::ast::*;
/// # use jakescript::lexer::*;
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
///     Program::new(Block::new(vec![Node::BinaryOp(
///         BinaryOp::Add,
///         Box::new((
///             Node::Constant(Constant::Numeric(100)),
///             Node::Constant(Constant::Numeric(50))
///         ))
///     ),]))
/// );
/// ```
///
/// ```rust
/// # use jakescript::ast::*;
/// # use jakescript::lexer::*;
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
///         Node::LocalVarDecl(
///             "a".to_owned(),
///             Some(Box::new(Node::Constant(Constant::Numeric(100))))
///         ),
///         Node::LocalVarDecl(
///             "b".to_owned(),
///             Some(Box::new(Node::Constant(Constant::Numeric(50))))
///         ),
///         Node::BinaryOp(
///             BinaryOp::Add,
///             Box::new((Node::Local("a".to_owned()), Node::Local("b".to_owned()),))
///         ),
///     ]))
/// );
/// ```
///
/// ```rust
/// # use jakescript::ast::*;
/// # use jakescript::lexer::*;
/// # use jakescript::parser::*;
/// let source = vec![
///     Token::Keyword(Keyword::Let),
///     Token::Identifier("x".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Literal(Literal::Numeric(0)),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Keyword(Keyword::While),
///     Token::Identifier("x".to_owned()),
///     Token::Punctuator(Punctuator::LessThan),
///     Token::Literal(Literal::Numeric(3)),
///     Token::Punctuator(Punctuator::OpenBrace),
///     Token::Identifier("x".to_owned()),
///     Token::Punctuator(Punctuator::Equal),
///     Token::Identifier("x".to_owned()),
///     Token::Punctuator(Punctuator::Plus),
///     Token::Literal(Literal::Numeric(1)),
///     Token::Punctuator(Punctuator::Semicolon),
///     Token::Punctuator(Punctuator::CloseBrace),
/// ];
/// let mut parser = Parser::for_tokens(source);
/// assert_eq!(
///     parser.execute(),
///     Program::new(Block::new(vec![
///         Node::LocalVarDecl(
///             "x".to_owned(),
///             Some(Box::new(Node::Constant(Constant::Numeric(0))))
///         ),
///         Node::While(
///             Box::new(Node::BinaryOp(
///                 BinaryOp::LessThan,
///                 Box::new((
///                     Node::Local("x".to_owned()),
///                     Node::Constant(Constant::Numeric(3))
///                 ))
///             )),
///             Block::new(vec![Node::BinaryOp(
///                 BinaryOp::Assign,
///                 Box::new((
///                     Node::Local("x".to_owned()),
///                     Node::BinaryOp(
///                         BinaryOp::Add,
///                         Box::new((
///                             Node::Local("x".to_owned()),
///                             Node::Constant(Constant::Numeric(1))
///                         ))
///                     )
///                 ))
///             ),])
///         ),
///     ]))
/// );
/// ```
impl Parser {
    pub fn execute(mut self) -> Program {
        let mut block = Vec::new();
        loop {
            // TODO: Check correctness of semicolons
            self.0
                .consume_if(|it| matches!(it, Token::Punctuator(Punctuator::Semicolon)));
            block.push(match self.0.peek() {
                Some(Token::Keyword(Keyword::Let)) => self.parse_variable_decl(),
                Some(Token::Keyword(Keyword::While)) => self.parse_while(),
                Some(_) => self.parse_expression(),
                None => break,
            });
        }
        Program::new(Block::new(block))
    }

    fn parse_expression(&mut self) -> Node {
        let lhs = match self.0.consume() {
            Some(Token::Identifier(name)) => Node::Local(name),
            Some(Token::Literal(literal)) => Node::Constant(match literal {
                Literal::Boolean(it) => Constant::Boolean(it),
                Literal::Null => Constant::Null,
                Literal::Numeric(it) => Constant::Numeric(it),
                Literal::String(it) => Constant::String(it),
            }),
            token => todo!("token: {:?}", token),
        };
        if self
            .0
            .consume_if(|it| it == &Token::Punctuator(Punctuator::Semicolon))
            .is_some()
        {
            lhs
        } else {
            self.try_parse_binary_operator(lhs)
        }
    }

    fn try_parse_binary_operator(&mut self, lhs: Node) -> Node {
        match self.0.peek() {
            Some(Token::Punctuator(Punctuator::OpenBrace | Punctuator::Semicolon)) => return lhs,
            Some(Token::Punctuator(_)) => {}
            _ => return lhs,
        }
        match self.0.consume().unwrap() {
            Token::Punctuator(punctuator) => {
                let op = match punctuator {
                    Punctuator::Equal => BinaryOp::Assign,
                    Punctuator::LessThan => BinaryOp::LessThan,
                    Punctuator::Plus => BinaryOp::Add,
                    punctuator => todo!("punctuator: {}", punctuator),
                };
                let rhs = self.parse_expression();
                Node::BinaryOp(op, Box::new((lhs, rhs)))
            }
            token => unreachable!("{:?}", token),
        }
    }

    fn parse_while(&mut self) -> Node {
        if !matches!(self.0.consume(), Some(Token::Keyword(Keyword::While))) {
            return self.invalid(ParseError);
        }

        let condition = self.parse_expression();

        if !matches!(
            self.0.consume(),
            Some(Token::Punctuator(Punctuator::OpenBrace))
        ) {
            return self.invalid(ParseError);
        }

        let mut body = Vec::new();
        while self
            .0
            .consume_if(|it| it == &Token::Punctuator(Punctuator::CloseBrace))
            .is_none()
        {
            body.push(self.parse_expression());
        }

        Node::While(Box::new(condition), Block::new(body))
    }

    fn parse_variable_decl(&mut self) -> Node {
        match self.0.consume() {
            Some(Token::Keyword(Keyword::Let)) => {}
            token => todo!("token: {:?}", token),
        };

        let identifier = if let Some(Token::Identifier(identifier)) = self.0.consume() {
            identifier
        } else {
            return self.invalid(ParseError);
        };

        let initialiser = match self.0.consume() {
            Some(Token::Punctuator(Punctuator::Equal)) => {
                let expr = self.parse_expression();
                Some(Box::new(expr))
            }
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            _ => return self.invalid(ParseError),
        };

        Node::LocalVarDecl(identifier, initialiser)
    }

    fn invalid(&mut self, err: ParseError) -> Node {
        self.0
            .consume_until(|t| matches!(t, Token::Punctuator(Punctuator::Semicolon)));
        self.0
            .consume_if(|t| matches!(t, Token::Punctuator(Punctuator::Semicolon)));
        Node::Invalid(err)
    }
}
