use crate::util::{IntoPeekableNth, PeekableNth};
use std::iter::{FilterMap, Iterator};
use std::str::{Chars, FromStr};

pub use error::*;
pub use token::*;

mod error;
mod token;

pub type Tokens<I> = FilterMap<I, fn(Element) -> Option<Token>>;

pub struct Lexer<I: Iterator<Item = char>>(PeekableNth<I>);

impl<'a> Lexer<Chars<'a>> {
    pub fn for_str(source: &'a str) -> Self {
        Self::for_chars(source.chars())
    }
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn for_chars(source: I) -> Self {
        Self(source.peekable_nth())
    }

    pub fn tokens(self) -> Tokens<Self> {
        self.filter_map(Element::token)
    }

    fn parse_element(&mut self) -> Option<Element> {
        self.parse_whitespace()
            .map(Element::Whitespace)
            .or_else(|| self.parse_line_terminator().map(Element::LineTerminator))
            .or_else(|| self.parse_comment().map(Element::Comment))
            .or_else(|| self.parse_token().map(Element::Token))
    }

    fn parse_token(&mut self) -> Option<Token> {
        // TODO: Parse template tokens.
        self.parse_punctuator()
            .map(Token::Punctuator)
            .or_else(|| self.parse_literal().map(Token::Literal))
            .or_else(|| self.parse_keyword_or_identifier())
    }

    fn parse_punctuator(&mut self) -> Option<Punctuator> {
        Punctuator::VALUES_IN_LEXICAL_ORDER
            .iter()
            .find(|punctuator| self.0.consume_str(punctuator.into_str()))
            .cloned()
    }

    fn parse_literal(&mut self) -> Option<Literal> {
        self.parse_null_literal()
            .map(|()| Literal::Null)
            .or_else(|| self.parse_undefined_literal().map(|()| Literal::Undefined))
            .or_else(|| self.parse_boolean_literal().map(Literal::Boolean))
            .or_else(|| self.parse_numeric_literal().map(Literal::Numeric))
            .or_else(|| self.parse_string_literal().map(Literal::String))
    }

    fn parse_null_literal(&mut self) -> Option<()> {
        self.0.consume_str("null").then_some(())
    }

    fn parse_undefined_literal(&mut self) -> Option<()> {
        self.0.consume_str("undefined").then_some(())
    }

    fn parse_boolean_literal(&mut self) -> Option<bool> {
        self.0
            .consume_str("true")
            .then_some(true)
            .or_else(|| self.0.consume_str("false").then_some(false))
    }

    fn parse_numeric_literal(&mut self) -> Option<i64> {
        if !matches!(self.0.peek(), Some(ch) if ch.is_ascii_digit()) {
            return None;
        }
        // FIXME: This is a naive implementation which doesn't match the spec.
        let mut content = String::new();
        let mut original_len = 0;
        for offset in 0.. {
            match self.0.peek_n(offset) {
                Some(ch) if ch.is_ascii_digit() => {
                    content.push(*ch);
                    original_len += 1;
                }
                Some('_') => {
                    original_len += 1;
                }
                Some(_) | None => break,
            }
        }
        match self.0.peek_n(original_len) {
            Some(next_ch) if is_identifier_start(*next_ch) => return None,
            Some(next_ch) if next_ch.is_ascii_digit() => return None,
            Some(_) | None => {}
        }
        if let Ok(value) = i64::from_str(&content) {
            self.0.advance_n(original_len);
            Some(value)
        } else {
            todo!("Lexer::parse_numeric_literal: content={}", content)
        }
    }

    fn parse_string_literal(&mut self) -> Option<String> {
        ['"', '\'']
            .into_iter()
            .find_map(|qt| self.parse_quoted_literal(qt))
    }

    fn parse_quoted_literal(&mut self, qt: char) -> Option<String> {
        if self.0.peek() != Some(&qt) {
            return None;
        }
        // Optimisation: Avoid allocating for empty string literals.
        if self.0.peek_n(1) == Some(&qt) {
            self.0.consume_exact(&qt);
            self.0.consume_exact(&qt);
            return Some(String::with_capacity(0));
        }
        // FIXME: This is a naive implementation which doesn't match the spec.
        let mut content = String::new();
        let mut escaped = false;
        let mut raw_content_len = 0;
        for offset in 1.. {
            match self.0.peek_n(offset) {
                Some(ch) if is_line_terminator(*ch) => return None,
                None => return None,
                Some(ch) if !escaped && *ch == qt => break,
                Some('\\') if !escaped => {
                    escaped = true;
                    raw_content_len += 1;
                }
                Some(ch) => {
                    content.push(*ch);
                    escaped = false;
                    raw_content_len += 1;
                }
            }
        }
        self.0.consume_exact(&qt);
        self.0.advance_n(raw_content_len);
        self.0.consume_exact(&qt);
        Some(content)
    }

    fn parse_keyword_or_identifier(&mut self) -> Option<Token> {
        let ident_or_keyword = self.parse_identifier_name()?;
        Some(if let Ok(keyword) = Keyword::from_str(&ident_or_keyword) {
            Token::Keyword(keyword)
        } else {
            Token::Identifier(ident_or_keyword)
        })
    }

    fn parse_identifier_name(&mut self) -> Option<String> {
        let ch0 = self.0.consume_if(|ch| is_identifier_start(*ch))?;
        let mut content = self.0.consume_while_string(|ch| is_identifier_part(*ch));
        content.insert(0, ch0);
        Some(content)
    }

    fn parse_whitespace(&mut self) -> Option<char> {
        self.0.consume_if(|ch| is_whitespace(*ch))
    }

    fn parse_line_terminator(&mut self) -> Option<LineTerminator> {
        match self.0.peek().cloned() {
            Some(CR) if self.0.peek_n(1) == Some(&LF) => {
                self.0.consume_exact(&CR);
                self.0.consume_exact(&LF);
                Some(LineTerminator::Crlf)
            }
            Some(CR) => {
                self.0.consume_exact(&CR);
                Some(LineTerminator::Cr)
            }
            Some(LF) => {
                self.0.consume_exact(&LF);
                Some(LineTerminator::Lf)
            }
            Some(LS) => {
                self.0.consume_exact(&LS);
                Some(LineTerminator::Ls)
            }
            Some(PS) => {
                self.0.consume_exact(&PS);
                Some(LineTerminator::Ps)
            }
            Some(_) | None => None,
        }
    }

    fn parse_comment(&mut self) -> Option<Comment> {
        self.parse_single_line_comment()
            .map(Comment::SingleLine)
            .or_else(|| self.parse_multi_line_comment().map(Comment::MultiLine))
    }

    fn parse_single_line_comment(&mut self) -> Option<String> {
        if self.0.consume_str("//") {
            let content = self.0.consume_until_string(|ch| is_line_terminator(*ch));
            Some(content)
        } else {
            None
        }
    }

    fn parse_multi_line_comment(&mut self) -> Option<String> {
        if !self.0.peek_str("/*") {
            return None;
        }
        let mut content = String::new();
        for offset in 2.. {
            let ch = *self.0.peek_n(offset)?;
            if ch == '*' && self.0.peek_n(offset + 1) == Some(&'/') {
                break;
            }
            content.push(ch);
        }
        self.0.consume_exact(&'/');
        self.0.consume_exact(&'*');
        self.0.advance_n(content.len());
        self.0.consume_exact(&'*');
        self.0.consume_exact(&'/');
        Some(content)
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = *self.0.peek()?;
        self.parse_element()
            .or_else(|| todo!("Lexer::next: ch={}", ch))
    }
}

/// CHARACTER TABULATION
const TAB: char = '\u{0009}';
/// LINE FEED (LF)
const LF: char = '\u{000A}';
/// LINE TABULATION
const VT: char = '\u{000B}';
/// FORM FEED (FF)
const FF: char = '\u{000C}';
/// CARRIAGE RETURN (CR)
const CR: char = '\u{000D}';
/// SPACE
const SP: char = '\u{0020}';
/// NO-BREAK SPACE
const NBSP: char = '\u{00A0}';
/// ZERO WIDTH NON-JOINER
const ZWNJ: char = '\u{200C}';
/// ZERO WIDTH JOINER
const ZWJ: char = '\u{200D}';
/// LINE SEPARATOR
const LS: char = '\u{2028}';
/// PARAGRAPH SEPARATOR
const PS: char = '\u{2029}';
/// ZERO WIDTH NO-BREAK SPACE
const ZWNBSP: char = '\u{FEFF}';

fn is_whitespace(ch: char) -> bool {
    // FIXME: Return `true` for USP (any other code point classified in the "Space_Separator"
    //  category, which is not the same as the Unicode "White_Space" property).
    matches!(ch, TAB | VT | FF | SP | NBSP | ZWNBSP)
}

fn is_line_terminator(ch: char) -> bool {
    matches!(ch, LF | CR | LS | PS)
}

fn is_identifier_start(ch: char) -> bool {
    // TODO: Handle Unicode escape sequence.
    is_unicode_start(ch) || matches!(ch, '$' | '_')
}

fn is_identifier_part(ch: char) -> bool {
    // TODO: Handle Unicode escape sequence.
    is_unicode_continue(ch) || matches!(ch, '$' | ZWNJ | ZWJ)
}

fn is_unicode_start(ch: char) -> bool {
    // FIXME: Check for characters with the Unicode "ID_Start" property.
    ch.is_ascii_alphabetic()
}

fn is_unicode_continue(ch: char) -> bool {
    // FIXME: Check for characters with the Unicode "ID_Continue" property.
    ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenise_keywords() {
        for keyword in Keyword::VALUES {
            let mut lexer = Lexer::for_str(keyword.into_str());
            assert_eq!(lexer.next(), Some(Element::Token(Token::Keyword(keyword))));
            assert_eq!(lexer.next(), None);
        }
    }

    #[test]
    fn tokenise_punctuators() {
        for punctuator in Punctuator::VALUES {
            let mut lexer = Lexer::for_str(punctuator.into_str());
            assert_eq!(
                lexer.next(),
                Some(Element::Token(Token::Punctuator(punctuator)))
            );
            assert_eq!(lexer.next(), None);
        }
    }

    #[test]
    fn tokenise_string_literal() {
        fn check_valid(source: &str, expected: &str) {
            let mut lexer = Lexer::for_str(source);
            assert_eq!(
                lexer.next(),
                Some(Element::Token(Token::Literal(Literal::String(
                    expected.to_owned()
                ))))
            );
            assert_eq!(lexer.next(), None);
        }

        check_valid(r#""""#, r#""#);
        check_valid(r#""hello, world!""#, r#"hello, world!"#);
        check_valid(
            r#""hello, \"escaped quotes\"!""#,
            r#"hello, "escaped quotes"!"#,
        );
        check_valid(r#""hello, back\\slash""#, r#"hello, back\slash"#);
        check_valid(r#""hello, \\\"\"\\\\""#, r#"hello, \""\\"#);

        check_valid(r#"''"#, r#""#);
        check_valid(r#"'hello, world!'"#, r#"hello, world!"#);
        check_valid(
            r#"'hello, \"escaped quotes\"!'"#,
            r#"hello, "escaped quotes"!"#,
        );
        check_valid(r#"'hello, back\\slash'"#, r#"hello, back\slash"#);
        check_valid(r#"'hello, \\\"\"\\\\'"#, r#"hello, \""\\"#);
    }
}
