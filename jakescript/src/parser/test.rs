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

macro_rules! at {
    [$loc:ident @ $line:literal : $col:literal] => {
        $loc.at($crate::token::SourcePosition::at($line, $col))
    };
}

macro_rules! line_terminator {
    [$value:ident, $loc:expr] => {
        $crate::token::Element::new_line_terminator(
            $crate::token::LineTerminator::$value,
            $loc
        )
    };
}

macro_rules! whitespace {
    [$value:literal, $loc:expr] => {
        $crate::token::Element::new_whitespace(
            $crate::token::Whitespace::from(non_empty_str!($value)),
            $loc
        )
    };
}

mod simple {
    use super::*;

    #[test]
    fn parse_simple() {
        let parser = Parser::for_elements(
            source_elements()
                .into_iter()
                .filter(|elem| elem.line_terminator().is_none() && elem.whitespace().is_none()),
        );
        assert_eq!(parser.execute().unwrap(), expected());
    }

    #[test]
    fn parse_simple_with_whitespace() {
        let parser = Parser::for_elements(source_elements().into_iter());
        assert_eq!(parser.execute().unwrap(), expected());
    }

    fn source_elements() -> Vec<Element> {
        let loc = SourceLocation::at_start_of("test");
        vec![
            Element::new_identifier(non_empty_str!("square"), at![loc@0:0]),
            Element::new_punctuator(OpenParen, at![loc@0:6]),
            Element::new_literal(
                token::Literal::Numeric(token::NumericLiteral::DecInt(4)),
                at![loc@0:7],
            ),
            Element::new_punctuator(CloseParen, at![loc@0:8]),
            Element::new_punctuator(Semi, at![loc@0:9]),
            line_terminator!(Lf, at![loc@0:10]),
            line_terminator!(Lf, at![loc@1:0]),
            Element::new_keyword(Function, at![loc@2:0]),
            whitespace!(" ", at![loc@2:8]),
            Element::new_identifier(non_empty_str!("square"), at![loc@2:9]),
            Element::new_punctuator(OpenParen, at![loc@2:15]),
            Element::new_identifier(non_empty_str!("n"), at![loc@2:16]),
            Element::new_punctuator(CloseParen, at![loc@2:17]),
            whitespace!(" ", at![loc@2:18]),
            Element::new_punctuator(OpenBrace, at![loc@2:19]),
            line_terminator!(Lf, at![loc@2:20]),
            whitespace!("    ", at![loc@3:0]),
            Element::new_keyword(Return, at![loc@3:4]),
            whitespace!(" ", at![loc@3:10]),
            Element::new_identifier(non_empty_str!("n"), at![loc@3:11]),
            whitespace!(" ", at![loc@3:12]),
            Element::new_punctuator(StarStar, at![loc@3:13]),
            whitespace!(" ", at![loc@3:15]),
            Element::new_literal(
                token::Literal::Numeric(token::NumericLiteral::DecInt(2)),
                at![loc@3:16],
            ),
            Element::new_punctuator(Semi, at![loc@3:17]),
            line_terminator!(Lf, at![loc@3:18]),
            Element::new_punctuator(CloseBrace, at![loc@4:0]),
            line_terminator!(Lf, at![loc@4:1]),
            line_terminator!(Lf, at![loc@5:0]),
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
    let loc = SourceLocation::at_start_of("test");
    let source = vec![
        Element::new_keyword(While, at![loc@0:0]),
        Element::new_punctuator(OpenParen, at![loc@0:5]),
        Element::new_literal(token::Literal::Boolean(true), at![loc@0:6]),
        Element::new_punctuator(CloseParen, at![loc@0:10]),
        Element::new_punctuator(OpenBrace, at![loc@0:11]),
    ];

    let parser = Parser::for_elements(source.into_iter());
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedEoi(AllowToken::Exactly(Token::Punctuator(CloseBrace)))
        )
    );
}

#[test]
fn parse_unclosed_paren() {
    let loc = SourceLocation::at_start_of("test");
    let source = vec![
        Element::new_keyword(While, at![loc@0:0]),
        Element::new_punctuator(OpenParen, at![loc@0:5]),
        Element::new_literal(token::Literal::Boolean(true), at![loc@0:6]),
        Element::new_punctuator(OpenBrace, at![loc@0:10]),
    ];

    let parser = Parser::for_elements(source.into_iter());
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(AllowToken::Exactly(Token::Punctuator(CloseParen)), actual)
            if actual == &Element::new_punctuator(OpenBrace, at![loc@0:10])
        )
    );
}

#[test]
fn parse_unfinished_variable_decl() {
    let loc = SourceLocation::at_start_of("test");
    let source = vec![
        Element::new_keyword(Let, at![loc@0:0]),
        Element::new_punctuator(Semi, at![loc@0:3]),
    ];

    let parser = Parser::for_elements(source.into_iter());
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(AllowToken::Exactly(Token::Identifier(_)), actual)
            if actual == &Element::new_punctuator(Semi, at![loc@0:3])
        )
    );
}

#[test]
fn parse_unfinished_binary_expression() {
    let loc = SourceLocation::at_start_of("test");
    let source = vec![
        Element::new_keyword(Let, at![loc@0:0]),
        Element::new_identifier(non_empty_str!("a"), at![loc@0:3]),
        Element::new_punctuator(Eq, at![loc@0:4]),
        Element::new_literal(
            token::Literal::Numeric(token::NumericLiteral::DecInt(1)),
            at![loc@0:5],
        ),
        Element::new_punctuator(Plus, at![loc@0:6]),
        Element::new_literal(
            token::Literal::Numeric(token::NumericLiteral::DecInt(2)),
            at![loc@0:7],
        ),
        Element::new_punctuator(Plus, at![loc@0:8]),
        Element::new_punctuator(Semi, at![loc@0:9]),
    ];

    let parser = Parser::for_elements(source.into_iter());
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(AllowToken::Unspecified, actual)
            if actual == &Element::new_punctuator(Semi, at![loc@0:9])
        )
    );
}
