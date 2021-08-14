use std::{fmt, iter};

pub type IdentifierName = String;

pub trait Node: Clone + fmt::Debug + PartialEq {}

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

impl Node for Program {}

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

impl Node for Block {}

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
    Break(BreakStatement),
    Continue(ContinueStatement),
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
    IfStatement(IfStatement),
    Return(ReturnStatement),
    VariableDeclaration(VariableDeclaration),
    WhileLoop(WhileLoop),
}

impl Node for Statement {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Assertion {
    pub condition: Expression,
}

impl Node for Assertion {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub success_block: Block,
    pub else_block: Option<Block>,
}

impl Node for IfStatement {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhileLoop {
    pub condition: Expression,
    pub block: Block,
}

impl Node for WhileLoop {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreakStatement {
    // TODO: label
}

impl Node for BreakStatement {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinueStatement {
    // TODO: label
}

impl Node for ContinueStatement {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReturnStatement {
    pub expr: Option<Expression>,
}

impl Node for ReturnStatement {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionDeclaration {
    pub fn_name: IdentifierName,
    pub param_names: Vec<IdentifierName>,
    pub body: Block,
}

impl Node for FunctionDeclaration {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub var_name: IdentifierName,
    pub initialiser: Option<Expression>,
}

impl Node for VariableDeclaration {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Grouping(GroupingExpression),
    Literal(LiteralExpression),
    FunctionCall(FunctionCallExpression),
    PropertyAccess(PropertyAccessExpression),
    VariableAccess(VariableAccessExpression),
}

impl Node for Expression {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssignmentExpression {
    pub kind: AssignmentOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl Node for AssignmentExpression {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryExpression {
    pub kind: BinaryOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl Node for BinaryExpression {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnaryExpression {
    pub kind: UnaryOp,
    pub operand: Box<Expression>,
}

impl Node for UnaryExpression {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupingExpression {
    pub inner: Box<Expression>,
}

impl Node for GroupingExpression {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralExpression {
    pub value: Value,
}

impl Node for LiteralExpression {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionCallExpression {
    pub fn_name: IdentifierName,
    pub arguments: Vec<Expression>,
}

impl Node for FunctionCallExpression {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertyAccessExpression {
    pub base: Box<Expression>,
    pub property_name: IdentifierName,
}

impl Node for PropertyAccessExpression {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariableAccessExpression {
    pub var_name: IdentifierName,
}

impl Node for VariableAccessExpression {}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub enum Value {
    Boolean(bool),
    Numeric(i64),
    String(String),
    Null,
    #[default]
    Undefined,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(ref value) => *value,
            Value::Numeric(ref value) => *value > 0,
            Value::String(ref value) => !value.is_empty(),
            Value::Null | Value::Undefined => false,
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Numeric(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, r#""{}""#, value),
            Self::Null => f.write_str("null"),
            Self::Undefined => f.write_str("undefined"),
        }
    }
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
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(3)
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
