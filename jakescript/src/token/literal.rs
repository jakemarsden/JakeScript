use crate::str::NonEmptyString;
use std::fmt;

// TODO: Support RegEx literals.
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Numeric(NumericLiteral),
    String(StringLiteral),
    RegEx(RegExLiteral),
    Null,
}

/// Numeric literal tokens are **always unsigned** (but can be made negative at runtime with the
/// negation unary operator).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NumericLiteral {
    BinInt(u64),
    OctInt(u64),
    DecInt(u64),
    HexInt(u64),
    Decimal(f64),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringLiteral {
    pub kind: StringLiteralKind,
    pub value: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StringLiteralKind {
    SingleQuoted,
    DoubleQuoted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegExLiteral {
    pub content: NonEmptyString,
    pub flags: Vec<char>,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Numeric(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::RegEx(value) => write!(f, "{}", value),
            Self::Null => f.write_str("null"),
        }
    }
}

impl fmt::Display for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BinInt(value) => write!(f, "{:#b}", value),
            Self::OctInt(value) => write!(f, "{:#o}", value),
            Self::DecInt(value) => write!(f, "{}", value),
            Self::HexInt(value) => write!(f, "{:#x}", value),
            Self::Decimal(value) => write!(f, "{}", value),
        }
    }
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            StringLiteralKind::SingleQuoted => write!(f, r#"'{}'"#, self.value),
            StringLiteralKind::DoubleQuoted => write!(f, r#""{}""#, self.value),
        }
    }
}

impl fmt::Display for RegExLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "/{}/{}",
            self.content,
            self.flags.iter().collect::<String>()
        )
    }
}
