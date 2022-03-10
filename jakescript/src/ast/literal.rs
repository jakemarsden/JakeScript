use super::block::Block;
use super::expression::Expression;
use super::identifier::Identifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum Literal {
    Boolean(bool),
    Numeric(NumericLiteral),
    // TODO: Store string literals in the constant pool.
    String(String),
    Array(Vec<Expression>),
    Function {
        name: Option<Identifier>,
        param_names: Vec<Identifier>,
        body: Block,
    },
    Object(HashMap<Identifier, Expression>),
    Null,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum NumericLiteral {
    /// Numeric literal tokens are **always unsigned** (but can be made negative at runtime with the
    /// negation unary operator).
    Int(u64),
}
