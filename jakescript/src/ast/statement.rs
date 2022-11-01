use super::block::Block;
use super::declaration::{Declaration, LexicalDeclaration, VariableDeclaration};
use super::expression::Expression;
use super::identifier::Identifier;
use crate::ast_node;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

ast_node!(
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
);

ast_node!(
    #[derive(Eq)]
    pub struct EmptyStatement {
        pub loc: SourceLocation,
    }
);

ast_node!(
    ##[source_location = |self| self.block.source_location()]
    pub struct BlockStatement {
        pub block: Block,
    }
);

ast_node!(
    ##[source_location = |self| self.declaration.source_location()]
    pub struct DeclarationStatement {
        pub declaration: Declaration,
    }
);

impl DeclarationStatement {
    pub fn is_hoisted(&self) -> bool {
        self.declaration.is_hoisted()
    }

    pub fn into_declaration_and_initialiser(self) -> (Declaration, Vec<Expression>) {
        self.declaration.into_declaration_and_initialiser()
    }
}

ast_node!(
    ##[source_location = |self| self.expression.source_location()]
    pub struct ExpressionStatement {
        pub expression: Expression,
    }
);

ast_node!(
    pub struct IfStatement {
        pub loc: SourceLocation,
        pub condition: Expression,
        pub body: Box<Statement>,
        pub else_body: Option<Box<Statement>>,
    }
);

ast_node!(
    pub struct SwitchStatement {
        pub loc: SourceLocation,
        pub value: Expression,
        pub cases: Vec<CaseStatement>,
        pub default_case: Option<DefaultCaseStatement>,
    }
);

ast_node!(
    pub struct CaseStatement {
        pub loc: SourceLocation,
        pub expected: Expression,
        pub body: Vec<Statement>,
    }
);

ast_node!(
    pub struct DefaultCaseStatement {
        pub loc: SourceLocation,
        pub body: Vec<Statement>,
    }
);

ast_node!(
    pub struct DoStatement {
        pub loc: SourceLocation,
        pub body: Box<Statement>,
        pub condition: Expression,
    }
);

ast_node!(
    pub struct WhileStatement {
        pub loc: SourceLocation,
        pub condition: Expression,
        pub body: Box<Statement>,
    }
);

ast_node!(
    pub struct ForStatement {
        pub loc: SourceLocation,
        pub initialiser: Option<LoopInitialiser>,
        pub condition: Option<Expression>,
        pub incrementor: Option<Expression>,
        pub body: Box<Statement>,
    }
);

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
ast_node!(
    #[serde(tag = "initialiser_type")]
    pub enum LoopInitialiser {
        Expression(Expression),
        VariableDeclaration(VariableDeclaration),
        LexicalDeclaration(LexicalDeclaration),
    }
);

ast_node!(
    pub struct TryStatement {
        pub loc: SourceLocation,
        pub body: Block,
        pub catch: Option<CatchStatement>,
        pub finally: Option<FinallyStatement>,
    }
);

ast_node!(
    pub struct CatchStatement {
        pub loc: SourceLocation,
        pub parameter: Option<Identifier>,
        pub body: Block,
    }
);

ast_node!(
    pub struct FinallyStatement {
        pub loc: SourceLocation,
        pub body: Block,
    }
);

// TODO: Support labels.
ast_node!(
    #[derive(Eq)]
    pub struct ContinueStatement {
        pub loc: SourceLocation,
    }
);

// TODO: Support labels.
ast_node!(
    #[derive(Eq)]
    pub struct BreakStatement {
        pub loc: SourceLocation,
    }
);

ast_node!(
    pub struct ReturnStatement {
        pub loc: SourceLocation,
        pub value: Option<Expression>,
    }
);

ast_node!(
    pub struct ThrowStatement {
        pub loc: SourceLocation,
        pub exception: Expression,
    }
);
