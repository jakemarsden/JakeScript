pub type IdentifierName = String;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program(pub Vec<BlockItem>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlockItem {
    Statement(Statement),
    Declaration(Declaration),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Block(Vec<BlockItem>),
    Expression(Expression),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    /// (op_kind, lhs, rhs)
    AssignmentOp(AssignmentOp, MemberExpression, Box<Expression>),
    /// (op_kind, lhs, rhs)
    BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
    /// (op_kind, operand)
    UnaryOp(UnaryOp, Box<Expression>),

    Member(MemberExpression),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemberExpression {
    /// (name)
    Identifier(IdentifierName),
    Literal(Literal),
    /// (base, property_name)
    PropertyAccess(Box<Expression>, IdentifierName),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    /// (value)
    Boolean(bool),
    Null,
    /// (value)
    Numeric(u64),
    /// (value)
    String(String),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Declaration {
    /// (decl_kind, var_name, initialiser)
    Variable(VariableDeclKind, IdentifierName, Option<Expression>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VariableDeclKind {
    Const,
    Let,
}
