use super::error::ErrorKind;
use super::Lexer;
use crate::token::*;
use fallible_iterator::FallibleIterator;
use std::assert_matches::assert_matches;

#[test]
fn tokenise_keywords() {
    for expected in Keyword::all() {
        let mut lexer = Lexer::for_str(expected.as_str());
        assert_matches!(
            lexer.next(),
            Ok(Some(Element::Token(Token::Keyword(actual)))) if actual == *expected
        );
        assert_matches!(lexer.next(), Ok(None));
    }
}

#[test]
fn tokenise_punctuators() {
    for expected in Punctuator::all() {
        let mut lexer = Lexer::for_str(expected.as_str());
        assert_matches!(
            lexer.next(),
            Ok(Some(Element::Token(Token::Punctuator(actual)))) if actual == *expected
        );
        assert_matches!(lexer.next(), Ok(None));
    }
}

#[test]
fn tokenise_string_literal() {
    fn check_valid(source: &str, expected: &str, single_qt: bool) {
        let mut lexer = Lexer::for_str(source);
        if single_qt {
            assert_matches!(
                lexer.next(),
                Ok(Some(Element::Token(Token::Literal(Literal::String(
                    StringLiteral::SingleQuoted(actual)
                ))))) if actual == expected
            );
        } else {
            assert_matches!(
                lexer.next(),
                Ok(Some(Element::Token(Token::Literal(Literal::String(
                    StringLiteral::DoubleQuoted(actual)
                ))))) if actual == expected
            );
        }
        assert_matches!(lexer.next(), Ok(None));
    }

    check_valid(r#""""#, r#""#, false);
    check_valid(r#""hello, world!""#, r#"hello, world!"#, false);
    check_valid(
        r#""hello, \"escaped quotes\"!""#,
        r#"hello, "escaped quotes"!"#,
        false,
    );
    check_valid(r#""hello, back\\slash""#, r#"hello, back\slash"#, false);
    check_valid(r#""hello, \\\"\"\\\\""#, r#"hello, \""\\"#, false);
    check_valid(r#""hello,\n\r\tworld""#, "hello,\n\r\tworld", false);

    check_valid(r#"''"#, r#""#, true);
    check_valid(r#"'hello, world!'"#, r#"hello, world!"#, true);
    check_valid(
        r#"'hello, \'escaped quotes\'!'"#,
        r#"hello, 'escaped quotes'!"#,
        true,
    );
    check_valid(r#"'hello, back\\slash'"#, r#"hello, back\slash"#, true);
    check_valid(r#"'hello, \\\'\'\\\\'"#, r#"hello, \''\\"#, true);
    check_valid(r#"'hello,\n\r\tworld'"#, "hello,\n\r\tworld", true);
}

#[test]
fn tokenise_unclosed_multi_line_comment() {
    let source_code = "/* abc";
    let mut lexer = Lexer::for_str(source_code);
    assert_matches!(lexer.next(), Err(err) if err.kind() == Some(ErrorKind::UnclosedComment));
    assert_matches!(lexer.next(), Ok(None));
}
