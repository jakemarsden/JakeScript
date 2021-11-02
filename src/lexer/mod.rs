use crate::iter::{IntoPeekableNth, PeekableNth};
use error::LexicalErrorKind::*;
use std::io;
use std::iter::{FilterMap, Map};
use std::str::{Chars, FromStr};

pub use error::*;
pub use token::*;

mod error;
mod token;

pub type Tokens<I> = FilterMap<I, fn(LexicalResult<Element>) -> Option<LexicalResult<Token>>>;
type Fallible<I> = Map<I, fn(char) -> io::Result<char>>;

pub struct Lexer<I: Iterator<Item = io::Result<char>>>(PeekableNth<I>, State);

impl<'a> Lexer<Fallible<Chars<'a>>> {
    pub fn for_str(source: &'a str) -> Self {
        Self::for_chars(source.chars())
    }
}

impl<I: Iterator<Item = char>> Lexer<Fallible<I>> {
    pub fn for_chars(source: I) -> Self {
        Self::for_chars_fallible(source.map(Ok))
    }
}

impl<I: Iterator<Item = io::Result<char>>> Lexer<I> {
    pub fn for_chars_fallible(source: I) -> Self {
        Self(source.peekable_nth(), State::default())
    }

    pub fn tokens(self) -> Tokens<Self> {
        self.filter_map(|result| match result {
            Ok(elem) => elem.token().map(Ok),
            Err(err) => Some(Err(err)),
        })
    }

    fn state(&self) -> State {
        self.1
    }

    fn set_state(&mut self, state: State) {
        self.1 = state;
    }

    fn parse_element(&mut self) -> LexicalResult<Element> {
        Ok(if let Some(it) = self.parse_whitespace()? {
            Element::Whitespace(it)
        } else if let Some(it) = self.parse_line_terminator()? {
            Element::LineTerminator(it)
        } else if let Some(it) = self.parse_comment()? {
            Element::Comment(it)
        } else if let Some(it) = self.parse_token()? {
            Element::Token(it)
        } else {
            let ch = self.0.try_peek().unwrap().unwrap();
            todo!("Lexer::parse_element: ch={}", ch)
        })
    }

    fn parse_token(&mut self) -> LexicalResult<Option<Token>> {
        // TODO: Parse template tokens.
        Ok(if let Some(value) = self.parse_punctuator()? {
            Some(Token::Punctuator(value))
        } else if let Some(value) = self.parse_literal()? {
            Some(Token::Literal(value))
        } else {
            self.parse_keyword_or_identifier()?
        })
    }

    fn parse_punctuator(&mut self) -> LexicalResult<Option<Punctuator>> {
        for value in Punctuator::VALUES_IN_LEXICAL_ORDER {
            if self.0.try_consume_str(value.into_str())? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn parse_literal(&mut self) -> LexicalResult<Option<Literal>> {
        Ok(if let Some(()) = self.parse_null_literal()? {
            Some(Literal::Null)
        } else if let Some(()) = self.parse_undefined_literal()? {
            Some(Literal::Undefined)
        } else if let Some(value) = self.parse_boolean_literal()? {
            Some(Literal::Boolean(value))
        } else if let Some(value) = self.parse_numeric_literal()? {
            Some(Literal::Numeric(value))
        } else {
            self.parse_string_literal()?.map(Literal::String)
        })
    }

    fn parse_null_literal(&mut self) -> LexicalResult<Option<()>> {
        Ok(self.0.try_consume_str("null")?.then_some(()))
    }

    fn parse_undefined_literal(&mut self) -> LexicalResult<Option<()>> {
        Ok(self.0.try_consume_str("undefined")?.then_some(()))
    }

    fn parse_boolean_literal(&mut self) -> LexicalResult<Option<bool>> {
        Ok(if self.0.try_consume_str("true")? {
            Some(true)
        } else if self.0.try_consume_str("false")? {
            Some(false)
        } else {
            None
        })
    }

    fn parse_numeric_literal(&mut self) -> LexicalResult<Option<NumericLiteral>> {
        let value = if let Some(value) = self.parse_non_decimal_int_literal()? {
            Some(value)
        } else {
            self.parse_decimal_literal()?
        };
        if let Some(value) = value {
            // Ensure the character following the numeric literal is valid
            match self.0.try_peek()? {
                Some(ch) if is_identifier_part(*ch) => {
                    Err(LexicalError::new(IdentifierFollowingNumericLiteral))
                }
                Some(ch) if ch.is_digit(10) => Err(LexicalError::new(DigitFollowingNumericLiteral)),
                Some(_) | None => Ok(Some(value)),
            }
        } else {
            Ok(None)
        }
    }

    /// ```plain
    /// DecimalLiteral::
    ///     DecimalIntegerLiteral . DecimalDigits(opt) ExponentPart(opt)
    ///     . DecimalDigits ExponentPart(opt)
    ///     DecimalIntegerLiteral ExponentPart(opt)
    /// ```
    fn parse_decimal_literal(&mut self) -> LexicalResult<Option<NumericLiteral>> {
        // TODO: Parse floating point values and exponents.
        // TODO: Parse big integer literals.
        Ok(self
            .parse_decimal_int_literal()?
            .map(NumericLiteral::DecInt))
    }

    /// ```plain
    /// DecimalIntegerLiteral::
    ///     0
    ///     NonZeroDigit DecimalDigits(opt)
    /// ```
    fn parse_decimal_int_literal(&mut self) -> LexicalResult<Option<u64>> {
        if self.0.try_next_if_eq(&'0')?.is_some() {
            Ok(Some(0))
        } else {
            self.parse_int_literal_part(10)
        }
    }

    /// ```plain
    /// BinaryIntegerLiteral::
    ///     0b BinaryDigits
    ///     0B BinaryDigits
    /// OctalIntegerLiteral::
    ///     0o OctalDigits
    ///     0O OctalDigits
    /// HexIntegerLiteral::
    ///     0x HexDigits
    ///     0X HexDigits
    /// ```
    fn parse_non_decimal_int_literal(&mut self) -> LexicalResult<Option<NumericLiteral>> {
        if !matches!(self.0.try_peek()?, Some('0')) {
            return Ok(None);
        }
        let (radix, ch1) = match self.0.try_peek_nth(1)? {
            Some(ch @ 'b' | ch @ 'B') => (2, *ch),
            Some(ch @ 'o' | ch @ 'O') => (8, *ch),
            Some(ch @ 'x' | ch @ 'X') => (16, *ch),
            _ => return Ok(None),
        };
        if !matches!(self.0.try_peek_nth(2)?, Some(ch2) if ch2.is_digit(radix)) {
            return Ok(None);
        }
        self.0.try_next_exact(&'0').unwrap();
        self.0.try_next_exact(&ch1).unwrap();
        let value = self.parse_int_literal_part(radix)?.unwrap();
        Ok(Some(match radix {
            2 => NumericLiteral::BinInt(value),
            8 => NumericLiteral::OctInt(value),
            16 => NumericLiteral::HexInt(value),
            _ => unreachable!("{}", radix),
        }))
    }

    fn parse_int_literal_part(&mut self, radix: u32) -> LexicalResult<Option<u64>> {
        let mut present = false;
        let mut value = 0;
        while let Some(ch) = self.0.try_next_if(|ch| ch.is_digit(radix))? {
            present = true;
            value *= radix as u64;
            value += ch.to_digit(radix).unwrap() as u64;
        }
        Ok(present.then_some(value))
    }

    fn parse_string_literal(&mut self) -> LexicalResult<Option<StringLiteral>> {
        Ok(if let Some(value) = self.parse_quoted_literal('\'')? {
            Some(StringLiteral::SingleQuoted(value))
        } else {
            self.parse_quoted_literal('"')?
                .map(StringLiteral::DoubleQuoted)
        })
    }

    fn parse_quoted_literal(&mut self, qt: char) -> LexicalResult<Option<String>> {
        if self.0.try_peek()? != Some(&qt) {
            return Ok(None);
        }
        // Optimisation: Avoid allocating for empty string literals.
        if self.0.try_peek_nth(1)? == Some(&qt) {
            self.0.try_next_exact(&qt)?;
            self.0.try_next_exact(&qt)?;
            return Ok(Some(String::with_capacity(0)));
        }
        // FIXME: This is a naive implementation which doesn't match the spec.
        let mut content = String::new();
        let mut escaped = false;
        let mut escape_count = 0;
        for offset in 1.. {
            let ch = if let Some(ch) = self.0.try_peek_nth(offset)? {
                ch
            } else {
                return Err(LexicalError::new(UnclosedStringLiteral));
            };
            match (ch, escaped) {
                (ch, _) if is_line_terminator(*ch) => {
                    return Err(LexicalError::new(UnclosedStringLiteral))
                }
                ('\\', false) => {
                    escaped = true;
                    escape_count += 1;
                }
                ('\\', true) => {
                    content.push('\\');
                    escaped = false;
                }
                (ch, false) if *ch == qt => break,
                (ch, true) if *ch == qt => {
                    content.push(*ch);
                    escaped = false;
                }
                ('n', true) => {
                    content.push('\n');
                    escaped = false;
                }
                ('r', true) => {
                    content.push('\r');
                    escaped = false;
                }
                ('t', true) => {
                    content.push('\t');
                    escaped = false;
                }

                (_, true) => return Err(LexicalError::new(IllegalStringLiteralEscapeSequence)),
                (ch, false) => {
                    content.push(*ch);
                }
            }
        }
        self.0.try_next_exact(&qt)?;
        self.0.advance_by(content.len() + escape_count).unwrap();
        self.0.try_next_exact(&qt)?;
        Ok(Some(content))
    }

    fn parse_keyword_or_identifier(&mut self) -> LexicalResult<Option<Token>> {
        Ok(self.parse_identifier_name()?.map(|ident_or_keyword| {
            Keyword::from_str(&ident_or_keyword)
                .map(Token::Keyword)
                .unwrap_or_else(|_| Token::Identifier(ident_or_keyword))
        }))
    }

    fn parse_identifier_name(&mut self) -> LexicalResult<Option<String>> {
        if let Some(ch0) = self.0.try_next_if(|ch| is_identifier_start(*ch))? {
            let mut content: String = self.0.try_collect_while(|ch| is_identifier_part(*ch))?;
            content.insert(0, ch0);
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    fn parse_whitespace(&mut self) -> LexicalResult<Option<char>> {
        Ok(self.0.try_next_if(|ch| is_whitespace(*ch))?)
    }

    fn parse_line_terminator(&mut self) -> LexicalResult<Option<LineTerminator>> {
        Ok(match self.0.try_peek()?.cloned() {
            Some(CR) if self.0.try_peek_nth(1)? == Some(&LF) => {
                self.0.try_next_exact(&CR)?;
                self.0.try_next_exact(&LF)?;
                Some(LineTerminator::Crlf)
            }
            Some(CR) => {
                self.0.try_next_exact(&CR)?;
                Some(LineTerminator::Cr)
            }
            Some(LF) => {
                self.0.try_next_exact(&LF)?;
                Some(LineTerminator::Lf)
            }
            Some(LS) => {
                self.0.try_next_exact(&LS)?;
                Some(LineTerminator::Ls)
            }
            Some(PS) => {
                self.0.try_next_exact(&PS)?;
                Some(LineTerminator::Ps)
            }
            Some(_) | None => None,
        })
    }

    fn parse_comment(&mut self) -> LexicalResult<Option<Comment>> {
        Ok(if let Some(content) = self.parse_single_line_comment()? {
            Some(Comment::SingleLine(content))
        } else {
            self.parse_multi_line_comment()?.map(Comment::MultiLine)
        })
    }

    fn parse_single_line_comment(&mut self) -> LexicalResult<Option<String>> {
        if self.0.try_peek()? == Some(&'/') && self.0.try_peek_nth(1)? == Some(&'/') {
            self.0.try_next_exact(&'/')?;
            self.0.try_next_exact(&'/')?;
            let content = self.0.try_collect_until(|ch| is_line_terminator(*ch))?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    fn parse_multi_line_comment(&mut self) -> LexicalResult<Option<String>> {
        if self.0.try_peek()? != Some(&'/') || self.0.try_peek_nth(1)? != Some(&'*') {
            return Ok(None);
        }
        let mut content = String::new();
        for offset in 2.. {
            let ch = match self.0.try_peek_nth(offset)? {
                Some(ch) => *ch,
                None => return Err(LexicalError::new(UnclosedComment)),
            };
            if ch == '*' && self.0.try_peek_nth(offset + 1)? == Some(&'/') {
                break;
            }
            content.push(ch);
        }
        self.0.try_next_exact(&'/')?;
        self.0.try_next_exact(&'*')?;
        self.0.advance_by(content.len()).unwrap();
        self.0.try_next_exact(&'*')?;
        self.0.try_next_exact(&'/')?;
        Ok(Some(content))
    }
}

impl<I: Iterator<Item = io::Result<char>>> Iterator for Lexer<I> {
    type Item = LexicalResult<Element>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state() {
            State::Normal => {
                if self.0.peek().is_some() {
                    let result = self.parse_element();
                    if result.is_err() {
                        self.set_state(State::Error);
                    }
                    Some(result)
                } else {
                    self.set_state(State::End);
                    None
                }
            }
            State::End | State::Error => None,
        }
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
enum State {
    #[default]
    Normal,
    Error,
    End,
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
    use std::assert_matches::assert_matches;

    #[test]
    fn tokenise_keywords() {
        for expected in Keyword::VALUES {
            let mut lexer = Lexer::for_str(expected.into_str());
            assert_matches!(
                lexer.next(),
                Some(Ok(Element::Token(Token::Keyword(actual)))) if actual == expected
            );
            assert_matches!(lexer.next(), None);
        }
    }

    #[test]
    fn tokenise_punctuators() {
        for expected in Punctuator::VALUES {
            let mut lexer = Lexer::for_str(expected.into_str());
            assert_matches!(
                lexer.next(),
                Some(Ok(Element::Token(Token::Punctuator(actual)))) if actual == expected
            );
            assert_matches!(lexer.next(), None);
        }
    }

    #[test]
    fn tokenise_string_literal() {
        fn check_valid(source: &str, expected: &str, single_qt: bool) {
            let mut lexer = Lexer::for_str(source);
            if single_qt {
                assert_matches!(
                    lexer.next(),
                    Some(Ok(Element::Token(Token::Literal(Literal::String(
                        StringLiteral::SingleQuoted(actual)
                    ))))) if actual == expected
                );
            } else {
                assert_matches!(
                    lexer.next(),
                    Some(Ok(Element::Token(Token::Literal(Literal::String(
                        StringLiteral::DoubleQuoted(actual)
                    ))))) if actual == expected
                );
            }
            assert_matches!(lexer.next(), None);
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
    fn tokenise_unclosed_string_literal() {
        let source_code = r#"'hello, world!"#;
        let mut lexer = Lexer::for_str(source_code);
        assert_matches!(lexer.next(), Some(Err(err)) if err.kind() == Some(UnclosedStringLiteral));
        assert_matches!(lexer.next(), None);

        let source_code = "'hello, world!\nClosed on the next line'";
        let mut lexer = Lexer::for_str(source_code);
        assert_matches!(lexer.next(), Some(Err(err)) if err.kind() == Some(UnclosedStringLiteral));
        assert_matches!(lexer.next(), None);
    }

    #[test]
    fn tokenise_illegal_string_literal_escape_sequence() {
        let source_code = r#""\z""#;
        let mut lexer = Lexer::for_str(source_code);
        assert_matches!(
            lexer.next(),
            Some(Err(err)) if err.kind() == Some(IllegalStringLiteralEscapeSequence)
        );
        assert_matches!(lexer.next(), None);

        // Can't escape single quote inside double quoted string literal
        let source_code = r#""\'""#;
        let mut lexer = Lexer::for_str(source_code);
        assert_matches!(
            lexer.next(),
            Some(Err(err)) if err.kind() == Some(IllegalStringLiteralEscapeSequence)
        );
        assert_matches!(lexer.next(), None);

        // Can't escape double quote inside single quoted string literal
        let source_code = r#"'\"'"#;
        let mut lexer = Lexer::for_str(source_code);
        assert_matches!(
            lexer.next(),
            Some(Err(err)) if err.kind() == Some(IllegalStringLiteralEscapeSequence)
        );
        assert_matches!(lexer.next(), None);
    }

    #[test]
    fn tokenise_unclosed_multi_line_comment() {
        let source_code = "/* abc";
        let mut lexer = Lexer::for_str(source_code);
        assert_matches!(lexer.next(), Some(Err(err)) if err.kind() == Some(UnclosedComment));
        assert_matches!(lexer.next(), None);
    }
}
