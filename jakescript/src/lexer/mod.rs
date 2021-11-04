use crate::iter::{IntoPeekableNth, PeekableNth};
use crate::str::NonEmptyString;
use enumerate::EnumerateStr;
use error::ErrorKind::*;
use std::io;
use std::iter::{FilterMap, Map};
use std::str::{Chars, FromStr};

pub use error::*;
pub use token::*;

mod error;
mod token;

pub type Tokens<I> = FilterMap<I, fn(Result) -> Option<Result<Token>>>;
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

    fn parse_element(&mut self) -> Result {
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

    fn parse_token(&mut self) -> Result<Option<Token>> {
        // TODO: Parse template tokens.
        Ok(if let Some(value) = self.parse_literal()? {
            Some(Token::Literal(value))
        } else if let Some(value) = self.parse_punctuator()? {
            Some(Token::Punctuator(value))
        } else {
            self.parse_keyword_or_identifier()?
        })
    }

    fn parse_punctuator(&mut self) -> Result<Option<Punctuator>> {
        for value in Punctuator::enumerate_in_lexical_order() {
            if self.0.try_consume_str(value.as_str())? {
                return Ok(Some(*value));
            }
        }
        Ok(None)
    }

    fn parse_literal(&mut self) -> Result<Option<Literal>> {
        Ok(if let Some(()) = self.parse_null_literal()? {
            Some(Literal::Null)
        } else if let Some(()) = self.parse_undefined_literal()? {
            Some(Literal::Undefined)
        } else if let Some(value) = self.parse_boolean_literal()? {
            Some(Literal::Boolean(value))
        } else if let Some(value) = self.parse_numeric_literal()? {
            Some(Literal::Numeric(value))
        } else if let Some(value) = self.parse_string_literal()? {
            Some(Literal::String(value))
        } else {
            self.parse_regex_literal()?.map(Literal::RegEx)
        })
    }

    fn parse_null_literal(&mut self) -> Result<Option<()>> {
        Ok(self.0.try_consume_str("null")?.then_some(()))
    }

    fn parse_undefined_literal(&mut self) -> Result<Option<()>> {
        Ok(self.0.try_consume_str("undefined")?.then_some(()))
    }

    fn parse_boolean_literal(&mut self) -> Result<Option<bool>> {
        Ok(if self.0.try_consume_str("true")? {
            Some(true)
        } else if self.0.try_consume_str("false")? {
            Some(false)
        } else {
            None
        })
    }

    fn parse_numeric_literal(&mut self) -> Result<Option<NumericLiteral>> {
        let value = if let Some(value) = self.parse_non_decimal_int_literal()? {
            Some(value)
        } else {
            self.parse_decimal_literal()?
        };
        if let Some(value) = value {
            // Ensure the character following the numeric literal is valid
            match self.0.try_peek()? {
                Some(ch) if is_identifier_part(*ch) => {
                    Err(Error::new(IdentifierFollowingNumericLiteral))
                }
                Some(ch) if ch.is_digit(10) => Err(Error::new(DigitFollowingNumericLiteral)),
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
    fn parse_decimal_literal(&mut self) -> Result<Option<NumericLiteral>> {
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
    fn parse_decimal_int_literal(&mut self) -> Result<Option<u64>> {
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
    fn parse_non_decimal_int_literal(&mut self) -> Result<Option<NumericLiteral>> {
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

    fn parse_int_literal_part(&mut self, radix: u32) -> Result<Option<u64>> {
        let mut present = false;
        let mut value = 0;
        while let Some(ch) = self.0.try_next_if(|ch| ch.is_digit(radix))? {
            present = true;
            value *= radix as u64;
            value += ch.to_digit(radix).unwrap() as u64;
        }
        Ok(present.then_some(value))
    }

    /// ```plain
    /// StringLiteral::
    ///     " DoubleStringCharactersopt(opt) "
    ///     ' SingleStringCharactersopt(opt) '
    fn parse_string_literal(&mut self) -> Result<Option<StringLiteral>> {
        Ok(if let Some(value) = self.parse_string_literal_impl('\'')? {
            Some(StringLiteral::SingleQuoted(value))
        } else {
            self.parse_string_literal_impl('"')?
                .map(StringLiteral::DoubleQuoted)
        })
    }

    fn parse_string_literal_impl(&mut self, qt: char) -> Result<Option<String>> {
        if self.0.try_peek()? != Some(&qt) {
            return Ok(None);
        }
        // Optimisation: Avoid allocating for empty string literals.
        if self.0.try_peek_nth(1)? == Some(&qt) {
            self.0.try_next_exact(&qt)?;
            self.0.try_next_exact(&qt)?;
            return Ok(Some(String::with_capacity(0)));
        }
        let mut escaped = false;
        let mut content = String::new();
        let mut raw_content_len = 0;

        let mut offset = 1;
        loop {
            let ch = if let Some(ch) = self.0.try_peek_nth(offset)? {
                ch
            } else {
                return Ok(None);
            };
            match (ch, escaped) {
                (ch, false) if *ch == qt => {
                    break;
                }
                ('\\', false) => {
                    escaped = true;
                }
                (ch @ &LS | ch @ &PS, false) => {
                    content.push(*ch);
                }
                (ch, false) if is_line_terminator(*ch) => {
                    return Ok(None);
                }
                (ch, true) if is_line_terminator(*ch) => {
                    // LineContinuation::
                    //     \ LineTerminatorSequence
                    // LineTerminatorSequence::
                    //     <LF>
                    //     <CR> [lookahead ≠ <LF>]
                    //     <LS>
                    //     <PS>
                    //     <CR> <LF>
                    escaped = false;
                    if *ch == CR && self.0.try_peek_nth(offset + 1)? == Some(&LF) {
                        // Skip the next iteration
                        offset += 1;
                        raw_content_len += 1;
                    }
                }
                (ch, true) => {
                    // TODO: Handle escape sequences properly
                    // EscapeSequence::
                    //     CharacterEscapeSequence
                    //     0 [lookahead ∉ DecimalDigit]
                    //     HexEscapeSequence
                    //     UnicodeEscapeSequence
                    escaped = false;
                    content.push(into_escaped(*ch));
                }
                (ch, false) => {
                    content.push(*ch);
                }
            }
            offset += 1;
            raw_content_len += 1;
        }
        debug_assert!(!escaped);
        debug_assert!(!content.is_empty());
        debug_assert!(raw_content_len >= content.len());
        self.0.try_next_exact(&qt)?;
        self.0.advance_by(raw_content_len).unwrap();
        self.0.try_next_exact(&qt)?;
        Ok(Some(content))
    }

    /// ```plain
    /// RegularExpressionLiteral::
    ///     / RegularExpressionBody / RegularExpressionFlags
    /// ```
    fn parse_regex_literal(&mut self) -> Result<Option<RegExLiteral>> {
        if !matches!(self.0.try_peek()?, Some('/')) {
            return Ok(None);
        }
        if matches!(self.0.try_peek_nth(1)?, Some('*')) {
            // Not a valid `RegularExpressionFirstChar`.
            return Ok(None);
        }
        if matches!(self.0.try_peek_nth(1)?, Some('/')) {
            // Not a valid `RegularExpressionFirstChar`. Empty regexes aren't representable because
            // `//` represents the start of a single-line comment. The spec suggests using `/(?:)/`
            // as a workaround.
            return Ok(None);
        }
        let mut escaped = false;
        let mut in_class = false;
        let mut content = String::new();
        let mut raw_content_len = 0;
        for offset in 1.. {
            let ch = if let Some(ch) = self.0.try_peek_nth(offset)? {
                ch
            } else {
                return Ok(None);
            };
            match (ch, escaped, in_class) {
                ('/', false, false) => {
                    break;
                }
                ('\\', false, _) => {
                    escaped = true;
                }
                ('[', false, false) => {
                    in_class = true;
                    content.push('[');
                }
                (']', false, true) => {
                    in_class = false;
                    content.push(']');
                }
                (ch, _, _) if is_line_terminator(*ch) => {
                    return Ok(None);
                }
                (ch, true, _) => {
                    escaped = false;
                    content.push(into_escaped(*ch));
                }
                (ch, false, _) => {
                    content.push(*ch);
                }
            }
            raw_content_len += 1;
        }
        debug_assert!(!escaped);
        debug_assert!(!in_class);
        debug_assert!(!content.is_empty());
        debug_assert!(raw_content_len >= content.len());
        self.0.try_next_exact(&'/')?;
        self.0.advance_by(raw_content_len).unwrap();
        self.0.try_next_exact(&'/')?;

        // Safety: It's not possible for the loop to break without pushing (at least) one character
        // to the string, given that the first char it handles cannot be '/'.
        let content = unsafe { NonEmptyString::from_unchecked(content) };

        let flags = self.0.try_collect_while(|ch| is_identifier_start(*ch))?;
        Ok(Some(RegExLiteral { content, flags }))
    }

    fn parse_keyword_or_identifier(&mut self) -> Result<Option<Token>> {
        Ok(self.parse_identifier_name()?.map(|ident_or_keyword| {
            Keyword::from_str(&ident_or_keyword)
                .map(Token::Keyword)
                .unwrap_or_else(|_| Token::Identifier(ident_or_keyword))
        }))
    }

    fn parse_identifier_name(&mut self) -> Result<Option<String>> {
        if let Some(ch0) = self.0.try_next_if(|ch| is_identifier_start(*ch))? {
            let mut content: String = self.0.try_collect_while(|ch| is_identifier_part(*ch))?;
            content.insert(0, ch0);
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    fn parse_whitespace(&mut self) -> Result<Option<char>> {
        Ok(self.0.try_next_if(|ch| is_whitespace(*ch))?)
    }

    fn parse_line_terminator(&mut self) -> Result<Option<LineTerminator>> {
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

    fn parse_comment(&mut self) -> Result<Option<Comment>> {
        Ok(if let Some(content) = self.parse_single_line_comment()? {
            Some(Comment::SingleLine(content))
        } else {
            self.parse_multi_line_comment()?.map(Comment::MultiLine)
        })
    }

    fn parse_single_line_comment(&mut self) -> Result<Option<String>> {
        if self.0.try_peek()? == Some(&'/') && self.0.try_peek_nth(1)? == Some(&'/') {
            self.0.try_next_exact(&'/')?;
            self.0.try_next_exact(&'/')?;
            let content = self.0.try_collect_until(|ch| is_line_terminator(*ch))?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    fn parse_multi_line_comment(&mut self) -> Result<Option<String>> {
        if self.0.try_peek()? != Some(&'/') || self.0.try_peek_nth(1)? != Some(&'*') {
            return Ok(None);
        }
        let mut content = String::new();
        for offset in 2.. {
            let ch = match self.0.try_peek_nth(offset)? {
                Some(ch) => *ch,
                None => return Err(Error::new(UnclosedComment)),
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
    type Item = Result;

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

/// NULL
const NUL: char = '\u{0000}';
/// BACKSPACE
const BS: char = '\u{0008}';
/// CHARACTER TABULATION
const HT: char = '\u{0009}';
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
    matches!(ch, HT | VT | FF | SP | NBSP | ZWNBSP)
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

fn into_escaped(ch: char) -> char {
    match ch {
        '0' => NUL,
        'b' => BS,
        't' => HT,
        'n' => LF,
        'v' => VT,
        'f' => FF,
        'r' => CR,
        ch => ch,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use enumerate::{Enumerate, EnumerateStr};
    use std::assert_matches::assert_matches;

    #[test]
    fn tokenise_keywords() {
        for expected in Keyword::enumerate() {
            let mut lexer = Lexer::for_str(expected.as_str());
            assert_matches!(
                lexer.next(),
                Some(Ok(Element::Token(Token::Keyword(actual)))) if actual == *expected
            );
            assert_matches!(lexer.next(), None);
        }
    }

    #[test]
    fn tokenise_punctuators() {
        for expected in Punctuator::enumerate() {
            let mut lexer = Lexer::for_str(expected.as_str());
            assert_matches!(
                lexer.next(),
                Some(Ok(Element::Token(Token::Punctuator(actual)))) if actual == *expected
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
    fn tokenise_unclosed_multi_line_comment() {
        let source_code = "/* abc";
        let mut lexer = Lexer::for_str(source_code);
        assert_matches!(lexer.next(), Some(Err(err)) if err.kind() == Some(UnclosedComment));
        assert_matches!(lexer.next(), None);
    }
}
