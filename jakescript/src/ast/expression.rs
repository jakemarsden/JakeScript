use super::identifier::Identifier;
use super::literal::{Literal, NumericLiteral, StringLiteral};
use super::op::{
    AssignmentOperator, BinaryOperator, RelationalOperator, UnaryOperator, UpdateOperator,
};
use crate::ast::Block;
use crate::ast_node;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

ast_node!(
    #[serde(tag = "expression_type")]
    pub enum Expression {
        IdentifierReference(IdentifierReferenceExpression),
        This(ThisExpression),
        New(NewExpression),
        Literal(LiteralExpression),
        Array(ArrayExpression),
        Object(ObjectExpression),
        /// Boxed due to large size only.
        Function(Box<FunctionExpression>),

        Assignment(AssignmentExpression),
        Binary(BinaryExpression),
        Relational(RelationalExpression),
        Unary(UnaryExpression),
        Update(UpdateExpression),
        Member(MemberExpression),
        Grouping(GroupingExpression),
        Ternary(TernaryExpression),
    }
);

impl From<FunctionExpression> for Expression {
    fn from(inner: FunctionExpression) -> Self {
        Self::from(Box::new(inner))
    }
}

ast_node!(
    #[derive(Eq)]
    pub struct IdentifierReferenceExpression {
        pub loc: SourceLocation,
        pub identifier: Identifier,
    }
);

ast_node!(
    #[derive(Eq)]
    pub struct ThisExpression {
        pub loc: SourceLocation,
    }
);

ast_node!(
    pub struct NewExpression {
        pub loc: SourceLocation,
        pub type_name: Identifier,
        pub arguments: Vec<Expression>,
    }
);

ast_node!(
    pub struct LiteralExpression {
        pub loc: SourceLocation,
        pub value: Literal,
    }
);

ast_node!(
    pub struct ArrayExpression {
        pub loc: SourceLocation,
        pub declared_elements: Vec<Expression>,
    }
);

ast_node!(
    pub struct ObjectExpression {
        pub loc: SourceLocation,
        pub declared_properties: Vec<DeclaredProperty>,
    }
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum DeclaredPropertyName {
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    Computed(Expression),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeclaredProperty {
    pub name: DeclaredPropertyName,
    pub initialiser: Expression,
}

ast_node!(
    pub struct FunctionExpression {
        pub loc: SourceLocation,
        pub binding: Option<Identifier>,
        pub formal_parameters: Vec<Identifier>,
        pub body: Block,
    }
);

ast_node!(
    pub struct AssignmentExpression {
        pub loc: SourceLocation,
        pub op: AssignmentOperator,
        pub lhs: Box<Expression>,
        pub rhs: Box<Expression>,
    }
);

ast_node!(
    pub struct BinaryExpression {
        pub loc: SourceLocation,
        pub op: BinaryOperator,
        pub lhs: Box<Expression>,
        pub rhs: Box<Expression>,
    }
);

ast_node!(
    pub struct RelationalExpression {
        pub loc: SourceLocation,
        pub op: RelationalOperator,
        pub lhs: Box<Expression>,
        pub rhs: Box<Expression>,
    }
);

ast_node!(
    pub struct UnaryExpression {
        pub loc: SourceLocation,
        pub op: UnaryOperator,
        pub operand: Box<Expression>,
    }
);

ast_node!(
    pub struct UpdateExpression {
        pub loc: SourceLocation,
        pub op: UpdateOperator,
        pub operand: Box<Expression>,
    }
);

ast_node!(
    pub enum MemberExpression {
        MemberAccess(MemberAccessExpression),
        ComputedMemberAccess(ComputedMemberAccessExpression),
        FunctionCall(FunctionCallExpression),
    }
);

ast_node!(
    pub struct MemberAccessExpression {
        pub loc: SourceLocation,
        pub base: Box<Expression>,
        pub member: Identifier,
    }
);

ast_node!(
    pub struct ComputedMemberAccessExpression {
        pub loc: SourceLocation,
        pub base: Box<Expression>,
        pub member: Box<Expression>,
    }
);

ast_node!(
    pub struct FunctionCallExpression {
        pub loc: SourceLocation,
        pub function: Box<Expression>,
        pub arguments: Vec<Expression>,
    }
);

ast_node!(
    pub struct GroupingExpression {
        pub loc: SourceLocation,
        pub inner: Box<Expression>,
    }
);

ast_node!(
    pub struct TernaryExpression {
        pub loc: SourceLocation,
        pub condition: Box<Expression>,
        pub lhs: Box<Expression>,
        pub rhs: Box<Expression>,
    }
);
