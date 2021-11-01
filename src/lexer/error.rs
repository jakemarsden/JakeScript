use std::{fmt, io};

pub type LexicalResult<T> = std::result::Result<T, LexicalError>;

#[derive(Debug)]
pub struct LexicalError(LexicalErrorInner);

impl LexicalError {
    pub fn new(kind: LexicalErrorKind) -> Self {
        Self(LexicalErrorInner::Normal(kind))
    }

    fn io(source: io::Error) -> Self {
        Self(LexicalErrorInner::Io(source))
    }

    pub fn kind(&self) -> Option<LexicalErrorKind> {
        match self.inner() {
            LexicalErrorInner::Normal(kind) => Some(*kind),
            LexicalErrorInner::Io(..) => None,
        }
    }

    fn inner(&self) -> &LexicalErrorInner {
        &self.0
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner() {
            LexicalErrorInner::Normal(kind) => write!(f, "{}", kind),
            LexicalErrorInner::Io(source) => write!(f, "IO error: {}", source),
        }
    }
}

impl std::error::Error for LexicalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.inner() {
            LexicalErrorInner::Normal(..) => None,
            LexicalErrorInner::Io(source) => Some(source),
        }
    }
}

impl From<io::Error> for LexicalError {
    fn from(source: io::Error) -> Self {
        Self::io(source)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LexicalErrorKind {
    IllegalStringLiteralEscapeSequence,
    UnclosedComment,
    UnclosedStringLiteral,
}

impl fmt::Display for LexicalErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::IllegalStringLiteralEscapeSequence => "Illegal escape sequence in string literal",
            Self::UnclosedComment => "Unclosed comment",
            Self::UnclosedStringLiteral => "Unclosed string literal",
        })
    }
}

#[derive(Debug)]
enum LexicalErrorInner {
    Normal(LexicalErrorKind),
    Io(io::Error),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BadKeywordError;

impl fmt::Display for BadKeywordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bad keyword")
    }
}

impl std::error::Error for BadKeywordError {}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BadPunctuatorError;

impl fmt::Display for BadPunctuatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bad punctuator")
    }
}

impl std::error::Error for BadPunctuatorError {}
