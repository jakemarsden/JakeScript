pub use keyword::*;
pub use literal::*;
pub use punctuator::*;

use crate::str::NonEmptyString;
use std::fmt;
use symbol::*;

pub mod symbol;

mod keyword;
mod literal;
mod punctuator;

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Token(Token),
    Comment(Comment),
    LineTerminator(LineTerminator),
    Whitespace(char),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(NonEmptyString),
    Keyword(Keyword),
    Literal(Literal),
    Punctuator(Punctuator),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Comment {
    SingleLine(String),
    MultiLine(String),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LineTerminator {
    /// Carriage return + line feed
    Crlf,
    /// Carriage return
    Cr,
    /// Line feed
    Lf,
    /// Line separator
    Ls,
    /// Paragraph separator
    Ps,
}

impl Element {
    pub fn token(self) -> Option<Token> {
        match self {
            Self::Token(token) => Some(token),
            Self::Comment(..) | Self::LineTerminator(..) | Self::Whitespace(..) => None,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Token(token) => write!(f, "{}", token),
            Self::Comment(content) => write!(f, "{}", content),
            Self::LineTerminator(content) => write!(f, "{}", content),
            Self::Whitespace(content) => write!(f, "{}", content),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(it) => write!(f, "{}", it),
            Self::Keyword(it) => write!(f, "{}", it),
            Self::Literal(it) => write!(f, "{}", it),
            Self::Punctuator(it) => write!(f, "{}", it),
        }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SingleLine(content) => write!(f, "//{}", content),
            Self::MultiLine(content) => write!(f, "/*{}*/", content),
        }
    }
}

impl LineTerminator {
    pub fn into_chars(self: LineTerminator) -> (char, Option<char>) {
        match self {
            Self::Crlf => (CR, Some(LF)),
            Self::Cr => (CR, None),
            Self::Lf => (LF, None),
            Self::Ls => (LS, None),
            Self::Ps => (PS, None),
        }
    }
}

impl fmt::Display for LineTerminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;
        match self.into_chars() {
            (ch0, Some(ch1)) => write!(f, "{}{}", ch0, ch1),
            (ch0, None) => f.write_char(ch0),
        }
    }
}
