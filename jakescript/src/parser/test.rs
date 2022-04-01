use super::error::{AllowToken, ErrorKind};
use super::Parser;
use crate::ast::{self, *};
use crate::non_empty_str;
use crate::token::Keyword::{Function, Let, Return, While};
use crate::token::Punctuator::{
    CloseBrace, CloseParen, Eq, OpenBrace, OpenParen, Plus, Semi, StarStar,
};
use crate::token::{self, *};
use std::assert_matches::assert_matches;

mod simple {
    use super::*;

    #[test]
    fn parse_simple() {
        let parser = Parser::for_elements(
            source_elements()
                .into_iter()
                .filter(|elem| matches!(elem, Element::Token(..))),
        );
        assert_eq!(parser.execute().unwrap(), expected());
    }

    #[test]
    fn parse_simple_with_whitespace() {
        let parser = Parser::for_elements(source_elements().into_iter());
        assert_eq!(parser.execute().unwrap(), expected());
    }

    fn source_elements() -> Vec<Element> {
        let sp = Element::Whitespace(Whitespace::from(non_empty_str!(" ")));
        let indent = Element::Whitespace(Whitespace::from(non_empty_str!("    ")));
        let lf = Element::LineTerminator(LineTerminator::Lf);

        vec![
            Element::Token(Token::Identifier(non_empty_str!("square"))),
            Element::Token(Token::Punctuator(OpenParen)),
            Element::Token(Token::Literal(token::Literal::Numeric(
                token::NumericLiteral::DecInt(4),
            ))),
            Element::Token(Token::Punctuator(CloseParen)),
            Element::Token(Token::Punctuator(Semi)),
            lf.clone(),
            lf.clone(),
            Element::Token(Token::Keyword(Function)),
            sp.clone(),
            Element::Token(Token::Identifier(non_empty_str!("square"))),
            Element::Token(Token::Punctuator(OpenParen)),
            Element::Token(Token::Identifier(non_empty_str!("n"))),
            Element::Token(Token::Punctuator(CloseParen)),
            sp.clone(),
            Element::Token(Token::Punctuator(OpenBrace)),
            lf.clone(),
            indent,
            Element::Token(Token::Keyword(Return)),
            sp.clone(),
            Element::Token(Token::Identifier(non_empty_str!("n"))),
            sp.clone(),
            Element::Token(Token::Punctuator(StarStar)),
            sp,
            Element::Token(Token::Literal(token::Literal::Numeric(
                token::NumericLiteral::DecInt(2),
            ))),
            Element::Token(Token::Punctuator(Semi)),
            lf.clone(),
            Element::Token(Token::Punctuator(CloseBrace)),
            lf.clone(),
            lf,
        ]
    }

    fn expected() -> Script {
        Script::new(Block::new(
            vec![Declaration::Function(FunctionDeclaration {
                binding: Identifier::from(non_empty_str!("square")),
                formal_parameters: vec![Identifier::from(non_empty_str!("n"))],
                body: Block::new(
                    vec![],
                    vec![BlockItem::Statement(Statement::Return(ReturnStatement {
                        value: Some(Expression::Binary(BinaryExpression {
                            op: BinaryOperator::Exponentiation,
                            lhs: Box::new(Expression::IdentifierReference(
                                IdentifierReferenceExpression {
                                    identifier: Identifier::from(non_empty_str!("n")),
                                },
                            )),
                            rhs: Box::new(Expression::Literal(LiteralExpression {
                                value: ast::Literal::Numeric(ast::NumericLiteral::Int(2)),
                            })),
                        })),
                    }))],
                ),
            })],
            vec![BlockItem::Statement(Statement::Expression(
                ExpressionStatement {
                    expression: Expression::Member(MemberExpression::FunctionCall(
                        FunctionCallExpression {
                            function: Box::new(Expression::IdentifierReference(
                                IdentifierReferenceExpression {
                                    identifier: Identifier::from(non_empty_str!("square")),
                                },
                            )),
                            arguments: vec![Expression::Literal(LiteralExpression {
                                value: Literal::Numeric(NumericLiteral::Int(4)),
                            })],
                        },
                    )),
                },
            ))],
        ))
    }
}

#[test]
fn parse_unclosed_block() {
    let source = vec![
        Token::Keyword(While),
        Token::Punctuator(OpenParen),
        Token::Literal(token::Literal::Boolean(true)),
        Token::Punctuator(CloseParen),
        Token::Punctuator(OpenBrace),
    ];

    let parser = Parser::for_elements(source.into_iter().map(Element::Token));
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedEoi(
                AllowToken::Exactly(Token::Punctuator(CloseBrace))
            )
        )
    );
}

#[test]
fn parse_unclosed_paren() {
    let source = vec![
        Token::Keyword(While),
        Token::Punctuator(OpenParen),
        Token::Literal(token::Literal::Boolean(true)),
        Token::Punctuator(OpenBrace),
    ];

    let parser = Parser::for_elements(source.into_iter().map(Element::Token));
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(
                AllowToken::Exactly(Token::Punctuator(CloseParen)),
                Element::Token(Token::Punctuator(OpenBrace))
            )
        )
    );
}

#[test]
fn parse_unfinished_variable_decl() {
    let source = vec![Token::Keyword(Let), Token::Punctuator(Semi)];

    let parser = Parser::for_elements(source.into_iter().map(Element::Token));
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(
                AllowToken::Exactly(Token::Identifier(_)),
                Element::Token(Token::Punctuator(Semi))
            )
        )
    );
}

#[test]
fn parse_unfinished_binary_expression() {
    let source = vec![
        Token::Keyword(Let),
        Token::Identifier(non_empty_str!("a")),
        Token::Punctuator(Eq),
        Token::Literal(token::Literal::Numeric(token::NumericLiteral::DecInt(1))),
        Token::Punctuator(Plus),
        Token::Literal(token::Literal::Numeric(token::NumericLiteral::DecInt(2))),
        Token::Punctuator(Plus),
        Token::Punctuator(Semi),
    ];

    let parser = Parser::for_elements(source.into_iter().map(Element::Token));
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(
                AllowToken::Unspecified, Element::Token(Token::Punctuator(Semi))
            )
        )
    );
}
