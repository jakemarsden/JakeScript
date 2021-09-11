use std::fmt;

pub type IdentifierName = String;
pub type IdentifierNameRef = str;

pub trait Node: Clone + fmt::Debug {}

#[derive(Clone, Default, Debug)]
pub struct Program {
    body: Block,
    constants: Vec<(ConstantId, ConstantValue)>,
}

impl Program {
    pub fn new(body: Block, constants: Vec<(ConstantId, ConstantValue)>) -> Self {
        Self { body, constants }
    }

    pub fn body(&self) -> &Block {
        &self.body
    }

    pub fn constants(&self) -> &[(ConstantId, ConstantValue)] {
        &self.constants
    }
}

impl Node for Program {}

#[derive(Clone, Default, Debug)]
pub struct Block(Vec<Statement>);

impl Block {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Self(stmts)
    }

    pub fn statements(&self) -> &[Statement] {
        &self.0
    }
}

impl Node for Block {}

#[derive(Clone, Debug)]
pub enum Statement {
    Assertion(Assertion),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
    IfStatement(IfStatement),
    Print(PrintStatement),
    Return(ReturnStatement),
    VariableDeclaration(VariableDeclaration),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
}

impl Node for Statement {}

#[derive(Clone, Debug)]
pub struct Assertion {
    pub condition: Expression,
}

impl Node for Assertion {}

#[derive(Clone, Debug)]
pub struct PrintStatement {
    pub argument: Expression,
    pub new_line: bool,
}

impl Node for PrintStatement {}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub success_block: Block,
    pub else_block: Option<Block>,
}

impl Node for IfStatement {}

#[derive(Clone, Debug)]
pub struct ForLoop {
    pub initialiser: Option<VariableDeclaration>,
    pub condition: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub block: Block,
}

impl Node for ForLoop {}

#[derive(Clone, Debug)]
pub struct WhileLoop {
    pub condition: Expression,
    pub block: Block,
}

impl Node for WhileLoop {}

#[derive(Clone, Debug)]
pub struct BreakStatement {
    // TODO: Support labels.
}

impl Node for BreakStatement {}

#[derive(Clone, Debug)]
pub struct ContinueStatement {
    // TODO: Support labels.
}

impl Node for ContinueStatement {}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub expr: Option<Expression>,
}

impl Node for ReturnStatement {}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub fn_name: ConstantId,
    pub param_names: Vec<ConstantId>,
    pub body: Block,
}

impl Node for FunctionDeclaration {}

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub var_name: ConstantId,
    pub initialiser: Option<Expression>,
}

impl Node for VariableDeclaration {}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct AssignmentExpression {
    pub kind: AssignmentOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl Node for AssignmentExpression {}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub kind: BinaryOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl Node for BinaryExpression {}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub kind: UnaryOp,
    pub operand: Box<Expression>,
}

impl Node for UnaryExpression {}

#[derive(Clone, Debug)]
pub struct GroupingExpression {
    pub inner: Box<Expression>,
}

impl Node for GroupingExpression {}

#[derive(Clone, Debug)]
pub struct LiteralExpression {
    pub value: Literal,
}

impl Node for LiteralExpression {}

#[derive(Clone, Debug)]
pub struct FunctionCallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl Node for FunctionCallExpression {}

#[derive(Clone, Debug)]
pub struct PropertyAccessExpression {
    pub base: Box<Expression>,
    pub property_name: ConstantId,
}

impl Node for PropertyAccessExpression {}

#[derive(Clone, Debug)]
pub struct VariableAccessExpression {
    pub var_name: ConstantId,
}

impl Node for VariableAccessExpression {}

#[derive(Clone, Default, Debug)]
pub enum Literal {
    Boolean(bool),
    Numeric(i64),
    // TODO: Store string literals in the constant pool.
    String(String),
    // TODO: Support properties in object literals.
    Object,
    Null,
    #[default]
    Undefined,
}

pub type ConstantValue = String;
pub type ConstantValueRef = str;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConstantId(usize);

impl ConstantId {
    pub(crate) fn new(idx: usize) -> Self {
        Self(idx)
    }

    pub(crate) fn idx(&self) -> usize {
        self.0
    }
}

impl fmt::Display for ConstantId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Note: 6 includes the 2 chars for the "0x" prefix, so only 4 actual digits are displayed.
        write!(f, "{:#06x}", self.0)
    }
}

impl fmt::Debug for ConstantId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub trait Operator {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> Precedence;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Op {
    Assignment(AssignmentOp),
    Binary(BinaryOp),
    Unary(UnaryOp),
    Grouping,
    FunctionCall,
    PropertyAccess,
}

impl Operator for Op {
    fn associativity(&self) -> Associativity {
        match self {
            Op::Assignment(kind) => kind.associativity(),
            Op::Binary(kind) => kind.associativity(),
            Op::Unary(kind) => kind.associativity(),
            Op::Grouping => GroupingOp.associativity(),
            Op::FunctionCall => FunctionCallOp.associativity(),
            Op::PropertyAccess => PropertyAccessOp.associativity(),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Op::Assignment(kind) => kind.precedence(),
            Op::Binary(kind) => kind.precedence(),
            Op::Unary(kind) => kind.precedence(),
            Op::Grouping => GroupingOp.precedence(),
            Op::FunctionCall => FunctionCallOp.precedence(),
            Op::PropertyAccess => PropertyAccessOp.precedence(),
        }
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub enum AssignmentOp {
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

impl Operator for AssignmentOp {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(3)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GroupingOp;

impl Operator for GroupingOp {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(21)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FunctionCallOp;

impl Operator for FunctionCallOp {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PropertyAccessOp;

impl Operator for PropertyAccessOp {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VariableDeclarationKind {
    Const,
    Let,
    Var,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Precedence(u8);

impl Precedence {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(21);
}
