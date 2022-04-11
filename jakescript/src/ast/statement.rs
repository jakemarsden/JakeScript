use super::block::Block;
use super::declaration::VariableDeclaration;
use super::expression::Expression;
use super::identifier::Identifier;
use super::Node;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "statement_type")]
pub enum Statement {
    Expression(ExpressionStatement),

    If(IfStatement),
    WhileLoop(WhileStatement),
    DoWhileLoop(DoWhileStatement),
    ForLoop(ForStatement),

    Continue(ContinueStatement),
    Break(BreakStatement),
    Return(ReturnStatement),
    Throw(ThrowStatement),
    Try(TryStatement),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Block,
    pub else_body: Option<Block>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DoWhileStatement {
    pub body: Block,
    pub condition: Expression,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ForStatement {
    pub initialiser: Option<VariableDeclaration>,
    pub condition: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub body: Block,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ContinueStatement {
    // TODO: Support labels.
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BreakStatement {
    // TODO: Support labels.
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ThrowStatement {
    pub exception: Expression,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TryStatement {
    pub body: Block,
    pub catch: Option<Catch>,
    pub finally: Option<Finally>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Catch {
    pub parameter: Option<Identifier>,
    pub body: Block,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Finally {
    pub body: Block,
    pub loc: SourceLocation,
}

impl Node for Statement {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::Expression(node) => node.source_location(),
            Self::If(node) => node.source_location(),
            Self::WhileLoop(node) => node.source_location(),
            Self::DoWhileLoop(node) => node.source_location(),
            Self::ForLoop(node) => node.source_location(),
            Self::Continue(node) => node.source_location(),
            Self::Break(node) => node.source_location(),
            Self::Return(node) => node.source_location(),
            Self::Throw(node) => node.source_location(),
            Self::Try(node) => node.source_location(),
        }
    }
}

impl Node for ExpressionStatement {
    fn source_location(&self) -> &SourceLocation {
        self.expression.source_location()
    }
}

impl Node for IfStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for WhileStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for DoWhileStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for ForStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for ContinueStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for BreakStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for ReturnStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for ThrowStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for TryStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for Catch {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for Finally {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}
