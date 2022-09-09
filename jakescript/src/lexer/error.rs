use crate::token::{Element, SourceLocation};
use std::{fmt, io};

pub type Result<T = Element> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    loc: SourceLocation,
}

impl Error {
    pub fn new(kind: impl Into<ErrorKind>, loc: &SourceLocation) -> Self {
        Self {
            kind: kind.into(),
            loc: loc.clone(),
        }
    }

    pub fn into_kind(self) -> ErrorKind {
        self.kind
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.source_location(), self.kind())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind() {
            ErrorKind::Io(source) => Some(source),
            ErrorKind::DigitFollowingNumericLiteral
            | ErrorKind::IdentifierFollowingNumericLiteral
            | ErrorKind::UnclosedComment => None,
        }
    }
}

impl From<(io::Error, SourceLocation)> for Error {
    fn from((source, loc): (io::Error, SourceLocation)) -> Self {
        Self::new(source, &loc)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    DigitFollowingNumericLiteral,
    IdentifierFollowingNumericLiteral,
    UnclosedComment,
    Io(io::Error),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DigitFollowingNumericLiteral => f.write_str("Digit following numeric literal"),
            Self::IdentifierFollowingNumericLiteral => {
                f.write_str("Identifier following numeric literal")
            }
            Self::UnclosedComment => f.write_str("Unclosed comment"),
            Self::Io(source) => write!(f, "IO error: {}", source),
        }
    }
}

impl From<io::Error> for ErrorKind {
    fn from(source: io::Error) -> Self {
        Self::Io(source)
    }
}
