use crate::interpreter::Value;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    AssertionFailed(AssertionFailedError),
    VariableAlreadyDefined(VariableAlreadyDefinedError),
    VariableNotDefined(VariableNotDefinedError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.source().unwrap())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::AssertionFailed(ref source) => source,
            Self::VariableAlreadyDefined(ref source) => source,
            Self::VariableNotDefined(ref source) => source,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssertionFailedError {
    value: Value,
}

impl AssertionFailedError {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl fmt::Display for AssertionFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assertion failed: {}", self.value)
    }
}

impl std::error::Error for AssertionFailedError {}

impl From<AssertionFailedError> for Error {
    fn from(source: AssertionFailedError) -> Self {
        Self::AssertionFailed(source)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableAlreadyDefinedError {
    var_name: String,
}

impl VariableAlreadyDefinedError {
    pub fn new(var_name: String) -> Self {
        Self { var_name }
    }
}

impl fmt::Display for VariableAlreadyDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Variable already defined: "{}""#, self.var_name)
    }
}

impl std::error::Error for VariableAlreadyDefinedError {}

impl From<VariableAlreadyDefinedError> for Error {
    fn from(source: VariableAlreadyDefinedError) -> Self {
        Self::VariableAlreadyDefined(source)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableNotDefinedError {
    var_name: String,
}

impl VariableNotDefinedError {
    pub fn new(var_name: String) -> Self {
        Self { var_name }
    }
}

impl fmt::Display for VariableNotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Variable not defined: "{}""#, self.var_name)
    }
}

impl std::error::Error for VariableNotDefinedError {}

impl From<VariableNotDefinedError> for Error {
    fn from(source: VariableNotDefinedError) -> Self {
        Self::VariableNotDefined(source)
    }
}
