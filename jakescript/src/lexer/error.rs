use crate::token::Element;
use std::{fmt, io};

pub type Result<T = Element> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(ErrorInner);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    DigitFollowingNumericLiteral,
    IdentifierFollowingNumericLiteral,
    UnclosedComment,
}

#[derive(Debug)]
enum ErrorInner {
    Normal(ErrorKind),
    Io(io::Error),
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self(ErrorInner::Normal(kind))
    }

    fn io(source: io::Error) -> Self {
        Self(ErrorInner::Io(source))
    }

    pub fn kind(&self) -> Option<ErrorKind> {
        match self.inner() {
            ErrorInner::Normal(kind) => Some(*kind),
            ErrorInner::Io(..) => None,
        }
    }

    fn inner(&self) -> &ErrorInner {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner() {
            ErrorInner::Normal(kind) => write!(f, "{}", kind),
            ErrorInner::Io(source) => write!(f, "IO error: {}", source),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.inner() {
            ErrorInner::Normal(..) => None,
            ErrorInner::Io(source) => Some(source),
        }
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Self::io(source)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::DigitFollowingNumericLiteral => "Digit following numeric literal",
            Self::IdentifierFollowingNumericLiteral => "Identifier following numeric literal",
            Self::UnclosedComment => "Unclosed comment",
        })
    }
}
