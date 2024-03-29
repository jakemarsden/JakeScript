use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::token::symbol::*;
use crate::token::*;
use error::ErrorKind::{
    DigitFollowingNumericLiteral, IdentifierFollowingNumericLiteral, UnclosedComment,
};
pub use error::*;
use fallible_iterator::FallibleIterator;
use source::{Fallible, SourceCode};
use std::io;
use std::str::{Chars, FromStr};

mod error;
mod source;
#[cfg(test)]
mod test;

pub struct Lexer<I: FallibleIterator<Item = char, Error = io::Error>> {
    source: SourceCode<I>,
}

impl<'a> Lexer<Fallible<Chars<'a>>> {
    pub fn for_str(source: &'a str, start_loc: SourceLocation) -> Self {
        Self::new(SourceCode::for_str(source, start_loc))
    }
}

impl<I: Iterator<Item = char>> Lexer<Fallible<I>> {
    pub fn for_chars(source: I, start_loc: SourceLocation) -> Lexer<Fallible<I>> {
        Self::new(SourceCode::for_chars(source, start_loc))
    }
}

impl<I: FallibleIterator<Item = char, Error = io::Error>> Lexer<I> {
    pub fn for_chars_fallible(source: I, start_loc: SourceLocation) -> Self {
        Self::new(SourceCode::for_chars_fallible(source, start_loc))
    }

    fn new(source: SourceCode<I>) -> Self {
        Self { source }
    }

    pub fn source_location(&self) -> &SourceLocation {
        self.source.location()
    }

    fn parse_element(&mut self) -> Result {
        let loc = self.source.location().clone();

        Ok(if let Some(it) = self.parse_whitespace()? {
            Element::new_whitespace(it, loc)
        } else if let Some(it) = self.parse_line_terminator()? {
            Element::new_line_terminator(it, loc)
        } else if let Some(it) = self.parse_comment()? {
            Element::new_comment(it, loc)
        } else if let Some(it) = self.parse_token()? {
            Element::new_token(it, loc)
        } else {
            let ch = self.source.peek()?.unwrap();
            todo!("Lexer::parse_element: ch={ch}")
        })
    }

    fn parse_token(&mut self) -> Result<Option<Token>> {
        // TODO: Parse template tokens.
        Ok(if let Some(value) = self.parse_literal()? {
            Some(Token::Literal(value))
        } else if let Some(value) = self.parse_punctuator()? {
            Some(Token::Punctuator(value))
        } else if let Some(value) = self.parse_template()? {
            Some(Token::Template(value))
        } else {
            self.parse_keyword_or_identifier()?
        })
    }

    fn parse_punctuator(&mut self) -> Result<Option<Punctuator>> {
        for value in Punctuator::all_in_lexical_order() {
            if self
                .source
                .advance_over_if_eq(value.as_str().chars())?
                .is_ok()
            {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn parse_literal(&mut self) -> Result<Option<Literal>> {
        Ok(if let Some(()) = self.parse_null_literal()? {
            Some(Literal::Null)
        } else if let Some(value) = self.parse_boolean_literal()? {
            Some(Literal::Boolean(value))
        } else if let Some(value) = self.parse_numeric_literal()? {
            Some(Literal::Numeric(value))
        } else if let Some(value) = self.parse_string_literal()? {
            Some(Literal::String(value))
        } else {
            self.parse_regex_literal()?
                .map(|value| Literal::RegEx(Box::new(value)))
        })
    }

    fn parse_null_literal(&mut self) -> Result<Option<()>> {
        Ok(self.source.advance_over_if_eq("null".chars())?.ok())
    }

    fn parse_boolean_literal(&mut self) -> Result<Option<bool>> {
        Ok(if self.source.advance_over_if_eq("true".chars())?.is_ok() {
            Some(true)
        } else if self.source.advance_over_if_eq("false".chars())?.is_ok() {
            Some(false)
        } else {
            None
        })
    }

    fn parse_numeric_literal(&mut self) -> Result<Option<NumericLiteral>> {
        let value = match self.parse_non_decimal_int_literal()? {
            Some(value) => Some(value),
            None => self.parse_decimal_literal()?,
        };
        if let Some(value) = value {
            // Ensure the character following the numeric literal is valid
            match self.source.peek()? {
                Some(ch) if is_identifier_part(*ch) => Err(Error::new(
                    IdentifierFollowingNumericLiteral,
                    self.source.location(),
                )),
                Some(ch) if ch.is_ascii_digit() => Err(Error::new(
                    DigitFollowingNumericLiteral,
                    self.source.location(),
                )),
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
        if self.source.next_if_eq(&'0')?.is_some() {
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
        if !matches!(self.source.peek()?, Some('0')) {
            return Ok(None);
        }
        let (radix, ch1) = match self.source.peek_nth(1)? {
            Some(ch @ ('b' | 'B')) => (2, *ch),
            Some(ch @ ('o' | 'O')) => (8, *ch),
            Some(ch @ ('x' | 'X')) => (16, *ch),
            _ => return Ok(None),
        };
        if !matches!(self.source.peek_nth(2)?, Some(ch2) if ch2.is_digit(radix)) {
            return Ok(None);
        }
        assert!(self.source.next_if_eq(&'0')?.is_some());
        assert!(self.source.next_if_eq(&ch1)?.is_some());
        let value = self.parse_int_literal_part(radix)?.unwrap();
        Ok(Some(match radix {
            2 => NumericLiteral::BinInt(value),
            8 => NumericLiteral::OctInt(value),
            16 => NumericLiteral::HexInt(value),
            _ => unreachable!("{radix}"),
        }))
    }

    fn parse_int_literal_part(&mut self, radix: u32) -> Result<Option<u64>> {
        let mut present = false;
        let mut value = 0;
        while let Some(ch) = self.source.next_if(|ch| ch.is_digit(radix))? {
            present = true;
            value *= u64::from(radix);
            value += u64::from(ch.to_digit(radix).unwrap());
        }
        Ok(present.then_some(value))
    }

    /// ```plain
    /// StringLiteral::
    ///     " DoubleStringCharacters(opt) "
    ///     ' SingleStringCharacters(opt) '
    fn parse_string_literal(&mut self) -> Result<Option<StringLiteral>> {
        let (kind, value) = if let Some(value) = self.parse_string_literal_impl('\'')? {
            (StringLiteralKind::SingleQuoted, value)
        } else if let Some(value) = self.parse_string_literal_impl('"')? {
            (StringLiteralKind::DoubleQuoted, value)
        } else {
            return Ok(None);
        };
        Ok(Some(StringLiteral { kind, value }))
    }

    fn parse_string_literal_impl(&mut self, qt: char) -> Result<Option<Box<str>>> {
        if self.source.peek()? != Some(&qt) {
            return Ok(None);
        }
        // Optimisation: Avoid allocating for empty string literals.
        if self.source.peek_nth(1)? == Some(&qt) {
            assert!(self.source.next_if_eq(&qt)?.is_some());
            assert!(self.source.next_if_eq(&qt)?.is_some());
            return Ok(Some(Box::from("")));
        }
        let mut escaped = false;
        let mut content = String::new();
        let mut raw_content_len = 0;

        let mut offset = 1;
        loop {
            let ch = if let Some(ch) = self.source.peek_nth(offset)? {
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
                (ch @ &(LS | PS), false) => {
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
                    if *ch == CR && self.source.peek_nth(offset + 1)? == Some(&LF) {
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
        assert!(self.source.next_if_eq(&qt)?.is_some());
        self.source.advance_by(raw_content_len)?.unwrap();
        assert!(self.source.next_if_eq(&qt)?.is_some());
        Ok(Some(content.into_boxed_str()))
    }

    /// ```plain
    /// TemplateLiteral::
    ///     NoSubstitutionTemplate
    ///     SubstitutionTemplate
    ///
    /// NoSubstitutionTemplate::
    ///     ` TemplateCharacters(opt) `
    /// ```
    fn parse_template(&mut self) -> Result<Option<Template>> {
        if !matches!(self.source.peek()?, Some('`')) {
            return Ok(None);
        }
        // TODO: Parse template literals properly (extract the inner ${expression}s for
        // substitution, etc.)
        let value = self.parse_string_literal_impl('`')?.unwrap();
        Ok(Some(Template { value }))
    }

    /// ```plain
    /// RegularExpressionLiteral::
    ///     / RegularExpressionBody / RegularExpressionFlags
    /// ```
    fn parse_regex_literal(&mut self) -> Result<Option<RegExLiteral>> {
        if !matches!(self.source.peek()?, Some('/')) {
            return Ok(None);
        }
        if matches!(self.source.peek_nth(1)?, Some('*')) {
            // Not a valid `RegularExpressionFirstChar`.
            return Ok(None);
        }
        if matches!(self.source.peek_nth(1)?, Some('/')) {
            // Not a valid `RegularExpressionFirstChar`. Empty regexes aren't representable
            // because `//` represents the start of a single-line comment. The
            // spec suggests using `/(?:)/` as a workaround.
            return Ok(None);
        }
        let mut escaped = false;
        let mut in_class = false;
        let mut content = String::new();
        let mut raw_content_len = 0;
        for offset in 1.. {
            let ch = if let Some(ch) = self.source.peek_nth(offset)? {
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
        assert!(self.source.next_if_eq(&'/')?.is_some());
        self.source.advance_by(raw_content_len)?.unwrap();
        assert!(self.source.next_if_eq(&'/')?.is_some());

        let mut flags = Vec::new();
        while let Some(ch) = self.source.next_if(|ch| is_identifier_start(*ch))? {
            flags.push(ch);
        }
        Ok(Some(RegExLiteral {
            content: content.into_boxed_str(),
            flags,
        }))
    }

    fn parse_keyword_or_identifier(&mut self) -> Result<Option<Token>> {
        Ok(self.parse_identifier_name()?.map(|ident_or_keyword| {
            Keyword::from_str(ident_or_keyword.as_ref())
                .map_or_else(|_| Token::Identifier(ident_or_keyword), Token::Keyword)
        }))
    }

    fn parse_identifier_name(&mut self) -> Result<Option<Box<str>>> {
        if let Some(ch0) = self.source.next_if(|ch| is_identifier_start(*ch))? {
            let mut content = String::from(ch0);
            while let Some(ch) = self.source.next_if(|ch| is_identifier_part(*ch))? {
                content.push(ch);
            }
            Ok(Some(content.into_boxed_str()))
        } else {
            Ok(None)
        }
    }

    fn parse_whitespace(&mut self) -> Result<Option<Whitespace>> {
        if let Some(ch0) = self.source.next_if(|ch| is_whitespace(*ch))? {
            let mut content = String::from(ch0);
            while let Some(ch) = self.source.next_if(|ch| is_whitespace(*ch))? {
                content.push(ch);
            }
            Ok(Some(Whitespace::from(content.into_boxed_str())))
        } else {
            Ok(None)
        }
    }

    fn parse_line_terminator(&mut self) -> Result<Option<LineTerminator>> {
        Ok(match self.source.peek()?.copied() {
            Some(CR) if self.source.peek_nth(1)? == Some(&LF) => {
                assert!(self.source.next_if_eq(&CR)?.is_some());
                assert!(self.source.next_if_eq(&LF)?.is_some());
                Some(LineTerminator::Crlf)
            }
            Some(CR) => {
                assert!(self.source.next_if_eq(&CR)?.is_some());
                Some(LineTerminator::Cr)
            }
            Some(LF) => {
                assert!(self.source.next_if_eq(&LF)?.is_some());
                Some(LineTerminator::Lf)
            }
            Some(LS) => {
                assert!(self.source.next_if_eq(&LS)?.is_some());
                Some(LineTerminator::Ls)
            }
            Some(PS) => {
                assert!(self.source.next_if_eq(&PS)?.is_some());
                Some(LineTerminator::Ps)
            }
            Some(_) | None => None,
        })
    }

    fn parse_comment(&mut self) -> Result<Option<Comment>> {
        if let Some(comment) = self.parse_single_line_comment()? {
            Ok(Some(comment))
        } else {
            self.parse_multi_line_comment()
        }
    }

    fn parse_single_line_comment(&mut self) -> Result<Option<Comment>> {
        if self.source.peek()? == Some(&'/') && self.source.peek_nth(1)? == Some(&'/') {
            assert!(self.source.next_if_eq(&'/')?.is_some());
            assert!(self.source.next_if_eq(&'/')?.is_some());
            let mut content = String::new();
            while let Some(ch) = self.source.next_if(|ch| !is_line_terminator(*ch))? {
                content.push(ch);
            }
            Ok(Some(Comment {
                kind: CommentKind::SingleLine,
                value: content,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_multi_line_comment(&mut self) -> Result<Option<Comment>> {
        if self.source.peek()? != Some(&'/') || self.source.peek_nth(1)? != Some(&'*') {
            return Ok(None);
        }
        let mut content = String::new();
        // Number of code points. Different from `content.len()`, which is the number of
        // bytes.
        let mut content_len = 0;
        for offset in 2.. {
            let ch = match self.source.peek_nth(offset)? {
                Some(ch) => *ch,
                None => return Err(Error::new(UnclosedComment, self.source.location())),
            };
            if ch == '*' && self.source.peek_nth(offset + 1)? == Some(&'/') {
                break;
            }
            content.push(ch);
            content_len += 1;
        }
        assert!(self.source.next_if_eq(&'/')?.is_some());
        assert!(self.source.next_if_eq(&'*')?.is_some());
        self.source.advance_by(content_len)?.unwrap();
        assert!(self.source.next_if_eq(&'*')?.is_some());
        assert!(self.source.next_if_eq(&'/')?.is_some());
        Ok(Some(Comment {
            kind: CommentKind::MultiLine,
            value: content,
        }))
    }
}

impl<I: FallibleIterator<Item = char, Error = io::Error>> FallibleIterator for Lexer<I> {
    type Error = Error;
    type Item = Element;

    fn next(&mut self) -> std::result::Result<Option<Self::Item>, Self::Error> {
        if self.source.peek()?.is_some() {
            self.parse_element().map(Some)
        } else {
            Ok(None)
        }
    }
}
