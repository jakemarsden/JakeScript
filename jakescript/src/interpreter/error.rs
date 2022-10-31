use super::value::Value;
use crate::token::SourceLocation;
use std::fmt;

pub type Result<T = Value> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    loc: SourceLocation,
}

impl Error {
    pub fn new(source: impl Into<ErrorKind>, loc: &SourceLocation) -> Self {
        Self {
            kind: source.into(),
            loc: loc.clone(),
        }
    }

    pub fn into_kind(self) -> ErrorKind {
        match self.kind {
            ErrorKind::Boxed(box boxed) => boxed.into_kind(),
            kind => kind,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        match self.kind {
            ErrorKind::Boxed(box ref boxed) => boxed.kind(),
            ref kind => kind,
        }
    }

    pub fn source_location(&self) -> &SourceLocation {
        match self.kind {
            ErrorKind::Boxed(box ref boxed) => boxed.source_location(),
            _ => &self.loc,
        }
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
            ErrorKind::Boxed(box boxed) => return boxed.source(),
            ErrorKind::Assertion(source) => source,
            ErrorKind::AssignToConstVariable(source) => source,
            ErrorKind::FunctionNotDefined(source) => source,
            ErrorKind::NumericOverflow(source) => source,
            ErrorKind::NotCallable(source) => source,
            ErrorKind::OutOfMemory(source) => source,
            ErrorKind::OutOfStackSpace(source) => source,
            ErrorKind::VariableAlreadyDefined(source) => source,
            ErrorKind::VariableNotDefined(source) => source,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// Required to be able to "upcast" from an [`Error`] to an [`ErrorKind`].
    Boxed(Box<Error>),

    Assertion(AssertionError),
    AssignToConstVariable(AssignToConstVariableError),
    FunctionNotDefined(FunctionNotDefinedError),
    NotCallable(NotCallableError),
    NumericOverflow(NumericOverflowError),
    OutOfMemory(OutOfMemoryError),
    OutOfStackSpace(OutOfStackSpaceError),
    VariableAlreadyDefined(VariableAlreadyDefinedError),
    VariableNotDefined(VariableNotDefinedError),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boxed(box source) => fmt::Display::fmt(source.kind(), f),
            Self::Assertion(source) => write!(f, "{source}"),
            Self::AssignToConstVariable(source) => write!(f, "{source}"),
            Self::FunctionNotDefined(source) => write!(f, "{source}",),
            Self::NotCallable(source) => write!(f, "{source}",),
            Self::NumericOverflow(source) => write!(f, "{source}",),
            Self::OutOfMemory(source) => write!(f, "{source}",),
            Self::OutOfStackSpace(source) => write!(f, "{source}",),
            Self::VariableAlreadyDefined(source) => write!(f, "{source}",),
            Self::VariableNotDefined(source) => write!(f, "{source}",),
        }
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

impl From<AssertionError> for ErrorKind {
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

impl From<AssignToConstVariableError> for ErrorKind {
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

impl From<VariableAlreadyDefinedError> for ErrorKind {
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

impl From<VariableNotDefinedError> for ErrorKind {
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

impl From<FunctionNotDefinedError> for ErrorKind {
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

impl From<NotCallableError> for ErrorKind {
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

impl From<NumericOverflowError> for ErrorKind {
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

impl From<OutOfMemoryError> for ErrorKind {
    fn from(source: OutOfMemoryError) -> Self {
        Self::OutOfMemory(source)
    }
}

#[derive(Clone, Debug)]
pub struct OutOfStackSpaceError;

impl fmt::Display for OutOfStackSpaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Out of stack space")
    }
}

impl std::error::Error for OutOfStackSpaceError {}

impl From<OutOfStackSpaceError> for ErrorKind {
    fn from(source: OutOfStackSpaceError) -> Self {
        Self::OutOfStackSpace(source)
    }
}

#[derive(Clone, Debug)]
pub struct InitialisationError(ErrorKind);

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
