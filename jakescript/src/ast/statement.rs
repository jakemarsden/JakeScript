use super::block::{Block, BlockItem};
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
    Switch(SwitchStatement),
    Do(DoStatement),
    While(WhileStatement),
    For(ForStatement),

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
    pub loc: SourceLocation,
    pub condition: Expression,
    pub body: Block,
    pub else_body: Option<Block>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SwitchStatement {
    pub loc: SourceLocation,
    pub value: Expression,
    pub cases: Vec<CaseStatement>,
    pub default_case: Option<DefaultCaseStatement>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CaseStatement {
    pub loc: SourceLocation,
    pub expected: Expression,
    pub body: Vec<BlockItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DefaultCaseStatement {
    pub loc: SourceLocation,
    pub body: Vec<BlockItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DoStatement {
    pub loc: SourceLocation,
    pub body: Block,
    pub condition: Expression,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WhileStatement {
    pub loc: SourceLocation,
    pub condition: Expression,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ForStatement {
    pub loc: SourceLocation,
    pub initialiser: Option<VariableDeclaration>,
    pub condition: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub body: Block,
}

// TODO: Support labels.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ContinueStatement {
    pub loc: SourceLocation,
}

// TODO: Support labels.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BreakStatement {
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReturnStatement {
    pub loc: SourceLocation,
    pub value: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ThrowStatement {
    pub loc: SourceLocation,
    pub exception: Expression,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TryStatement {
    pub loc: SourceLocation,
    pub body: Block,
    pub catch: Option<CatchStatement>,
    pub finally: Option<FinallyStatement>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CatchStatement {
    pub loc: SourceLocation,
    pub parameter: Option<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FinallyStatement {
    pub loc: SourceLocation,
    pub body: Block,
}

impl Node for Statement {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::Expression(node) => node.source_location(),

            Self::If(node) => node.source_location(),
            Self::Switch(node) => node.source_location(),
            Self::Do(node) => node.source_location(),
            Self::While(node) => node.source_location(),
            Self::For(node) => node.source_location(),

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

impl Node for SwitchStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for CaseStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for DefaultCaseStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for DoStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for WhileStatement {
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

impl Node for CatchStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for FinallyStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}
