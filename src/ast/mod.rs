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
    Assertion {
        condition: Expression,
    },
    Block(Vec<BlockItem>),
    Expression(Expression),
    If {
        condition: Expression,
        success_block: Vec<BlockItem>,
        else_block: Option<Vec<BlockItem>>,
    },
    WhileLoop {
        condition: Expression,
        block: Vec<BlockItem>,
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
    Numeric(u64),
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
    Variable {
        kind: VariableDeclKind,
        var_name: IdentifierName,
        initialiser: Option<Expression>,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VariableDeclKind {
    Const,
    Let,
}
