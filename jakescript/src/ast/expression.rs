use super::identifier::Identifier;
use super::literal::{Literal, NumericLiteral, StringLiteral};
use super::Node;
use crate::ast::Block;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "expression_type")]
pub enum Expression {
    IdentifierReference(IdentifierReferenceExpression),
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IdentifierReferenceExpression {
    pub identifier: Identifier,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LiteralExpression {
    pub value: Literal,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ArrayExpression {
    pub declared_elements: Vec<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ObjectExpression {
    pub declared_properties: Vec<DeclaredProperty>,
    pub loc: SourceLocation,
}

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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FunctionExpression {
    pub binding: Option<Identifier>,
    pub formal_parameters: Vec<Identifier>,
    pub body: Block,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AssignmentExpression {
    pub op: AssignmentOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BinaryExpression {
    pub op: BinaryOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RelationalExpression {
    pub op: RelationalOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnaryExpression {
    pub op: UnaryOperator,
    pub operand: Box<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateExpression {
    pub op: UpdateOperator,
    pub operand: Box<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum MemberExpression {
    FunctionCall(FunctionCallExpression),
    MemberAccess(MemberAccessExpression),
    ComputedMemberAccess(ComputedMemberAccessExpression),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FunctionCallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MemberAccessExpression {
    pub base: Box<Expression>,
    pub member: Identifier,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ComputedMemberAccessExpression {
    pub base: Box<Expression>,
    pub member: Box<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GroupingExpression {
    pub inner: Box<Expression>,
    pub loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub loc: SourceLocation,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Assignment(AssignmentOperator),
    Binary(BinaryOperator),
    Relational(RelationalOperator),
    Unary(UnaryOperator),
    Update(UpdateOperator),
    Member(MemberOperator),
    Grouping,
    Ternary,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub enum AssignmentOperator {
    #[default]
    Assign,
    ComputeAssign(BinaryOperator),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Exponentiation,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXOr,
    LogicalAnd,
    LogicalOr,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseRightShiftUnsigned,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum RelationalOperator {
    Equality,
    Inequality,
    StrictEquality,
    StrictInequality,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum UnaryOperator {
    NumericPlus,
    NumericNegation,
    BitwiseNot,
    LogicalNot,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum UpdateOperator {
    GetAndIncrement,
    IncrementAndGet,
    GetAndDecrement,
    DecrementAndGet,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum MemberOperator {
    FunctionCall,
    MemberAccess,
    ComputedMemberAccess,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GroupingOperator;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TernaryOperator;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Precedence(u8);

impl Node for Expression {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::IdentifierReference(node) => node.source_location(),
            Self::Literal(node) => node.source_location(),
            Self::Array(node) => node.source_location(),
            Self::Object(node) => node.source_location(),
            Self::Function(node) => node.source_location(),
            Self::Assignment(node) => node.source_location(),
            Self::Binary(node) => node.source_location(),
            Self::Relational(node) => node.source_location(),
            Self::Unary(node) => node.source_location(),
            Self::Update(node) => node.source_location(),
            Self::Member(node) => node.source_location(),
            Self::Grouping(node) => node.source_location(),
            Self::Ternary(node) => node.source_location(),
        }
    }
}

impl Node for IdentifierReferenceExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for LiteralExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for ArrayExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for ObjectExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for FunctionExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for AssignmentExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for BinaryExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for RelationalExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for UnaryExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for UpdateExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for MemberExpression {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::FunctionCall(node) => node.source_location(),
            Self::MemberAccess(node) => node.source_location(),
            Self::ComputedMemberAccess(node) => node.source_location(),
        }
    }
}

impl Node for FunctionCallExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for MemberAccessExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for ComputedMemberAccessExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for GroupingExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for TernaryExpression {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Operator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::Assignment(kind) => kind.associativity(),
            Self::Binary(kind) => kind.associativity(),
            Self::Relational(kind) => kind.associativity(),
            Self::Unary(kind) => kind.associativity(),
            Self::Update(kind) => kind.associativity(),
            Self::Member(kind) => kind.associativity(),
            Self::Grouping => GroupingOperator::associativity(),
            Self::Ternary => TernaryOperator::associativity(),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::Assignment(kind) => kind.precedence(),
            Self::Binary(kind) => kind.precedence(),
            Self::Relational(kind) => kind.precedence(),
            Self::Unary(kind) => kind.precedence(),
            Self::Update(kind) => kind.precedence(),
            Self::Member(kind) => kind.precedence(),
            Self::Grouping => GroupingOperator::precedence(),
            Self::Ternary => TernaryOperator::precedence(),
        }
    }
}

impl AssignmentOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::Assign | Self::ComputeAssign(..) => Associativity::RightToLeft,
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::Assign | Self::ComputeAssign(..) => Precedence(3),
        }
    }
}

impl BinaryOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::Exponentiation => Associativity::RightToLeft,
            _ => Associativity::LeftToRight,
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::Exponentiation => Precedence(16),
            Self::Multiplication | Self::Division | Self::Modulus => Precedence(15),
            Self::Addition | Self::Subtraction => Precedence(14),
            Self::BitwiseLeftShift | Self::BitwiseRightShift | Self::BitwiseRightShiftUnsigned => {
                Precedence(13)
            }
            Self::BitwiseAnd => Precedence(10),
            Self::BitwiseXOr => Precedence(9),
            Self::BitwiseOr => Precedence(8),
            Self::LogicalAnd => Precedence(7),
            Self::LogicalOr => Precedence(6),
        }
    }
}

impl RelationalOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::Equality
            | Self::Inequality
            | Self::StrictEquality
            | Self::StrictInequality
            | Self::GreaterThan
            | Self::GreaterThanOrEqual
            | Self::LessThan
            | Self::LessThanOrEqual => Associativity::LeftToRight,
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::GreaterThan
            | Self::GreaterThanOrEqual
            | Self::LessThan
            | Self::LessThanOrEqual => Precedence(12),
            Self::Equality | Self::Inequality | Self::StrictEquality | Self::StrictInequality => {
                Precedence(11)
            }
        }
    }
}

impl UnaryOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::NumericPlus | Self::NumericNegation | Self::BitwiseNot | Self::LogicalNot => {
                Associativity::RightToLeft
            }
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::NumericPlus | Self::NumericNegation | Self::BitwiseNot | Self::LogicalNot => {
                Precedence(17)
            }
        }
    }
}

impl UpdateOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::GetAndIncrement
            | Self::IncrementAndGet
            | Self::GetAndDecrement
            | Self::DecrementAndGet => Associativity::RightToLeft,
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::GetAndIncrement | Self::GetAndDecrement => Precedence(18),
            Self::IncrementAndGet | Self::DecrementAndGet => Precedence(17),
        }
    }
}

impl MemberOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::FunctionCall | Self::MemberAccess | Self::ComputedMemberAccess => {
                Associativity::LeftToRight
            }
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::FunctionCall | Self::MemberAccess | Self::ComputedMemberAccess => Precedence(20),
        }
    }
}

impl GroupingOperator {
    pub fn associativity() -> Associativity {
        Associativity::LeftToRight
    }

    pub fn precedence() -> Precedence {
        Precedence(21)
    }
}

impl TernaryOperator {
    pub fn associativity() -> Associativity {
        Associativity::RightToLeft
    }

    pub fn precedence() -> Precedence {
        Precedence(4)
    }
}

impl Precedence {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(21);
}
