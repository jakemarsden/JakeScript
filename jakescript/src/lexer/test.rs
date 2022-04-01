use super::error::ErrorKind;
use super::Lexer;
use crate::token::*;
use fallible_iterator::FallibleIterator;
use std::assert_matches::assert_matches;

#[test]
fn tokenise_keywords() {
    for expected in Keyword::all() {
        let mut lexer = Lexer::for_str(expected.as_str());
        assert_eq!(lexer.next().unwrap(), Some(Element::new_keyword(*expected)));
        assert_eq!(lexer.next().unwrap(), None);
    }
}

#[test]
fn tokenise_punctuators() {
    for expected in Punctuator::all() {
        let mut lexer = Lexer::for_str(expected.as_str());
        assert_eq!(
            lexer.next().unwrap(),
            Some(Element::new_punctuator(*expected))
        );
        assert_eq!(lexer.next().unwrap(), None);
    }
}

#[test]
fn tokenise_string_literal() {
    use crate::token::StringLiteralKind::{DoubleQuoted, SingleQuoted};

    fn check_valid(source: &str, expected: &str, expected_kind: StringLiteralKind) {
        let mut lexer = Lexer::for_str(source);
        assert_eq!(
            lexer.next().unwrap(),
            Some(Element::new_literal(Literal::String(StringLiteral {
                kind: expected_kind,
                value: expected.to_owned(),
            })))
        );
        assert_eq!(lexer.next().unwrap(), None);
    }

    check_valid(r#""""#, r#""#, DoubleQuoted);
    check_valid(r#""hello, world!""#, r#"hello, world!"#, DoubleQuoted);
    check_valid(
        r#""hello, \"escaped quotes\"!""#,
        r#"hello, "escaped quotes"!"#,
        DoubleQuoted,
    );
    check_valid(
        r#""hello, back\\slash""#,
        r#"hello, back\slash"#,
        DoubleQuoted,
    );
    check_valid(r#""hello, \\\"\"\\\\""#, r#"hello, \""\\"#, DoubleQuoted);
    check_valid(r#""hello,\n\r\tworld""#, "hello,\n\r\tworld", DoubleQuoted);

    check_valid(r#"''"#, r#""#, SingleQuoted);
    check_valid(r#"'hello, world!'"#, r#"hello, world!"#, SingleQuoted);
    check_valid(
        r#"'hello, \'escaped quotes\'!'"#,
        r#"hello, 'escaped quotes'!"#,
        SingleQuoted,
    );
    check_valid(
        r#"'hello, back\\slash'"#,
        r#"hello, back\slash"#,
        SingleQuoted,
    );
    check_valid(r#"'hello, \\\'\'\\\\'"#, r#"hello, \''\\"#, SingleQuoted);
    check_valid(r#"'hello,\n\r\tworld'"#, "hello,\n\r\tworld", SingleQuoted);
}

#[test]
fn tokenise_unclosed_multi_line_comment() {
    let source_code = "/* abc";
    let mut lexer = Lexer::for_str(source_code);
    assert_matches!(lexer.next(), Err(err) if err.kind() == Some(ErrorKind::UnclosedComment));
    assert_matches!(lexer.next(), Err(err) if err.kind() == Some(ErrorKind::UnclosedComment));
}
