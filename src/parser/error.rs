use crate::ast::Program;
use crate::lexer::LexicalError;
use std::fmt;

pub type ParseResult<T = Program> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct ParseError(ParseErrorInner);

impl ParseError {
    pub fn lexical(source: LexicalError) -> Self {
        Self(ParseErrorInner::Lexical(source))
    }

    fn inner(&self) -> &ParseErrorInner {
        &self.0
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.inner() {
            ParseErrorInner::Lexical(source) => Some(source),
        }
    }
}

impl From<LexicalError> for ParseError {
    fn from(source: LexicalError) -> Self {
        Self::lexical(source)
    }
}

#[derive(Clone, Debug)]
enum ParseErrorInner {
    Lexical(LexicalError),
}

impl fmt::Display for ParseErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lexical(source) => write!(f, "Lexical error: {}", source),
        }
    }
}
