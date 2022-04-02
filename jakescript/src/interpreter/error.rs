use super::value::Value;
use crate::token::SourceLocation;
use std::fmt;

pub type Result<T = Value> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    loc: SourceLocation,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    Assertion(AssertionError),
    AssignToConstVariable(AssignToConstVariableError),
    FunctionNotDefined(FunctionNotDefinedError),
    NotCallable(NotCallableError),
    NumericOverflow(NumericOverflowError),
    OutOfMemory(OutOfMemoryError),
    VariableAlreadyDefined(VariableAlreadyDefinedError),
    VariableNotDefined(VariableNotDefinedError),
}

#[derive(Clone, Debug)]
pub struct AssertionError {
    detail_msg: String,
}

#[derive(Clone, Debug)]
pub struct AssignToConstVariableError;

#[derive(Clone, Debug)]
pub struct VariableAlreadyDefinedError;

#[derive(Clone, Debug)]
pub struct VariableNotDefinedError;

#[derive(Clone, Debug)]
pub struct FunctionNotDefinedError;

#[derive(Clone, Debug)]
pub struct NotCallableError;

#[derive(Clone, Debug)]
pub struct NumericOverflowError;

#[derive(Clone, Debug)]
pub struct OutOfMemoryError;

#[derive(Clone, Debug)]
pub struct InitialisationError(ErrorKind);

impl Error {
    pub fn new(source: impl Into<ErrorKind>, loc: &SourceLocation) -> Self {
        Self {
            kind: source.into(),
            loc: loc.clone(),
        }
    }

    pub fn into_kind(self) -> ErrorKind {
        self.kind
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.source_location(), self.kind())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self.kind() {
            ErrorKind::Assertion(source) => source,
            ErrorKind::AssignToConstVariable(source) => source,
            ErrorKind::FunctionNotDefined(source) => source,
            ErrorKind::NumericOverflow(source) => source,
            ErrorKind::NotCallable(source) => source,
            ErrorKind::OutOfMemory(source) => source,
            ErrorKind::VariableAlreadyDefined(source) => source,
            ErrorKind::VariableNotDefined(source) => source,
        })
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Assertion(source) => write!(f, "{}", source),
            Self::AssignToConstVariable(source) => write!(f, "{}", source),
            Self::FunctionNotDefined(source) => write!(f, "{}", source),
            Self::NotCallable(source) => write!(f, "{}", source),
            Self::NumericOverflow(source) => write!(f, "{}", source),
            Self::OutOfMemory(source) => write!(f, "{}", source),
            Self::VariableAlreadyDefined(source) => write!(f, "{}", source),
            Self::VariableNotDefined(source) => write!(f, "{}", source),
        }
    }
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

impl From<AssertionError> for ErrorKind {
    fn from(source: AssertionError) -> Self {
        Self::Assertion(source)
    }
}

impl fmt::Display for AssignToConstVariableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(r#"Cannot assign to a variable declared as "const""#)
    }
}

impl std::error::Error for AssignToConstVariableError {}

impl From<AssignToConstVariableError> for ErrorKind {
    fn from(source: AssignToConstVariableError) -> Self {
        Self::AssignToConstVariable(source)
    }
}

impl fmt::Display for VariableAlreadyDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("A variable with the same name is already defined in the current scope")
    }
}

impl std::error::Error for VariableAlreadyDefinedError {}

impl From<VariableAlreadyDefinedError> for ErrorKind {
    fn from(source: VariableAlreadyDefinedError) -> Self {
        Self::VariableAlreadyDefined(source)
    }
}

impl fmt::Display for VariableNotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Variable not defined in the current scope")
    }
}

impl std::error::Error for VariableNotDefinedError {}

impl From<VariableNotDefinedError> for ErrorKind {
    fn from(source: VariableNotDefinedError) -> Self {
        Self::VariableNotDefined(source)
    }
}

impl fmt::Display for FunctionNotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Function not defined in the current scope")
    }
}

impl std::error::Error for FunctionNotDefinedError {}

impl From<FunctionNotDefinedError> for ErrorKind {
    fn from(source: FunctionNotDefinedError) -> Self {
        Self::FunctionNotDefined(source)
    }
}

impl fmt::Display for NotCallableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Object or primitive not callable")
    }
}

impl std::error::Error for NotCallableError {}

impl From<NotCallableError> for ErrorKind {
    fn from(source: NotCallableError) -> Self {
        Self::NotCallable(source)
    }
}

impl fmt::Display for NumericOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Numeric overflow")
    }
}

impl std::error::Error for NumericOverflowError {}

impl From<NumericOverflowError> for ErrorKind {
    fn from(source: NumericOverflowError) -> Self {
        Self::NumericOverflow(source)
    }
}

impl fmt::Display for OutOfMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Out of memory")
    }
}

impl std::error::Error for OutOfMemoryError {}

impl From<OutOfMemoryError> for ErrorKind {
    fn from(source: OutOfMemoryError) -> Self {
        Self::OutOfMemory(source)
    }
}

impl InitialisationError {
    pub fn into_kind(self) -> ErrorKind {
        self.0
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for InitialisationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Initialisation error: {}", self.0)
    }
}

impl std::error::Error for InitialisationError {}

impl<T> From<T> for InitialisationError
where
    ErrorKind: From<T>,
{
    fn from(source: T) -> Self {
        Self(ErrorKind::from(source))
    }
}
