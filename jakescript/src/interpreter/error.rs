use crate::ast::Expression;
use crate::interpreter::value::Value;
use std::fmt;

pub type Result<T = Value> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    AssertionFailed(AssertionFailedError),
    AssignToConstVariable(AssignToConstVariableError),
    FunctionArgumentMismatch(FunctionArgumentMismatchError),
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
            Self::AssertionFailed(ref source) => source,
            Self::AssignToConstVariable(ref source) => source,
            Self::FunctionArgumentMismatch(ref source) => source,
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