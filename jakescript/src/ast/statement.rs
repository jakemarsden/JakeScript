use super::block::Block;
use super::declaration::{Declaration, LexicalDeclaration, VariableDeclaration};
use super::expression::Expression;
use super::identifier::Identifier;
use super::Node;
use crate::impl_node;
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct EmptyStatement {
    pub loc: SourceLocation,
}

impl_node!(EmptyStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BlockStatement {
    pub block: Block,
}

impl Node for BlockStatement {
    fn source_location(&self) -> &SourceLocation {
        self.block.source_location()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeclarationStatement {
    pub declaration: Declaration,
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn source_location(&self) -> &SourceLocation {
        self.expression.source_location()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IfStatement {
    pub loc: SourceLocation,
    pub condition: Expression,
    pub body: Box<Statement>,
    pub else_body: Option<Box<Statement>>,
}

impl_node!(IfStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SwitchStatement {
    pub loc: SourceLocation,
    pub value: Expression,
    pub cases: Vec<CaseStatement>,
    pub default_case: Option<DefaultCaseStatement>,
}

impl_node!(SwitchStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CaseStatement {
    pub loc: SourceLocation,
    pub expected: Expression,
    pub body: Vec<Statement>,
}

impl_node!(CaseStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DefaultCaseStatement {
    pub loc: SourceLocation,
    pub body: Vec<Statement>,
}

impl_node!(DefaultCaseStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DoStatement {
    pub loc: SourceLocation,
    pub body: Box<Statement>,
    pub condition: Expression,
}

impl_node!(DoStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WhileStatement {
    pub loc: SourceLocation,
    pub condition: Expression,
    pub body: Box<Statement>,
}

impl_node!(WhileStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ForStatement {
    pub loc: SourceLocation,
    pub initialiser: Option<LoopInitialiser>,
    pub condition: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub body: Box<Statement>,
}

impl_node!(ForStatement);

// TODO: a better way to factor this, if there is one. A for-loop's initialiser
// can either be a variable declaration or an expression (why?), so it's type
// can't be as simple as `VariableDeclaration`. It also can't be `Expression`
// because a declaration can't be made a type of expression, and it can't be
// `Statement` because only certain types of statement are allowed.
// - valid (can be a variable declaration): `for (var i = 0;;) {}`
// - valid (can be any expression): `for (1 + 1;;) {}`
// - valid (can be any expression): `for (console.log("hello world");;) {}`
// - invalid (can't be any statement): `for (if (true) {};;) {}`
// - invalid (a declaration can't be type of expression): `console.log(let foo =
//   "bar");`
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "initialiser_type")]
pub enum LoopInitialiser {
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    LexicalDeclaration(LexicalDeclaration),
}

impl Node for LoopInitialiser {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::Expression(node) => node.source_location(),
            Self::VariableDeclaration(node) => node.source_location(),
            Self::LexicalDeclaration(node) => node.source_location(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TryStatement {
    pub loc: SourceLocation,
    pub body: Block,
    pub catch: Option<CatchStatement>,
    pub finally: Option<FinallyStatement>,
}

impl_node!(TryStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CatchStatement {
    pub loc: SourceLocation,
    pub parameter: Option<Identifier>,
    pub body: Block,
}

impl_node!(CatchStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FinallyStatement {
    pub loc: SourceLocation,
    pub body: Block,
}

impl_node!(FinallyStatement);

// TODO: Support labels.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ContinueStatement {
    pub loc: SourceLocation,
}

impl_node!(ContinueStatement);

// TODO: Support labels.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct BreakStatement {
    pub loc: SourceLocation,
}

impl_node!(BreakStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReturnStatement {
    pub loc: SourceLocation,
    pub value: Option<Expression>,
}

impl_node!(ReturnStatement);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ThrowStatement {
    pub loc: SourceLocation,
    pub exception: Expression,
}

impl_node!(ThrowStatement);
