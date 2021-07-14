use crate::util::Stream;
use std::iter::Iterator;
use std::str::FromStr;

pub use token::*;

mod token;

/// ZERO WIDTH NON-JOINER
const ZWNJ: char = '\u{200C}';
/// ZERO WIDTH JOINER
const ZWJ: char = '\u{200D}';

pub struct Lexer(Stream<char>);

impl Lexer {
    pub fn for_str(source: &str) -> Self {
        Self::for_chars(source.chars())
    }

    pub fn for_chars(source: impl Iterator<Item = char>) -> Self {
        Self(Stream::new(source))
    }

    fn consume_whitespace(&mut self) {
        self.0.consume_while(|ch| ch.is_whitespace());
    }

    fn consume_symbol(&mut self) -> Option<Token> {
        // Order is important. For two symbols which start with the same substring, the longer one
        // needs to be checked first.
        let symbols = [
            Symbol::DoubleAmpersand,
            Symbol::AmpersandEqual,
            Symbol::Ampersand,
            Symbol::DoubleAsteriskEqual,
            Symbol::DoubleAsterisk,
            Symbol::AsteriskEqual,
            Symbol::Asterisk,
            Symbol::BangDoubleEqual,
            Symbol::BangEqual,
            Symbol::Bang,
            Symbol::CaretEqual,
            Symbol::Caret,
            Symbol::CloseBrace,
            Symbol::CloseBracket,
            Symbol::CloseParen,
            Symbol::Colon,
            Symbol::Comma,
            Symbol::TripleDot,
            Symbol::Dot,
            Symbol::TripleEqual,
            Symbol::DoubleEqual,
            Symbol::EqualMoreThan,
            Symbol::Equal,
            Symbol::DoubleLessThanEqual,
            Symbol::DoubleLessThan,
            Symbol::LessThanEqual,
            Symbol::LessThan,
            Symbol::DoubleMinus,
            Symbol::MinusEqual,
            Symbol::Minus,
            Symbol::TripleMoreThanEqual,
            Symbol::TripleMoreThan,
            Symbol::DoubleMoreThanEqual,
            Symbol::DoubleMoreThan,
            Symbol::MoreThanEqual,
            Symbol::MoreThan,
            Symbol::OpenBrace,
            Symbol::OpenBracket,
            Symbol::OpenParen,
            Symbol::PercentEqual,
            Symbol::Percent,
            Symbol::DoublePipe,
            Symbol::PipeEqual,
            Symbol::Pipe,
            Symbol::DoublePlus,
            Symbol::PlusEqual,
            Symbol::Plus,
            Symbol::DoubleQuestion,
            Symbol::Question,
            Symbol::Semicolon,
            Symbol::SlashEqual,
            Symbol::Slash,
            Symbol::Tilde,
        ];
        for symbol in symbols {
            if self.0.consume_str(symbol.to_str()) {
                return Some(Token::Symbol(symbol));
            }
        }
        None
    }

    fn consume_string_literal(&mut self) -> Option<Token> {
        // Opening quote
        let qt = self.0.consume_if(|ch| *ch == '"' || *ch == '\'')?;

        // Optimisation: avoid alloc for empty string literals
        if self.0.consume_eq(&qt).is_some() {
            return Some(Token::Literal(Literal::String(String::with_capacity(0))));
        }

        let mut content = String::new();
        let mut escaped = false;
        Some(loop {
            match self.0.peek() {
                Some(ch) if *ch == qt && !escaped => {
                    self.0.consume_exact(&qt);
                    break Token::Literal(Literal::String(content));
                }
                Some('\\') if !escaped => {
                    self.0.consume_exact(&'\\');
                    escaped = true;
                }
                Some('\n') | None => break Token::Invalid(LexicalError::UnclosedStringLiteral),
                Some(ch) => {
                    content.push(*ch);
                    escaped = false;
                    self.0.advance();
                }
            }
        })
    }

    fn consume_numeric_literal(&mut self) -> Option<Token> {
        let ch = self.0.consume_if(|ch| ch.is_ascii_digit())?;

        let mut content = String::new();
        content.push(ch);

        // FIXME: Retun error if no space between numeric literal and something else
        while let Some(ch) = self.0.consume_if(|ch| ch.is_ascii_digit()) {
            content.push(ch);
        }

        Some(match u64::from_str(&content) {
            Ok(num) => Token::Literal(Literal::Numeric(num)),
            Err(err) => Token::Invalid(LexicalError::BadNumericLiteral(err)),
        })
    }

    fn consume_identifier_or_keyword_or_null_or_bool(&mut self) -> Option<Token> {
        fn is_identifier_start(ch: &char) -> bool {
            // FIXME: Acutally check if the character has the "ID_Start" Unicode property
            let has_id_start = ch.is_ascii_alphabetic();
            has_id_start || *ch == '$' || *ch == '_'
        }

        fn is_identifier_part(ch: &char) -> bool {
            // FIXME: Actually check if the character has the "ID_Continue" Unicode property
            let has_id_continue = ch.is_ascii_alphabetic() || ch.is_ascii_digit() || *ch == '_';
            has_id_continue || *ch == '$' || *ch == ZWNJ || *ch == ZWJ
        }

        let ch = self.0.consume_if(is_identifier_start)?;
        let mut content = String::new();
        content.push(ch);

        while let Some(ch) = self.0.consume_if(is_identifier_part) {
            content.push(ch);
        }

        Some(if let Ok(keyword) = Keyword::from_str(&content) {
            Token::Keyword(keyword)
        } else if &content == "null" {
            Token::Literal(Literal::Null)
        } else if &content == "true" {
            Token::Literal(Literal::Boolean(true))
        } else if &content == "false" {
            Token::Literal(Literal::Boolean(false))
        } else {
            Token::Identifier(content)
        })
    }
}

/// ```rust
/// # use jakescript::lexer::*;
/// let source = r##"100 + 50;"##;
/// let mut lexer = Lexer::for_str(source);
///
/// assert_eq!(lexer.next(), Some(Token::Literal(Literal::Numeric(100))));
/// assert_eq!(lexer.next(), Some(Token::Symbol(Symbol::Plus)));
/// assert_eq!(lexer.next(), Some(Token::Literal(Literal::Numeric(50))));
/// assert_eq!(lexer.next(), Some(Token::Symbol(Symbol::Semicolon)));
/// assert_eq!(lexer.next(), None);
/// assert_eq!(lexer.next(), None);
/// ```
///
/// ```rust
/// # use jakescript::lexer::*;
/// let source = r##"
/// let a = 100;
/// let b = 50;
/// a + b;
/// "##;
///
/// let mut lexer = Lexer::for_str(source);
/// assert_eq!(
///     lexer.collect::<Vec<_>>(),
///     vec![
///         Token::Keyword(Keyword::Let),
///         Token::Identifier("a".to_owned()),
///         Token::Symbol(Symbol::Equal),
///         Token::Literal(Literal::Numeric(100)),
///         Token::Symbol(Symbol::Semicolon),
///         Token::Keyword(Keyword::Let),
///         Token::Identifier("b".to_owned()),
///         Token::Symbol(Symbol::Equal),
///         Token::Literal(Literal::Numeric(50)),
///         Token::Symbol(Symbol::Semicolon),
///         Token::Identifier("a".to_owned()),
///         Token::Symbol(Symbol::Plus),
///         Token::Identifier("b".to_owned()),
///         Token::Symbol(Symbol::Semicolon),
///     ]
/// );
/// ```
///
/// ```rust
/// # use jakescript::lexer::*;
/// let source = r##"
/// let x = 0;
/// while x < 3 {
///     x = x + 1;
/// }
/// "##;
///
/// let mut lexer = Lexer::for_str(source);
/// assert_eq!(
///     lexer.collect::<Vec<_>>(),
///     vec![
///         Token::Keyword(Keyword::Let),
///         Token::Identifier("x".to_owned()),
///         Token::Symbol(Symbol::Equal),
///         Token::Literal(Literal::Numeric(0)),
///         Token::Symbol(Symbol::Semicolon),
///
///         Token::Keyword(Keyword::While),
///         //Token::Symbol(Symbol::OpenParen),
///         Token::Identifier("x".to_owned()),
///         Token::Symbol(Symbol::LessThan),
///         Token::Literal(Literal::Numeric(3)),
///         //Token::Symbol(Symbol::CloseParen),
///         Token::Symbol(Symbol::OpenBrace),
///
///         Token::Identifier("x".to_owned()),
///         Token::Symbol(Symbol::Equal),
///         Token::Identifier("x".to_owned()),
///         Token::Symbol(Symbol::Plus),
///         Token::Literal(Literal::Numeric(1)),
///         Token::Symbol(Symbol::Semicolon),
///
///         Token::Symbol(Symbol::CloseBrace),
///     ]
/// );
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespace();
        let ch = *self.0.peek()?;

        Some(if let Some(token) = self.consume_symbol() {
            token
        } else if let Some(token) = self.consume_string_literal() {
            token
        } else if let Some(token) = self.consume_numeric_literal() {
            token
        } else if let Some(token) = self.consume_identifier_or_keyword_or_null_or_bool() {
            token
        } else {
            todo!("{}", ch)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenise_keywords() {
        for keyword in Keyword::VALUES {
            let mut lexer = Lexer::for_str(keyword.to_str());
            assert_eq!(lexer.next(), Some(Token::Keyword(keyword)));
            assert_eq!(lexer.next(), None);
        }
    }

    #[test]
    fn tokenise_symbols() {
        for symbol in Symbol::VALUES {
            let mut lexer = Lexer::for_str(symbol.to_str());
            assert_eq!(lexer.next(), Some(Token::Symbol(symbol)));
            assert_eq!(lexer.next(), None);
        }
    }

    #[test]
    fn tokenise_string_literal() {
        fn check_valid(source: &str, expected: &str) {
            check(
                source,
                Some(Token::Literal(Literal::String(expected.to_owned()))),
            );
        }

        fn check(source: &str, expected: Option<Token>) {
            let mut lexer = Lexer::for_str(source);
            assert_eq!(lexer.next(), expected);
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
