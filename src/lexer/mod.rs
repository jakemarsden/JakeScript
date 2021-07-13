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

    fn consume_character_literal(&mut self) -> Option<Token> {
        Some(match self.consume_quoted_literal('\'') {
            Some(Ok(content)) if content.len() == 1 => {
                let ch = content.chars().next().unwrap();
                Token::Literal(Literal::Character(ch))
            }
            Some(Ok(_)) => Token::Invalid(LexError::NotSingleCharacter),
            Some(Err(())) => Token::Invalid(LexError::UnclosedCharacterLiteral),
            None => return None,
        })
    }

    fn consume_string_literal(&mut self) -> Option<Token> {
        Some(match self.consume_quoted_literal('"') {
            Some(Ok(content)) => Token::Literal(Literal::String(content)),
            Some(Err(())) => Token::Invalid(LexError::UnclosedStringLiteral),
            None => return None,
        })
    }

    fn consume_quoted_literal(&mut self, qt: char) -> Option<Result<String, ()>> {
        // Opening quote
        self.0.consume_if(|ch| ch == &qt)?;

        if self.0.consume_if(|ch| ch == &qt).is_some() {
            // Empty string literal
            return Some(Ok(String::with_capacity(0)));
        }

        let mut content = String::new();
        let mut escaped = false;
        loop {
            match self.0.peek() {
                Some(ch) if *ch == qt && !escaped => {
                    self.0.consume_exact(&qt);
                    break Some(Ok(content));
                }
                Some('\\') if !escaped => {
                    self.0.consume_exact(&'\\');
                    escaped = true;
                }
                Some('\n') | None => break Some(Err(())),
                Some(ch) => {
                    if escaped {
                        escaped = false;
                        content.push('\\');
                    }
                    content.push(*ch);
                    self.0.advance();
                }
            }
        }
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
            Ok(num) => Token::Literal(Literal::Integer(num)),
            Err(err) => Token::Invalid(LexError::BadNumericLiteral(err)),
        })
    }

    fn consume_identifier_or_keyword(&mut self) -> Option<Token> {
        fn is_identifier_start(ch: &char) -> bool {
            ch.is_ascii_alphabetic() || *ch == '_' || *ch == '$'
        }

        fn is_identifier_middle(ch: &char) -> bool {
            is_identifier_start(ch) || ch.is_ascii_digit()
        }

        let ch = self.0.consume_if(is_identifier_start)?;
        let mut content = String::new();
        content.push(ch);

        while let Some(ch) = self.0.consume_if(is_identifier_middle) {
            content.push(ch);
        }

        Some(if let Ok(keyword) = Keyword::from_str(&content) {
            Token::Keyword(keyword)
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
/// assert_eq!(lexer.next(), Some(Token::Literal(Literal::Integer(100))));
/// assert_eq!(lexer.next(), Some(Token::Symbol(Symbol::Plus)));
/// assert_eq!(lexer.next(), Some(Token::Literal(Literal::Integer(50))));
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
///         Token::Literal(Literal::Integer(100)),
///         Token::Symbol(Symbol::Semicolon),
///         Token::Keyword(Keyword::Let),
///         Token::Identifier("b".to_owned()),
///         Token::Symbol(Symbol::Equal),
///         Token::Literal(Literal::Integer(50)),
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
///         Token::Literal(Literal::Integer(0)),
///         Token::Symbol(Symbol::Semicolon),
///
///         Token::Keyword(Keyword::While),
///         //Token::Symbol(Symbol::OpenParen),
///         Token::Identifier("x".to_owned()),
///         Token::Symbol(Symbol::LessThan),
///         Token::Literal(Literal::Integer(3)),
///         //Token::Symbol(Symbol::CloseParen),
///         Token::Symbol(Symbol::OpenBrace),
///
///         Token::Identifier("x".to_owned()),
///         Token::Symbol(Symbol::Equal),
///         Token::Identifier("x".to_owned()),
///         Token::Symbol(Symbol::Plus),
///         Token::Literal(Literal::Integer(1)),
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
        } else if let Some(token) = self.consume_character_literal() {
            token
        } else if let Some(token) = self.consume_string_literal() {
            token
        } else if let Some(token) = self.consume_numeric_literal() {
            token
        } else if let Some(token) = self.consume_identifier_or_keyword() {
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
}
