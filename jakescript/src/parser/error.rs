use crate::ast::Script;
use crate::lexer;
use crate::token::Token;
use ansi_term::Style;
use std::fmt;

pub type Result<T = Script> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
pub enum ErrorKind {
    Lexical(lexer::Error),
    UnexpectedEoi(AllowToken),
    UnexpectedToken(AllowToken, Token),
}

#[derive(Debug)]
pub enum AllowToken {
    // TODO: Be more specific in the cases where this is used, and remove if possible
    Unspecified,
    Exactly(Token),
    AnyOf(Token, Token, Vec<Token>),
}

impl Error {
    pub fn lexical(source: lexer::Error) -> Self {
        Self(ErrorKind::Lexical(source))
    }

    pub fn unexpected(expected: AllowToken, actual: Option<Token>) -> Self {
        match actual {
            Some(actual) => Self::unexpected_token(expected, actual),
            None => Self::unexpected_eoi(expected),
        }
    }

    pub fn unexpected_eoi(expected: AllowToken) -> Self {
        Self(ErrorKind::UnexpectedEoi(expected))
    }

    pub fn unexpected_token(expected: AllowToken, actual: Token) -> Self {
        Self(ErrorKind::UnexpectedToken(expected, actual))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind().source()
    }
}

impl From<lexer::Error> for Error {
    fn from(source: lexer::Error) -> Self {
        Self::lexical(source)
    }
}

impl ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorKind::Lexical(source) => Some(source),
            ErrorKind::UnexpectedEoi(..) | ErrorKind::UnexpectedToken(..) => None,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lexical(source) => write!(f, "Lexical error: {}", source),
            Self::UnexpectedEoi(expected) => {
                write!(f, "Unexpected end of input: Expected {}", expected)
            }
            Self::UnexpectedToken(expected, actual) => {
                write!(
                    f,
                    "Unexpected token: Expected {} but was {}",
                    expected,
                    highlight(actual.to_string()),
                )
            }
        }
    }
}

impl fmt::Display for AllowToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unspecified => f.write_str("any token"),
            Self::Exactly(t0) => write!(f, "{}", highlight(t0.to_string())),
            Self::AnyOf(t0, t1, rest) => {
                let mut str = format!(
                    "{} or {}",
                    highlight(t0.to_string()),
                    highlight(t1.to_string())
                );
                for t in rest {
                    str.push_str(" or ");
                    str.push_str(&highlight(t.to_string()).to_string());
                }
                f.write_str(&str)
            }
        }
    }
}

fn highlight<'a>(input: String) -> ansi_term::ANSIGenericString<'a, str> {
    Style::new().bold().paint(input)
}
