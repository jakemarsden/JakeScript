use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Literal {
    Boolean(bool),
    Null,
    Numeric(NumericLiteral),
    // TODO: Store string literals in the constant pool.
    String(StringLiteral),
}

/// Numeric literals are **always unsigned**, but can be made negative at
/// runtime with the negation unary operator.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum NumericLiteral {
    Float(f64),
    Int(u64),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StringLiteral {
    pub value: Box<str>,
}
