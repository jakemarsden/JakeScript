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

    /// ```rust
    /// # use jakescript::ast::*;
    /// # use jakescript::lexer::*;
    /// # use jakescript::parser::*;
    /// let source = vec![
    ///     Token::Literal(Literal::Integer(100)),
    ///     Token::Symbol(Symbol::Plus),
    ///     Token::Literal(Literal::Integer(50)),
    ///     Token::Symbol(Symbol::Semicolon),
    /// ];
    /// let mut parser = Parser::for_tokens(source);
    /// assert_eq!(
    ///     parser.execute(),
    ///     Program::new(vec![Node::BinaryOp(
    ///         BinaryOp::Add,
    ///         Box::new((
    ///             Node::Constant(Constant::Integer(100)),
    ///             Node::Constant(Constant::Integer(50))
    ///         ))
    ///     ),])
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
    ///     Token::Symbol(Symbol::Equal),
    ///     Token::Literal(Literal::Integer(100)),
    ///     Token::Symbol(Symbol::Semicolon),
    ///     Token::Keyword(Keyword::Let),
    ///     Token::Identifier("b".to_owned()),
    ///     Token::Symbol(Symbol::Equal),
    ///     Token::Literal(Literal::Integer(50)),
    ///     Token::Symbol(Symbol::Semicolon),
    ///     Token::Identifier("a".to_owned()),
    ///     Token::Symbol(Symbol::Plus),
    ///     Token::Identifier("b".to_owned()),
    ///     Token::Symbol(Symbol::Semicolon),
    /// ];
    /// let mut parser = Parser::for_tokens(source);
    /// assert_eq!(
    ///     parser.execute(),
    ///     Program::new(vec![
    ///         Node::LocalVarDecl(
    ///             "a".to_owned(),
    ///             Some(Box::new(Node::Constant(Constant::Integer(100))))
    ///         ),
    ///         Node::LocalVarDecl(
    ///             "b".to_owned(),
    ///             Some(Box::new(Node::Constant(Constant::Integer(50))))
    ///         ),
    ///         Node::BinaryOp(
    ///             BinaryOp::Add,
    ///             Box::new((Node::Local("a".to_owned()), Node::Local("b".to_owned()),))
    ///         ),
    ///     ])
    /// );
    /// ```
    pub fn execute(mut self) -> Program {
        let mut block = Vec::new();
        loop {
            // TODO: Check correctness of semicolons
            self.0
                .consume_if(|it| matches!(it, Token::Symbol(Symbol::Semicolon)));
            block.push(match self.0.peek() {
                Some(Token::Keyword(Keyword::Let)) => self.parse_variable_decl(),
                Some(_) => self.parse_expression(),
                None => break,
            });
        }
        Program::new(block)
    }

    fn parse_expression(&mut self) -> Node {
        let lhs = match self.0.consume() {
            Some(Token::Identifier(name)) => Node::Local(name),
            Some(Token::Literal(literal)) => Node::Constant(match literal {
                Literal::Character(it) => Constant::Character(it),
                Literal::Integer(it) => Constant::Integer(it),
                Literal::String(it) => Constant::String(it),
            }),
            token => todo!("token: {:?}", token),
        };
        self.try_parse_binary_operator(lhs)
    }

    fn try_parse_binary_operator(&mut self, lhs: Node) -> Node {
        match self.0.peek() {
            Some(Token::Symbol(Symbol::Semicolon)) => return lhs,
            Some(Token::Symbol(_)) => {}
            _ => return lhs,
        }
        match self.0.consume().unwrap() {
            Token::Symbol(symbol) => {
                let op = match symbol {
                    Symbol::Plus => BinaryOp::Add,
                    Symbol::Equal | Symbol::Semicolon => return self.invalid(ParseError),
                };
                let rhs = self.parse_expression();
                Node::BinaryOp(op, Box::new((lhs, rhs)))
            }
            token => unreachable!("{:?}", token),
        }
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
            Some(Token::Symbol(Symbol::Equal)) => {
                let expr = self.parse_expression();
                Some(Box::new(expr))
            }
            Some(Token::Symbol(Symbol::Semicolon)) => None,
            _ => return self.invalid(ParseError),
        };

        Node::LocalVarDecl(identifier, initialiser)
    }

    fn invalid(&mut self, err: ParseError) -> Node {
        self.0
            .consume_until(|t| matches!(t, Token::Symbol(Symbol::Semicolon)));
        self.0
            .consume_if(|t| matches!(t, Token::Symbol(Symbol::Semicolon)));
        Node::Invalid(err)
    }
}
