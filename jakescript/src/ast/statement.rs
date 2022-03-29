use super::block::Block;
use super::declaration::VariableDeclaration;
use super::expression::Expression;
use super::identifier::Identifier;
use super::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "statement_type")]
pub enum Statement {
    Break(BreakStatement),
    Continue(ContinueStatement),
    Expression(Expression),
    If(IfStatement),
    Return(ReturnStatement),
    Throw(ThrowStatement),
    Try(TryStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub success_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForLoop {
    pub initialiser: Option<VariableDeclaration>,
    pub condition: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub body: Block,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BreakStatement {
    // TODO: Support labels.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContinueStatement {
    // TODO: Support labels.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReturnStatement {
    pub expr: Option<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThrowStatement {
    pub exception: Expression,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TryStatement {
    pub body: Block,
    pub catch_block: Option<CatchBlock>,
    pub finally_block: Option<FinallyBlock>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatchBlock {
    pub exception_identifier: Option<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FinallyBlock {
    pub inner: Block,
}

impl Node for Statement {}

impl Node for IfStatement {}

impl Node for ForLoop {}

impl Node for WhileLoop {}

impl Node for BreakStatement {}

impl Node for ContinueStatement {}

impl Node for ReturnStatement {}

impl Node for ThrowStatement {}

impl Node for TryStatement {}

impl Node for CatchBlock {}

impl Node for FinallyBlock {}
