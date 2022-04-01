use super::error::{AllowToken, ErrorKind};
use super::Parser;
use crate::non_empty_str;
use crate::token::*;
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
    let parser = Parser::for_elements(tokens.into_iter().map(Element::Token));
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedEoi(
                AllowToken::Exactly(Token::Punctuator(Punctuator::CloseBrace))
            )
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
    let parser = Parser::for_elements(tokens.into_iter().map(Element::Token));
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(
                AllowToken::Exactly(Token::Punctuator(Punctuator::CloseParen)),
                Element::Token(Token::Punctuator(Punctuator::OpenBrace))
            )
        )
    );
}

#[test]
fn parse_unfinished_variable_decl() {
    let tokens = vec![
        Token::Keyword(Keyword::Let),
        Token::Punctuator(Punctuator::Semi),
    ];
    let parser = Parser::for_elements(tokens.into_iter().map(Element::Token));
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(
                AllowToken::Exactly(Token::Identifier(_)),
                Element::Token(Token::Punctuator(Punctuator::Semi))
            )
        )
    );
}

#[test]
fn parse_unfinished_binary_expression() {
    let tokens = vec![
        Token::Keyword(Keyword::Let),
        Token::Identifier(non_empty_str!("a")),
        Token::Punctuator(Punctuator::Eq),
        Token::Literal(Literal::Numeric(NumericLiteral::DecInt(1))),
        Token::Punctuator(Punctuator::Plus),
        Token::Literal(Literal::Numeric(NumericLiteral::DecInt(2))),
        Token::Punctuator(Punctuator::Plus),
        Token::Punctuator(Punctuator::Semi),
    ];
    let parser = Parser::for_elements(tokens.into_iter().map(Element::Token));
    assert_matches!(
        parser.execute(),
        Err(err) if matches!(
            err.kind(),
            ErrorKind::UnexpectedToken(
                AllowToken::Unspecified, Element::Token(Token::Punctuator(Punctuator::Semi))
            )
        )
    );
}
