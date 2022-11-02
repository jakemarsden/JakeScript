use serde::{Deserialize, Serialize};

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum MemberOperator {
    MemberAccess,
    ComputedMemberAccess,
    FunctionCall,
}

impl MemberOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::MemberAccess | Self::ComputedMemberAccess | Self::FunctionCall => {
                Associativity::LeftToRight
            }
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::MemberAccess | Self::ComputedMemberAccess | Self::FunctionCall => Precedence(20),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GroupingOperator;

impl GroupingOperator {
    pub fn associativity() -> Associativity {
        Associativity::LeftToRight
    }

    pub fn precedence() -> Precedence {
        Precedence(21)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TernaryOperator;

impl TernaryOperator {
    pub fn associativity() -> Associativity {
        Associativity::RightToLeft
    }

    pub fn precedence() -> Precedence {
        Precedence(4)
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