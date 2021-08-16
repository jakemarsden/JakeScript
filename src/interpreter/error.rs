use crate::ast::Expression;
use crate::interpreter::value::Value;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    AssertionFailed(AssertionFailedError),
    AssignToConstVariable(AssignToConstVariableError),
    FunctionAlreadyDefined(FunctionAlreadyDefinedError),
    FunctionArgumentMismatch(FunctionArgumentMismatchError),
    FunctionNotDefined(FunctionNotDefinedError),
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
            Self::FunctionAlreadyDefined(ref source) => source,
            Self::FunctionArgumentMismatch(ref source) => source,
            Self::FunctionNotDefined(ref source) => source,
            Self::VariableAlreadyDefined(ref source) => source,
            Self::VariableNotDefined(ref source) => source,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssertionFailedError {
    condition: Expression,
    value: Value,
}

impl AssertionFailedError {
    pub fn new(condition: Expression, value: Value) -> Self {
        Self { condition, value }
    }
}

impl fmt::Display for AssertionFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Assertion failed (evaluated to `{:?}`): {:#?}",
            self.value, self.condition
        )
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

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionAlreadyDefinedError;

impl fmt::Display for FunctionAlreadyDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Function already defined in the current scope")
    }
}

impl std::error::Error for FunctionAlreadyDefinedError {}

impl From<FunctionAlreadyDefinedError> for Error {
    fn from(source: FunctionAlreadyDefinedError) -> Self {
        Self::FunctionAlreadyDefined(source)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionArgumentMismatchError;

impl fmt::Display for FunctionArgumentMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Function arguments don't match declared parameters")
    }
}

impl std::error::Error for FunctionArgumentMismatchError {}

impl From<FunctionArgumentMismatchError> for Error {
    fn from(source: FunctionArgumentMismatchError) -> Self {
        Self::FunctionArgumentMismatch(source)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionNotDefinedError;

impl fmt::Display for FunctionNotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Function not defined in the current scope")
    }
}

impl std::error::Error for FunctionNotDefinedError {}

impl From<FunctionNotDefinedError> for Error {
    fn from(source: FunctionNotDefinedError) -> Self {
        Self::FunctionNotDefined(source)
    }
}
