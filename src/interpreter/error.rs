use crate::ast::Value;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    AssertionFailed(AssertionFailedError),
    AssignToConstVariable(AssignToConstVariableError),
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
            Self::AssignToConstVariable(ref source) => source,
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
pub struct AssignToConstVariableError;

impl fmt::Display for AssignToConstVariableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(r#"Cannot assign to a variable declared as "const""#)
    }
}

impl std::error::Error for AssignToConstVariableError {}

impl From<AssignToConstVariableError> for Error {
    fn from(source: AssignToConstVariableError) -> Self {
        Self::AssignToConstVariable(source)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableAlreadyDefinedError;

impl fmt::Display for VariableAlreadyDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("A variable with the same name is already defined in the current scope")
    }
}

impl std::error::Error for VariableAlreadyDefinedError {}

impl From<VariableAlreadyDefinedError> for Error {
    fn from(source: VariableAlreadyDefinedError) -> Self {
        Self::VariableAlreadyDefined(source)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableNotDefinedError;

impl fmt::Display for VariableNotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Variable not defined in the current scope")
    }
}

impl std::error::Error for VariableNotDefinedError {}

impl From<VariableNotDefinedError> for Error {
    fn from(source: VariableNotDefinedError) -> Self {
        Self::VariableNotDefined(source)
    }
}
