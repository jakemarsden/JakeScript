use super::identifier::Identifier;
use super::literal::Literal;
use super::Node;
use serde::{Deserialize, Serialize};
use std::fmt;

pub trait Op: Copy + Eq + fmt::Debug {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> Precedence;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "expression_type")]
pub enum Expression {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Ternary(TernaryExpression),
    Grouping(GroupingExpression),
    FunctionCall(FunctionCallExpression),
    PropertyAccess(PropertyAccessExpression),
    ComputedPropertyAccess(ComputedPropertyAccessExpression),

    Literal(LiteralExpression),
    VariableAccess(VariableAccessExpression),
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
pub struct UnaryExpression {
    pub op: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupingExpression {
    pub inner: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiteralExpression {
    pub value: Literal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FunctionCallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PropertyAccessExpression {
    pub base: Box<Expression>,
    pub property_name: Identifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComputedPropertyAccessExpression {
    pub base: Box<Expression>,
    pub property: Box<Expression>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VariableAccessExpression {
    pub var_name: Identifier,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Assignment(AssignmentOperator),
    Binary(BinaryOperator),
    Unary(UnaryOperator),
    Ternary,
    Grouping,
    FunctionCall,
    PropertyAccess,
    ComputedPropertyAccess,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub enum AssignmentOperator {
    #[default]
    Assign,
    AddAssign,
    DivAssign,
    ModAssign,
    MulAssign,
    PowAssign,
    SubAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
    ShiftRightUnsignedAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXOrAssign,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BinaryOperator {
    Add,
    Div,
    Mod,
    Mul,
    Pow,
    Sub,
    Equal,
    NotEqual,
    Identical,
    NotIdentical,
    LessThan,
    LessThanOrEqual,
    MoreThan,
    MoreThanOrEqual,
    ShiftLeft,
    ShiftRight,
    ShiftRightUnsigned,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXOr,
    LogicalAnd,
    LogicalOr,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum UnaryOperator {
    DecrementPrefix,
    DecrementPostfix,
    IncrementPrefix,
    IncrementPostfix,
    BitwiseNot,
    LogicalNot,
    NumericNegate,
    NumericPlus,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TernaryOperator;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GroupingOperator;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FunctionCallOperator;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PropertyAccessOperator;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ComputedPropertyAccessOperator;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Precedence(u8);

impl Node for Expression {}

impl Node for AssignmentExpression {}

impl Node for BinaryExpression {}

impl Node for UnaryExpression {}

impl Node for TernaryExpression {}

impl Node for GroupingExpression {}

impl Node for LiteralExpression {}

impl Node for FunctionCallExpression {}

impl Node for PropertyAccessExpression {}

impl Node for ComputedPropertyAccessExpression {}

impl Node for VariableAccessExpression {}

impl Op for Operator {
    fn associativity(&self) -> Associativity {
        match self {
            Self::Assignment(kind) => kind.associativity(),
            Self::Binary(kind) => kind.associativity(),
            Self::Unary(kind) => kind.associativity(),
            Self::Ternary => TernaryOperator.associativity(),
            Self::Grouping => GroupingOperator.associativity(),
            Self::FunctionCall => FunctionCallOperator.associativity(),
            Self::PropertyAccess => PropertyAccessOperator.associativity(),
            Self::ComputedPropertyAccess => ComputedPropertyAccessOperator.associativity(),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Assignment(kind) => kind.precedence(),
            Self::Binary(kind) => kind.precedence(),
            Self::Unary(kind) => kind.precedence(),
            Self::Ternary => TernaryOperator.precedence(),
            Self::Grouping => GroupingOperator.precedence(),
            Self::FunctionCall => FunctionCallOperator.precedence(),
            Self::PropertyAccess => PropertyAccessOperator.precedence(),
            Self::ComputedPropertyAccess => ComputedPropertyAccessOperator.precedence(),
        }
    }
}

impl Op for AssignmentOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(3)
    }
}

impl Op for BinaryOperator {
    fn associativity(&self) -> Associativity {
        match self {
            Self::Pow => Associativity::RightToLeft,
            _ => Associativity::LeftToRight,
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Pow => Precedence(16),
            Self::Mul | Self::Div | Self::Mod => Precedence(15),
            Self::Add | Self::Sub => Precedence(14),
            Self::ShiftLeft | Self::ShiftRight | Self::ShiftRightUnsigned => Precedence(13),
            Self::LessThan | Self::LessThanOrEqual | Self::MoreThan | Self::MoreThanOrEqual => {
                Precedence(12)
            }
            Self::Equal | Self::NotEqual | Self::Identical | Self::NotIdentical => Precedence(11),
            Self::BitwiseAnd => Precedence(10),
            Self::BitwiseXOr => Precedence(9),
            Self::BitwiseOr => Precedence(8),
            Self::LogicalAnd => Precedence(7),
            Self::LogicalOr => Precedence(6),
        }
    }
}

impl Op for UnaryOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::IncrementPostfix | Self::DecrementPostfix => Precedence(18),
            Self::LogicalNot
            | Self::BitwiseNot
            | Self::NumericPlus
            | Self::NumericNegate
            | Self::IncrementPrefix
            | Self::DecrementPrefix => Precedence(17),
        }
    }
}

impl Op for TernaryOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(4)
    }
}

impl Op for GroupingOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(21)
    }
}

impl Op for FunctionCallOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

impl Op for PropertyAccessOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

impl Op for ComputedPropertyAccessOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

impl Precedence {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(21);
}
