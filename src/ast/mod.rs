pub type IdentifierName = String;

pub type Block = Vec<Statement>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program(pub Block);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assertion {
        condition: Expression,
    },
    Block(Block),
    Expression(Expression),
    If {
        condition: Expression,
        success_block: Block,
        else_block: Option<Block>,
    },
    VariableDeclaration {
        kind: VariableDeclKind,
        var_name: IdentifierName,
        initialiser: Option<Expression>,
    },
    WhileLoop {
        condition: Expression,
        block: Block,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    AssignmentOp {
        kind: AssignmentOp,
        lhs: MemberExpression,
        rhs: Box<Expression>,
    },
    BinaryOp {
        kind: BinaryOp,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    UnaryOp {
        kind: UnaryOp,
        operand: Box<Expression>,
    },

    Member(MemberExpression),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemberExpression {
    Identifier(IdentifierName),
    Literal(Literal),
    PropertyAccess {
        base: Box<Expression>,
        member_name: IdentifierName,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Null,
    Numeric(i64),
    String(String),
}

pub trait Operator {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> Precedence;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AssignmentOp {
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

impl Operator for AssignmentOp {
    fn associativity(&self) -> Associativity {
        match self {
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::PowAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::ShiftLeftAssign
            | Self::ShiftRightAssign
            | Self::ShiftRightUnsignedAssign
            | Self::BitwiseAndAssign
            | Self::BitwiseXOrAssign
            | Self::BitwiseOrAssign => Associativity::RightToLeft,
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::PowAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::ShiftLeftAssign
            | Self::ShiftRightAssign
            | Self::ShiftRightUnsignedAssign
            | Self::BitwiseAndAssign
            | Self::BitwiseXOrAssign
            | Self::BitwiseOrAssign => Precedence(3),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOp {
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

impl Operator for BinaryOp {
    fn associativity(&self) -> Associativity {
        match self {
            Self::Pow => Associativity::RightToLeft,
            Self::Mul | Self::Div | Self::Mod => Associativity::LeftToRight,
            Self::Add | Self::Sub => Associativity::LeftToRight,
            Self::ShiftLeft | Self::ShiftRight | Self::ShiftRightUnsigned => {
                Associativity::LeftToRight
            }
            Self::LessThan | Self::LessThanOrEqual | Self::MoreThan | Self::MoreThanOrEqual => {
                Associativity::LeftToRight
            }
            Self::Equal | Self::NotEqual | Self::Identical | Self::NotIdentical => {
                Associativity::LeftToRight
            }
            Self::BitwiseAnd => Associativity::LeftToRight,
            Self::BitwiseXOr => Associativity::LeftToRight,
            Self::BitwiseOr => Associativity::LeftToRight,
            Self::LogicalAnd => Associativity::LeftToRight,
            Self::LogicalOr => Associativity::LeftToRight,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnaryOp {
    DecrementPrefix,
    DecrementPostfix,
    IncrementPrefix,
    IncrementPostfix,

    BitwiseNot,
    LogicalNot,
    NumericNegate,
    NumericPlus,
}

impl Operator for UnaryOp {
    fn associativity(&self) -> Associativity {
        match self {
            Self::IncrementPostfix
            | Self::DecrementPostfix
            | Self::LogicalNot
            | Self::BitwiseNot
            | Self::NumericPlus
            | Self::NumericNegate
            | Self::IncrementPrefix
            | Self::DecrementPrefix => Associativity::RightToLeft,
        }
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VariableDeclKind {
    Const,
    Let,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Precedence(u8);

impl Precedence {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(21);
}
