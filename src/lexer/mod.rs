use crate::util::Stream;
use std::iter::Iterator;
use std::str::FromStr;

pub use token::*;

mod token;

pub struct Lexer(Stream<char>);

impl Lexer {
    pub fn for_str(source: &str) -> Self {
        Self::for_chars(source.chars())
    }

    pub fn for_chars(source: impl Iterator<Item = char>) -> Self {
        Self(Stream::new(source))
    }

    pub fn tokens(self) -> impl Iterator<Item = Token> {
        self.filter_map(|it| {
            if let Element::Token(token) = it {
                Some(token)
            } else {
                None
            }
        })
    }
}

/// ```rust
/// # use jakescript::lexer::*;
/// let source = r##"100 + 50;"##;
///
/// let mut lexer = Lexer::for_str(source);
/// assert_eq!(
///     lexer.collect::<Vec<_>>(),
///     vec![
///         Element::Token(Token::Literal(Literal::Numeric(100))),
///         Element::Whitespace(' '),
///         Element::Token(Token::Punctuator(Punctuator::Plus)),
///         Element::Whitespace(' '),
///         Element::Token(Token::Literal(Literal::Numeric(50))),
///         Element::Token(Token::Punctuator(Punctuator::Semicolon)),
///     ]
/// )
/// ```
///
/// ```rust
/// # use jakescript::lexer::*;
/// let source = r##"
/// let a = /* Hello, */ 100;
/// let b = 50; // world!
/// console.log("100 + 50 = %s", a + b);
/// "##;
///
/// let mut lexer = Lexer::for_str(source);
/// assert_eq!(
///     lexer.collect::<Vec<_>>(),
///     vec![
///         Element::LineTerminator,
///         Element::Token(Token::Keyword(Keyword::Let)),
///         Element::Whitespace(' '),
///         Element::Token(Token::Identifier("a".to_owned())),
///         Element::Whitespace(' '),
///         Element::Token(Token::Punctuator(Punctuator::Equal)),
///         Element::Whitespace(' '),
///         Element::Comment(" Hello, ".to_owned(), CommentKind::MultiLine),
///         Element::Whitespace(' '),
///         Element::Token(Token::Literal(Literal::Numeric(100))),
///         Element::Token(Token::Punctuator(Punctuator::Semicolon)),
///         Element::LineTerminator,
///         Element::Token(Token::Keyword(Keyword::Let)),
///         Element::Whitespace(' '),
///         Element::Token(Token::Identifier("b".to_owned())),
///         Element::Whitespace(' '),
///         Element::Token(Token::Punctuator(Punctuator::Equal)),
///         Element::Whitespace(' '),
///         Element::Token(Token::Literal(Literal::Numeric(50))),
///         Element::Token(Token::Punctuator(Punctuator::Semicolon)),
///         Element::Whitespace(' '),
///         Element::Comment(" world!".to_owned(), CommentKind::SingleLine),
///         Element::LineTerminator,
///         Element::Token(Token::Identifier("console".to_owned())),
///         Element::Token(Token::Punctuator(Punctuator::Dot)),
///         Element::Token(Token::Identifier("log".to_owned())),
///         Element::Token(Token::Punctuator(Punctuator::OpenParen)),
///         Element::Token(Token::Literal(Literal::String("100 + 50 = %s".to_owned()))),
///         Element::Token(Token::Punctuator(Punctuator::Comma)),
///         Element::Whitespace(' '),
///         Element::Token(Token::Identifier("a".to_owned())),
///         Element::Whitespace(' '),
///         Element::Token(Token::Punctuator(Punctuator::Plus)),
///         Element::Whitespace(' '),
///         Element::Token(Token::Identifier("b".to_owned())),
///         Element::Token(Token::Punctuator(Punctuator::CloseParen)),
///         Element::Token(Token::Punctuator(Punctuator::Semicolon)),
///         Element::LineTerminator,
///     ]
/// );
/// ```
///
/// ```rust
/// # use jakescript::lexer::*;
/// let source = r##"
/// let x = 0;
/// while (x < 3) {
///     x = x + 1;
/// }
/// "##;
///
/// let mut lexer = Lexer::for_str(source);
/// assert_eq!(
///     lexer.tokens().collect::<Vec<_>>(),
///     vec![
///         Token::Keyword(Keyword::Let),
///         Token::Identifier("x".to_owned()),
///         Token::Punctuator(Punctuator::Equal),
///         Token::Literal(Literal::Numeric(0)),
///         Token::Punctuator(Punctuator::Semicolon),
///
///         Token::Keyword(Keyword::While),
///         Token::Punctuator(Punctuator::OpenParen),
///         Token::Identifier("x".to_owned()),
///         Token::Punctuator(Punctuator::LessThan),
///         Token::Literal(Literal::Numeric(3)),
///         Token::Punctuator(Punctuator::CloseParen),
///         Token::Punctuator(Punctuator::OpenBrace),
///
///         Token::Identifier("x".to_owned()),
///         Token::Punctuator(Punctuator::Equal),
///         Token::Identifier("x".to_owned()),
///         Token::Punctuator(Punctuator::Plus),
///         Token::Literal(Literal::Numeric(1)),
///         Token::Punctuator(Punctuator::Semicolon),
///
///         Token::Punctuator(Punctuator::CloseBrace),
///     ]
/// );
impl Iterator for Lexer {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = *self.0.peek()?;

        Some(if let Some(whitespace) = self.parse_whitespace() {
            Element::Whitespace(whitespace)
        } else if let Some(..) = self.parse_line_terminator_sequence() {
            Element::LineTerminator
        } else if let Some((content, kind)) = self.parse_comment() {
            Element::Comment(content, kind)
        } else if let Some(token) = self.parse_token() {
            Element::Token(token)
        } else {
            todo!("{}", ch)
        })
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

impl Lexer {
    fn is_whitespace(ch: char) -> bool {
        // FIXME: or USP, which is any other code point classified in the "Space_Separator"
        // category, which is different from looking at the Unicode "White_Space" property
        matches!(ch, TAB | VT | FF | SP | NBSP | ZWNBSP)
    }

    fn is_line_terminator(ch: char) -> bool {
        matches!(ch, LF | CR | LS | PS)
    }

    fn is_identifier_start(ch: char) -> bool {
        // TODO: Handle Unicode escape sequence
        Self::is_unicode_start(ch) || matches!(ch, '$' | '_')
    }

    fn is_identifier_part(ch: char) -> bool {
        // TODO: Handle Unicode escape sequence
        Self::is_unicode_continue(ch) || matches!(ch, '$' | ZWNJ | ZWJ)
    }

    fn is_unicode_start(ch: char) -> bool {
        // FIXME: Actually check for characters with the Unicode "ID_Start" property
        ch.is_ascii_alphabetic()
    }

    fn is_unicode_continue(ch: char) -> bool {
        // FIXME: Actually check for characters with the Unicode "ID_Continue" property
        ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_'
    }

    fn parse_whitespace(&mut self) -> Option<char> {
        self.0.consume_if(|ch| Self::is_whitespace(*ch))
    }

    fn parse_line_terminator_sequence(&mut self) -> Option<(char, Option<char>)> {
        match self.0.peek() {
            Some(ch @ (&LF | &LS | &PS)) => {
                let ch = *ch;
                self.0.consume_exact(&ch);
                Some((ch, None))
            }
            Some(&CR) if self.0.peek_n(1) != Some(&LF) => {
                self.0.consume_exact(&CR);
                Some((CR, None))
            }
            Some(&CR) if self.0.peek_n(1) == Some(&LF) => {
                self.0.consume_exact(&CR);
                self.0.consume_exact(&LF);
                Some((CR, Some(LF)))
            }
            Some(_) | None => None,
        }
    }

    fn parse_comment(&mut self) -> Option<(String, CommentKind)> {
        if let Some(content) = self.parse_multi_line_comment() {
            return Some((content, CommentKind::MultiLine));
        }
        if let Some(content) = self.parse_single_line_comment() {
            return Some((content, CommentKind::SingleLine));
        }
        None
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

    fn parse_single_line_comment(&mut self) -> Option<String> {
        if self.0.consume_str("//") {
            Some(
                self.0
                    .consume_until_string(|ch| Self::is_line_terminator(*ch)),
            )
        } else {
            None
        }
    }

    fn parse_token(&mut self) -> Option<Token> {
        Some(
            if let Some(ident_name_or_keyword) = self.parse_identifier_name() {
                if let Ok(keyword) = Keyword::from_str(&ident_name_or_keyword) {
                    Token::Keyword(keyword)
                } else {
                    Token::Identifier(ident_name_or_keyword)
                }
            } else if let Some(punctuator) = self.parse_punctuator() {
                Token::Punctuator(punctuator)
            } else if let Some(()) = self.parse_null_literal() {
                Token::Literal(Literal::Null)
            } else if let Some(bool_lit) = self.parse_boolean_literal() {
                Token::Literal(Literal::Boolean(bool_lit))
            } else if let Some(numeric_lit) = self.parse_numeric_literal() {
                Token::Literal(Literal::Numeric(numeric_lit))
            } else if let Some(string_lit) = self.parse_string_literal() {
                Token::Literal(Literal::String(string_lit))
            } else {
                // TODO: Parse template tokens
                return None;
            },
        )
    }

    fn parse_identifier_name(&mut self) -> Option<String> {
        let ch0 = self.0.consume_if(|ch| Self::is_identifier_start(*ch))?;
        let mut content = self
            .0
            .consume_while_string(|ch| Self::is_identifier_part(*ch));
        content.insert(0, ch0);
        Some(content)
    }

    fn parse_punctuator(&mut self) -> Option<Punctuator> {
        for punctuator in Punctuator::VALUES_IN_LEXICAL_ORDER {
            if self.0.consume_str(punctuator.to_str()) {
                return Some(punctuator);
            }
        }
        None
    }

    fn parse_null_literal(&mut self) -> Option<()> {
        if self.0.consume_str("null") {
            Some(())
        } else {
            None
        }
    }

    fn parse_boolean_literal(&mut self) -> Option<bool> {
        if self.0.consume_str("true") {
            Some(true)
        } else if self.0.consume_str("false") {
            Some(false)
        } else {
            None
        }
    }

    fn parse_numeric_literal(&mut self) -> Option<u64> {
        // FIXME: This is a naieve implementation which doesn't match the spec
        let content = self.0.consume_while_string(|ch| ch.is_ascii_digit());
        if !content.is_empty() {
            u64::from_str(&content).ok()
        } else {
            None
        }
    }

    fn parse_string_literal(&mut self) -> Option<String> {
        if let Some(content) = self.parse_quoted_literal('"') {
            return Some(content);
        }
        if let Some(content) = self.parse_quoted_literal('\'') {
            return Some(content);
        }
        None
    }

    fn parse_quoted_literal(&mut self, qt: char) -> Option<String> {
        if self.0.peek() != Some(&qt) {
            return None;
        }
        // Optimisation: avoid alloc for empty string literals
        if self.0.peek_n(1) == Some(&qt) {
            self.0.consume_exact(&qt);
            self.0.consume_exact(&qt);
            return Some(String::with_capacity(0));
        }
        // FIXME: This is a naieve implementation which doesn't match the spec
        let mut content = String::new();
        let mut escaped = false;
        let mut raw_content_len = 0;
        for offset in 1.. {
            match self.0.peek_n(offset) {
                Some(ch) if Self::is_line_terminator(*ch) => return None,
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenise_keywords() {
        for keyword in Keyword::VALUES {
            let mut lexer = Lexer::for_str(keyword.to_str());
            assert_eq!(lexer.next(), Some(Element::Token(Token::Keyword(keyword))));
            assert_eq!(lexer.next(), None);
        }
    }

    #[test]
    fn tokenise_punctuators() {
        for punctuator in Punctuator::VALUES {
            let mut lexer = Lexer::for_str(punctuator.to_str());
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
