use super::block::Block;
use super::declaration::VariableDeclaration;
use super::expression::Expression;
use super::identifier::Identifier;
use super::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "statement_type")]
pub enum Statement {
    Expression(ExpressionStatement),

    If(IfStatement),
    WhileLoop(WhileStatement),
    ForLoop(ForStatement),

    Continue(ContinueStatement),
    Break(BreakStatement),
    Return(ReturnStatement),
    Throw(ThrowStatement),
    Try(TryStatement),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Block,
    pub else_body: Option<Block>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForStatement {
    pub initialiser: Option<VariableDeclaration>,
    pub condition: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub body: Block,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContinueStatement {
    // TODO: Support labels.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BreakStatement {
    // TODO: Support labels.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThrowStatement {
    pub exception: Expression,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TryStatement {
    pub body: Block,
    pub catch: Option<Catch>,
    pub finally: Option<Finally>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Catch {
    pub parameter: Option<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Finally {
    pub body: Block,
}

impl Node for Statement {}

impl Node for ExpressionStatement {}

impl Node for IfStatement {}

impl Node for WhileStatement {}

impl Node for ForStatement {}

impl Node for ContinueStatement {}

impl Node for BreakStatement {}

impl Node for ReturnStatement {}

impl Node for ThrowStatement {}

impl Node for TryStatement {}

impl Node for Catch {}

impl Node for Finally {}
