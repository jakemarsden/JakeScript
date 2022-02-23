use crate::interpreter::value::Value;
use std::fmt;

pub type Result<T = Value> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    Assertion(AssertionError),
    AssignToConstVariable(AssignToConstVariableError),
    FunctionNotDefined(FunctionNotDefinedError),
    NotCallable(NotCallableError),
    NumericOverflow(NumericOverflowError),
    OutOfMemory(OutOfMemoryError),
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
            Self::Assertion(ref source) => source,
            Self::AssignToConstVariable(ref source) => source,
            Self::FunctionNotDefined(ref source) => source,
            Self::NumericOverflow(ref source) => source,
            Self::NotCallable(ref source) => source,
            Self::OutOfMemory(ref source) => source,
            Self::VariableAlreadyDefined(ref source) => source,
            Self::VariableNotDefined(ref source) => source,
        })
    }
}

#[derive(Clone, Debug)]
pub struct AssertionError {
    detail_msg: String,
}

impl AssertionError {
    pub fn new(detail_msg: String) -> Self {
        Self { detail_msg }
    }

    pub fn detail_msg(&self) -> &str {
        &self.detail_msg
    }
}

impl fmt::Display for AssertionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assertion failed: {}", self.detail_msg)
    }
}

impl std::error::Error for AssertionError {}

impl From<AssertionError> for Error {
    fn from(source: AssertionError) -> Self {
        Self::Assertion(source)
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct NotCallableError;

impl fmt::Display for NotCallableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Object or primitive not callable")
    }
}

impl std::error::Error for NotCallableError {}

impl From<NotCallableError> for Error {
    fn from(source: NotCallableError) -> Self {
        Self::NotCallable(source)
    }
}

#[derive(Clone, Debug)]
pub struct NumericOverflowError;

impl fmt::Display for NumericOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Numeric overflow")
    }
}

impl std::error::Error for NumericOverflowError {}

impl From<NumericOverflowError> for Error {
    fn from(source: NumericOverflowError) -> Self {
        Self::NumericOverflow(source)
    }
}

#[derive(Clone, Debug)]
pub struct OutOfMemoryError;

impl fmt::Display for OutOfMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Out of memory")
    }
}

impl std::error::Error for OutOfMemoryError {}

impl From<OutOfMemoryError> for Error {
    fn from(source: OutOfMemoryError) -> Self {
        Self::OutOfMemory(source)
    }
}

#[derive(Clone, Debug)]
pub struct InitialisationError(Error);

impl fmt::Display for InitialisationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Initialisation error: {}", self.0)
    }
}

impl std::error::Error for InitialisationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl<T> From<T> for InitialisationError
where
    Error: From<T>,
{
    fn from(source: T) -> Self {
        Self(Error::from(source))
    }
}
