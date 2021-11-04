use std::fmt;

pub type IdentifierName = String;
pub type IdentifierNameRef = str;

pub trait Node: Clone + fmt::Debug {}

#[derive(Clone, Default, Debug)]
pub struct Program {
    body: Block,
    constants: ConstantPool,
}

impl Program {
    pub fn new(body: Block, constants: ConstantPool) -> Self {
        Self { body, constants }
    }

    pub fn body(&self) -> &Block {
        &self.body
    }

    pub fn constants(&self) -> &ConstantPool {
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
    Assert(AssertStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Declaration(DeclarationStatement),
    Expression(Expression),
    IfStatement(IfStatement),
    Print(PrintStatement),
    Return(ReturnStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
}

impl Node for Statement {}

#[derive(Clone, Debug)]
pub enum DeclarationStatement {
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

impl Node for DeclarationStatement {}

#[derive(Clone, Debug)]
pub struct AssertStatement {
    pub condition: Expression,
}

impl Node for AssertStatement {}

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
    pub body: Block,
}

impl Node for ForLoop {}

#[derive(Clone, Debug)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Block,
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
    pub entries: Vec<VariableDeclarationEntry>,
}

impl Node for VariableDeclaration {}

#[derive(Clone, Debug)]
pub enum Expression {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Ternary(TernaryExpression),
    Grouping(GroupingExpression),
    FunctionCall(FunctionCallExpression),
    PropertyAccess(PropertyAccessExpression),

    Literal(LiteralExpression),
    VariableAccess(VariableAccessExpression),
}

impl Node for Expression {}

#[derive(Clone, Debug)]
pub struct AssignmentExpression {
    pub kind: AssignmentOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl Node for AssignmentExpression {}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub kind: BinaryOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl Node for BinaryExpression {}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub kind: UnaryOperator,
    pub operand: Box<Expression>,
}

impl Node for UnaryExpression {}

#[derive(Clone, Debug)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl Node for TernaryExpression {}

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
    /// Numeric literal tokens are **always unsigned** (but can be made negative at runtime with the
    /// negation unary operator).
    Numeric(u64),
    // TODO: Store string literals in the constant pool.
    String(String),
    // TODO: Support properties in object literals.
    Object,
    AnonFunction {
        param_names: Vec<ConstantId>,
        body: Block,
    },
    Null,
    #[default]
    Undefined,
}

pub type ConstantValue = String;
pub type ConstantValueRef = str;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConstantId(usize);

impl ConstantId {
    fn new(idx: usize) -> Self {
        Self(idx)
    }

    fn idx(&self) -> usize {
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

#[derive(Clone, Default, Debug)]
pub struct ConstantPool {
    constants: Vec<ConstantValue>,
}

impl ConstantPool {
    pub fn lookup(&self, id: ConstantId) -> &ConstantValueRef {
        match self.constants.get(id.idx()) {
            Some(value) => value,
            None => panic!("Invalid constant ID: {}", id),
        }
    }

    pub fn allocate_if_absent(&mut self, value: ConstantValue) -> ConstantId {
        if let Some((idx, _existing)) = self
            .constants
            .iter()
            .enumerate()
            .find(|(_idx, existing)| existing.as_str() == value.as_str())
        {
            ConstantId::new(idx)
        } else {
            let idx = self.constants.len();
            self.constants.push(value);
            ConstantId::new(idx)
        }
    }
}

pub trait Op {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> Precedence;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Operator {
    Assignment(AssignmentOperator),
    Binary(BinaryOperator),
    Unary(UnaryOperator),
    Ternary,
    Grouping,
    FunctionCall,
    PropertyAccess,
}

impl Op for Operator {
    fn associativity(&self) -> Associativity {
        match self {
            Self::Assignment(kind) => kind.associativity(),
            Self::Binary(kind) => kind.associativity(),
            Self::Unary(kind) => kind.associativity(),
            Self::Ternary => TernaryOp.associativity(),
            Self::Grouping => GroupingOp.associativity(),
            Self::FunctionCall => FunctionCallOp.associativity(),
            Self::PropertyAccess => PropertyAccessOp.associativity(),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Assignment(kind) => kind.precedence(),
            Self::Binary(kind) => kind.precedence(),
            Self::Unary(kind) => kind.precedence(),
            Self::Ternary => TernaryOp.precedence(),
            Self::Grouping => GroupingOp.precedence(),
            Self::FunctionCall => FunctionCallOp.precedence(),
            Self::PropertyAccess => PropertyAccessOp.precedence(),
        }
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
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

impl Op for AssignmentOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(3)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TernaryOp;

impl Op for TernaryOp {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(4)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GroupingOp;

impl Op for GroupingOp {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(21)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FunctionCallOp;

impl Op for FunctionCallOp {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PropertyAccessOp;

impl Op for PropertyAccessOp {
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

#[derive(Clone, Debug)]
pub struct VariableDeclarationEntry {
    pub var_name: ConstantId,
    pub initialiser: Option<Expression>,
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
