use super::identifier::Identifier;
use super::literal::{Literal, NumericLiteral, StringLiteral};
use super::Node;
use crate::ast::Block;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentifierReferenceExpression {
    pub identifier: Identifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiteralExpression {
    pub value: Literal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArrayExpression {
    pub declared_elements: Vec<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectExpression {
    pub declared_properties: Vec<DeclaredProperty>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DeclaredPropertyName {
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    Computed(Expression),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclaredProperty {
    pub name: DeclaredPropertyName,
    pub initialiser: Expression,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FunctionExpression {
    pub binding: Option<Identifier>,
    pub formal_parameters: Vec<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssignmentExpression {
    pub op: AssignmentOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinaryExpression {
    pub op: BinaryOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelationalExpression {
    pub op: RelationalOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnaryExpression {
    pub op: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateExpression {
    pub op: UpdateOperator,
    pub operand: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MemberExpression {
    FunctionCall(FunctionCallExpression),
    MemberAccess(MemberAccessExpression),
    ComputedMemberAccess(ComputedMemberAccessExpression),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FunctionCallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberAccessExpression {
    pub base: Box<Expression>,
    pub member: Identifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComputedMemberAccessExpression {
    pub base: Box<Expression>,
    pub member: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupingExpression {
    pub inner: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
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

impl Node for Expression {}

impl Node for IdentifierReferenceExpression {}

impl Node for LiteralExpression {}

impl Node for ArrayExpression {}

impl Node for ObjectExpression {}

impl Node for FunctionExpression {}

impl Node for AssignmentExpression {}

impl Node for BinaryExpression {}

impl Node for RelationalExpression {}

impl Node for UnaryExpression {}

impl Node for UpdateExpression {}

impl Node for MemberExpression {}

impl Node for FunctionCallExpression {}

impl Node for MemberAccessExpression {}

impl Node for ComputedMemberAccessExpression {}

impl Node for GroupingExpression {}

impl Node for TernaryExpression {}

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
