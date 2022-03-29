use super::block::Block;
use super::expression::Expression;
use super::identifier::Identifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum Literal {
    Boolean(bool),
    Numeric(NumericLiteral),
    // TODO: Store string literals in the constant pool.
    String(String),
    Array(ArrayLiteral),
    Function(Box<FunctionLiteral>),
    Object(Box<ObjectLiteral>),
    Null,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum NumericLiteral {
    /// Numeric literal tokens are **always unsigned** (but can be made negative at runtime with the
    /// negation unary operator).
    Int(u64),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArrayLiteral {
    pub declared_elements: Vec<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FunctionLiteral {
    pub name: Option<Identifier>,
    pub param_names: Vec<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectLiteral {
    pub declared_properties: HashMap<Identifier, Expression>,
}
