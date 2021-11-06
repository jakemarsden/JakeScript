use crate::str::NonEmptyString;
use std::fmt;

pub trait Node: Clone + fmt::Debug {}

#[derive(Clone, Default, Debug)]
pub struct Program {
    body: Block,
}

impl Program {
    pub fn new(body: Block) -> Self {
        Self { body }
    }

    pub fn body(&self) -> &Block {
        &self.body
    }
}

impl Node for Program {}

#[derive(Clone, Default, Debug)]
pub struct Block {
    stmts: Vec<Statement>,
    hoisted_decls: Vec<DeclarationStatement>,
}

impl Block {
    pub fn new(stmts: Vec<Statement>, hoisted_decls: Vec<DeclarationStatement>) -> Self {
        Self {
            stmts,
            hoisted_decls,
        }
    }

    pub fn statements(&self) -> &[Statement] {
        &self.stmts
    }

    pub fn hoisted_declarations(&self) -> &[DeclarationStatement] {
        &self.hoisted_decls
    }
}

impl Node for Block {}

#[derive(Clone, Debug)]
pub enum Statement {
    Assert(AssertStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Declaration(DeclarationStatement),
    Exit(ExitStatement),
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

impl DeclarationStatement {
    pub fn is_hoisted(&self) -> bool {
        match self {
            Self::Function(..) => true,
            Self::Variable(decl) => decl.is_hoisted(),
        }
    }
}

impl Node for DeclarationStatement {}

#[derive(Clone, Debug)]
pub struct AssertStatement {
    pub condition: Expression,
}

impl Node for AssertStatement {}

#[derive(Clone, Debug)]
pub struct ExitStatement;

impl Node for ExitStatement {}

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
    pub fn_name: Identifier,
    pub param_names: Vec<Identifier>,
    pub body: Block,
}

impl Node for FunctionDeclaration {}

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub entries: Vec<VariableDeclarationEntry>,
}

impl VariableDeclaration {
    pub fn is_escalated(&self) -> bool {
        match self.kind {
            VariableDeclarationKind::Let | VariableDeclarationKind::Const => false,
            VariableDeclarationKind::Var => true,
        }
    }

    pub fn is_hoisted(&self) -> bool {
        match self.kind {
            VariableDeclarationKind::Let | VariableDeclarationKind::Const => false,
            VariableDeclarationKind::Var => true,
        }
    }

    /// Split the declaration into
    ///
    /// 1. a "main" [`VariableDeclaration`], sans initialisers, to declare each entry
    /// 2. a new, synthesised [`Expression`] to initialise each entry, for each entry which started
    /// with an initialiser.
    pub fn into_declaration_and_initialiser(mut self) -> (Self, Vec<Expression>) {
        let mut initialisers = Vec::with_capacity(self.entries.len());
        for entry in &mut self.entries {
            if let Some(initialiser) = entry.initialiser.take() {
                // Synthesise an assignment expression to initialise the variable
                initialisers.push(Expression::Assignment(AssignmentExpression {
                    kind: AssignmentOperator::Assign,
                    lhs: Box::new(Expression::VariableAccess(VariableAccessExpression {
                        var_name: entry.var_name.clone(),
                    })),
                    rhs: Box::new(initialiser),
                }));
            }
        }
        (self, initialisers)
    }
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
    pub property_name: Identifier,
}

impl Node for PropertyAccessExpression {}

#[derive(Clone, Debug)]
pub struct VariableAccessExpression {
    pub var_name: Identifier,
}

impl Node for VariableAccessExpression {}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Identifier(NonEmptyString);

impl From<NonEmptyString> for Identifier {
    fn from(s: NonEmptyString) -> Self {
        Self(s)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
        param_names: Vec<Identifier>,
        body: Block,
    },
    Null,
    #[default]
    Undefined,
}

pub trait Op: Copy + Eq + fmt::Debug {
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
            Self::Ternary => TernaryOperator.associativity(),
            Self::Grouping => GroupingOperator.associativity(),
            Self::FunctionCall => FunctionCallOperator.associativity(),
            Self::PropertyAccess => PropertyAccessOperator.associativity(),
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
pub struct TernaryOperator;

impl Op for TernaryOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(4)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GroupingOperator;

impl Op for GroupingOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(21)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FunctionCallOperator;

impl Op for FunctionCallOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PropertyAccessOperator;

impl Op for PropertyAccessOperator {
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
    pub var_name: Identifier,
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
