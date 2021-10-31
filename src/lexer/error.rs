use std::fmt;

pub type LexResult<T> = std::result::Result<T, LexicalError>;

#[derive(Clone, Debug)]
pub struct LexicalError {
    kind: LexicalErrorKind,
}

impl LexicalError {
    pub fn new(kind: LexicalErrorKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> LexicalErrorKind {
        self.kind
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{}", self.kind())
        todo!("LexicalError::fmt")
    }
}

impl std::error::Error for LexicalError {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LexicalErrorKind {}

impl fmt::Display for LexicalErrorKind {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        todo!("LexicalErrorKind::fmt: {:?}", self)
    }
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
