use super::block::Block;
use super::declaration::{Declaration, VariableDeclaration};
use super::expression::Expression;
use super::identifier::Identifier;
use super::Node;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "statement_type")]
pub enum Statement {
    Empty(EmptyStatement),
    Block(BlockStatement),
    Declaration(DeclarationStatement),
    Expression(ExpressionStatement),

    If(IfStatement),
    Switch(SwitchStatement),
    Do(DoStatement),
    While(WhileStatement),
    For(ForStatement),
    Try(TryStatement),

    Continue(ContinueStatement),
    Break(BreakStatement),
    Return(ReturnStatement),
    Throw(ThrowStatement),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmptyStatement {
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BlockStatement {
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeclarationStatement {
    pub declaration: Declaration,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IfStatement {
    pub loc: SourceLocation,
    pub condition: Expression,
    pub body: Box<Statement>,
    pub else_body: Option<Box<Statement>>,
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
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DefaultCaseStatement {
    pub loc: SourceLocation,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DoStatement {
    pub loc: SourceLocation,
    pub body: Box<Statement>,
    pub condition: Expression,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WhileStatement {
    pub loc: SourceLocation,
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ForStatement {
    pub loc: SourceLocation,
    pub initialiser: Option<LoopInitialiser>,
    pub condition: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub body: Box<Statement>,
}

// TODO: a better way to factor this, if there is one. A for-loop's initialiser can either be a
//  variable declaration or an expression (why?), so it's type can't be as simple as
//  `VariableDeclaration`. It also can't be `Expression` because a declaration can't be made a type
//  of expression, and it can't be `Statement` because only certain types of statement are allowed.
//  - valid:   for (var i = 0;;) {}                  // can be a variable declaration
//  - valid:   for (1 + 1;;) {}                      // can be any expression
//  - valid:   for (console.log("hello world");;) {} // can be any expression
//  - invalid: for (if (true) {};;) {}               // can't be any statement
//  - invalid: console.log(let foo = "bar");         // a declaration can't be a type of expression
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "initialiser_type")]
pub enum LoopInitialiser {
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
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

impl Node for Statement {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::Empty(node) => node.source_location(),
            Self::Block(node) => node.source_location(),
            Self::Declaration(node) => node.source_location(),
            Self::Expression(node) => node.source_location(),

            Self::If(node) => node.source_location(),
            Self::Switch(node) => node.source_location(),
            Self::Do(node) => node.source_location(),
            Self::While(node) => node.source_location(),
            Self::For(node) => node.source_location(),
            Self::Try(node) => node.source_location(),

            Self::Continue(node) => node.source_location(),
            Self::Break(node) => node.source_location(),
            Self::Return(node) => node.source_location(),
            Self::Throw(node) => node.source_location(),
        }
    }
}

impl Node for EmptyStatement {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for BlockStatement {
    fn source_location(&self) -> &SourceLocation {
        self.block.source_location()
    }
}

impl DeclarationStatement {
    pub fn is_hoisted(&self) -> bool {
        self.declaration.is_hoisted()
    }

    pub fn into_declaration_and_initialiser(self) -> (Declaration, Vec<Expression>) {
        self.declaration.into_declaration_and_initialiser()
    }
}

impl Node for DeclarationStatement {
    fn source_location(&self) -> &SourceLocation {
        self.declaration.source_location()
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

impl Node for LoopInitialiser {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::Expression(node) => node.source_location(),
            Self::VariableDeclaration(node) => node.source_location(),
        }
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
