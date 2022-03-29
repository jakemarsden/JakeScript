use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Literal {
    Boolean(bool),
    Numeric(NumericLiteral),
    // TODO: Store string literals in the constant pool.
    String(StringLiteral),
    Null,
}

/// Numeric literal tokens are **always unsigned** (but can be made negative at runtime with the
/// negation unary operator).
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum NumericLiteral {
    Int(u64),
    Float(f64),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StringLiteral {
    pub value: String,
}
