use super::value::Value;
use crate::ast::{Identifier, SourceLocation};
use std::fmt;

macro_rules! error_kinds {
    ($vis:vis enum $type_name:ident {$(
        $(#[$variant_attr:meta] )*
        $variant:ident($(#[$inner_attr:meta] )* $inner_vis:vis struct $inner:ident {
            $($inner_field_vis:vis $inner_field:ident: $inner_field_ty:ty, )*
        }) => $display_name:literal,
    )*}) => {
        #[derive(Clone, Debug)]
        $vis enum $type_name {$(
            $(#[$variant_attr] )*
            $variant($inner),
        )*}

        impl ::std::error::Error for $type_name {
            fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
                ::std::option::Option::Some(match self {
                    $(Self::$variant(source) => source, )*
                })
            }
        }

        impl ::std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if f.alternate() {
                    match self {
                        $(Self::$variant(source) => write!(f, "{source}"), )*
                    }
                } else {
                    f.write_str(match self {
                        $(Self::$variant(_) => $display_name, )*
                    })
                }
            }
        }

        $($(#[$inner_attr] )*
        #[derive(Clone, Debug)]
        $inner_vis struct $inner {
            $($inner_field: $inner_field_ty, )*
        }

        impl $inner {
            $inner_vis fn new($($inner_field: $inner_field_ty, )*) -> Self {
                Self { $($inner_field, )* }
            }

            $($inner_field_vis fn $inner_field(&self) -> &$inner_field_ty {
                &self.$inner_field
            })*
        }

        impl ::std::error::Error for $inner {}

        impl ::std::convert::From<$inner> for $type_name {
            fn from(inner: $inner) -> Self {
                Self::$variant(inner)
            }
        })*
    };
}

pub type Result<T = Value> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    loc: SourceLocation,
}

impl Error {
    pub fn new(kind: impl Into<ErrorKind>, loc: &SourceLocation) -> Self {
        Self {
            kind: kind.into(),
            loc: loc.clone(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn into_kind(self) -> ErrorKind {
        self.kind
    }

    pub fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind().source()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} at {:#}: {:#}",
            self.kind(),
            self.source_location(),
            self.kind()
        )
    }
}

// TODO: Most of these variants should be thrown as exceptions within the
// interpreter, and shouldn't cause it to exit.
error_kinds!(pub enum ErrorKind {
    Assertion(pub struct AssertionError {
        pub detail_msg: String,
    }) => "assertion failed",

    AssignToConstVariable(pub struct AssignToConstVariableError {
        pub name: Identifier,
    }) => "assign to const variable",

    VariableAlreadyDefined(pub struct VariableAlreadyDefinedError {
        pub name: Identifier,
    }) => "variable already defined",

    VariableNotDefined(pub struct VariableNotDefinedError {
        pub name: Identifier,
    }) => "variable not defined",

    FunctionNotDefined(pub struct FunctionNotDefinedError {
        pub name: Identifier,
    }) => "function not defined",
    NotCallable(#[derive(Default)] pub struct NotCallableError {
    }) => "object or primitive not callable",

    NumericOverflow(#[derive(Default)] pub struct NumericOverflowError {}) => "numeric overflow",

    OutOfHeapSpace(#[derive(Default)] pub struct OutOfHeapSpaceError {}) => "out of heap space",
    OutOfStackSpace(#[derive(Default)] pub struct OutOfStackSpaceError {}) => "out of stack space",

    /// Temporary hack to allow errors to be propagated outside of function calls while still
    /// retaining source location information.
    Boxed(pub struct BoxedError {
        inner: Box<Error>,
    }) => "function call",
});

impl ErrorKind {
    pub fn boxed(inner: Error) -> Self {
        Self::from(BoxedError::from(inner))
    }
}

impl fmt::Display for AssertionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.detail_msg())
    }
}

impl fmt::Display for AssignToConstVariableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` was declared as `const`", self.name())
    }
}

impl fmt::Display for VariableAlreadyDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` already exists in the current scope", self.name())
    }
}

impl fmt::Display for VariableNotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` is not visible from the current scope", self.name())
    }
}

impl fmt::Display for FunctionNotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` is not visible from the current scope", self.name())
    }
}

impl fmt::Display for NotCallableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("object or primitive is not callable")
    }
}

impl fmt::Display for NumericOverflowError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl fmt::Display for OutOfHeapSpaceError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl fmt::Display for OutOfStackSpaceError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl fmt::Display for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl From<Error> for BoxedError {
    fn from(inner: Error) -> Self {
        Self::new(Box::new(inner))
    }
}

#[derive(Clone, Debug)]
pub struct InitialisationError(ErrorKind);

impl InitialisationError {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        self.0
    }
}

impl std::error::Error for InitialisationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.kind())
    }
}

impl fmt::Display for InitialisationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "initialisation error: {}", self.0)
    }
}

impl<T> From<T> for InitialisationError
where
    ErrorKind: From<T>,
{
    fn from(source: T) -> Self {
        Self(ErrorKind::from(source))
    }
}
