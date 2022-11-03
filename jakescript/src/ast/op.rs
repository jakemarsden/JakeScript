use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Member(MemberOperator),

    Assignment(AssignmentOperator),
    Binary(BinaryOperator),
    Grouping,
    Relational(RelationalOperator),
    Ternary,
    Unary(UnaryOperator),
    Update(UpdateOperator),
}

impl Operator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::Member(kind) => kind.associativity(),

            Self::Assignment(kind) => kind.associativity(),
            Self::Binary(kind) => kind.associativity(),
            Self::Grouping => Associativity::LeftToRight,
            Self::Relational(kind) => kind.associativity(),
            Self::Ternary => Associativity::RightToLeft,
            Self::Unary(kind) => kind.associativity(),
            Self::Update(kind) => kind.associativity(),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::Member(kind) => kind.precedence(),

            Self::Assignment(kind) => kind.precedence(),
            Self::Binary(kind) => kind.precedence(),
            Self::Grouping => Precedence(21),
            Self::Relational(kind) => kind.precedence(),
            Self::Ternary => Precedence(4),
            Self::Unary(kind) => kind.precedence(),
            Self::Update(kind) => kind.precedence(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum MemberOperator {
    ComputedMemberAccess,
    FunctionCall,
    MemberAccess,
}

impl MemberOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::ComputedMemberAccess | Self::FunctionCall | Self::MemberAccess => {
                Associativity::LeftToRight
            }
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::ComputedMemberAccess | Self::FunctionCall | Self::MemberAccess => Precedence(20),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub enum AssignmentOperator {
    #[default]
    Assign,
    ComputeAssign(BinaryOperator),
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum UnaryOperator {
    NumericPlus,
    NumericNegation,
    BitwiseNot,
    LogicalNot,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum UpdateOperator {
    GetAndIncrement,
    IncrementAndGet,
    GetAndDecrement,
    DecrementAndGet,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Precedence(u8);

impl Precedence {
    pub const MAX: Self = Self(21);
    pub const MIN: Self = Self(1);
}
