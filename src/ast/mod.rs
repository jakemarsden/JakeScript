use std::iter;

pub type IdentifierName = String;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program(Block);

impl Program {
    pub fn new(block: Block) -> Self {
        Self(block)
    }

    pub fn block(&self) -> &Block {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block(Vec<Statement>);

impl Block {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Self(stmts)
    }

    pub fn statements(&self) -> &[Statement] {
        &self.0
    }

    pub fn iter(&self) -> impl iter::Iterator<Item = &Statement> {
        self.statements().iter()
    }
}

impl iter::IntoIterator for Block {
    type Item = Statement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl iter::FromIterator<Statement> for Block {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Statement>,
    {
        let stmts: Vec<_> = iter.into_iter().collect();
        Self::new(stmts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assertion(Assertion),
    Block(Block),
    Expression(Expression),
    IfStatement(IfStatement),
    VariableDeclaration(VariableDeclaration),
    WhileLoop(WhileLoop),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Assertion {
    pub condition: Expression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub success_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhileLoop {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub var_name: IdentifierName,
    pub initialiser: Option<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),

    Literal(Literal),
    PropertyAccess {
        base: Box<Expression>,
        member_name: IdentifierName,
    },
    VariableAccess(IdentifierName),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssignmentExpression {
    pub kind: AssignmentOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryExpression {
    pub kind: BinaryOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnaryExpression {
    pub kind: UnaryOp,
    pub operand: Box<Expression>,
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
pub enum VariableDeclarationKind {
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
