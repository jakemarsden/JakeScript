use std::fmt;

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
