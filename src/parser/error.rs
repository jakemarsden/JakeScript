use crate::ast::Program;
use crate::lexer::{LexicalError, Token};
use std::fmt;

pub type ParseResult<T = Program> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError(ParseErrorKind);

impl ParseError {
    pub fn lexical(source: LexicalError) -> Self {
        Self(ParseErrorKind::Lexical(source))
    }

    pub fn unclosed_block() -> Self {
        Self(ParseErrorKind::UnclosedBlock)
    }

    pub fn unexpected_eoi() -> Self {
        Self(ParseErrorKind::UnexpectedEoi)
    }

    pub fn unexpected_token(expected: Vec<Token>, actual: Option<Token>) -> Self {
        Self(ParseErrorKind::UnexpectedToken(expected, actual))
    }

    pub fn kind(&self) -> &ParseErrorKind {
        &self.0
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ParseErrorKind::Lexical(source) => write!(f, "Lexical error: {}", source),
            ParseErrorKind::UnclosedBlock => f.write_str("Unclosed block"),
            ParseErrorKind::UnexpectedEoi => write!(f, "Unexpected end of input"),
            ParseErrorKind::UnexpectedToken(expected, Some(actual)) => write!(
                f,
                "Unexpected token: expected one of {:?} but was {}",
                expected, actual
            ),
            ParseErrorKind::UnexpectedToken(expected, None) => {
                write!(
                    f,
                    "Unexpected token: expected one of {:?} but was <end>",
                    expected
                )
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind() {
            ParseErrorKind::Lexical(source) => Some(source),
            ParseErrorKind::UnclosedBlock
            | ParseErrorKind::UnexpectedEoi
            | ParseErrorKind::UnexpectedToken(..) => None,
        }
    }
}

impl From<LexicalError> for ParseError {
    fn from(source: LexicalError) -> Self {
        Self::lexical(source)
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    Lexical(LexicalError),
    UnclosedBlock,
    UnexpectedEoi,
    UnexpectedToken(Vec<Token>, Option<Token>),
}
