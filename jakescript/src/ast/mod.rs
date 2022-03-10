use crate::str::NonEmptyString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub trait Node: Clone + fmt::Debug {}

pub trait Op: Copy + Eq + fmt::Debug {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> Precedence;
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Program {
    body: Block,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Block {
    hoisted_decls: Vec<DeclarationStatement>,
    stmts: Vec<Statement>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "statement_type")]
pub enum Statement {
    Break(BreakStatement),
    Continue(ContinueStatement),
    Declaration(DeclarationStatement),
    Expression(Expression),
    If(IfStatement),
    Return(ReturnStatement),
    Throw(ThrowStatement),
    Try(TryStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "declaration_type")]
pub enum DeclarationStatement {
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub success_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForLoop {
    pub initialiser: Option<VariableDeclaration>,
    pub condition: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub body: Block,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BreakStatement {
    // TODO: Support labels.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContinueStatement {
    // TODO: Support labels.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub expr: Option<Expression>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThrowStatement {
    pub exception: Expression,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TryStatement {
    pub body: Block,
    pub catch_block: Option<CatchBlock>,
    pub finally_block: Option<FinallyBlock>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatchBlock {
    pub exception_identifier: Option<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinallyBlock {
    pub inner: Block,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub fn_name: Identifier,
    pub param_names: Vec<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub entries: Vec<VariableDeclarationEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "expression_type")]
pub enum Expression {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Ternary(TernaryExpression),
    Grouping(GroupingExpression),
    FunctionCall(FunctionCallExpression),
    PropertyAccess(PropertyAccessExpression),
    ComputedPropertyAccess(ComputedPropertyAccessExpression),

    Literal(LiteralExpression),
    VariableAccess(VariableAccessExpression),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub op: AssignmentOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub op: BinaryOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub op: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupingExpression {
    pub inner: Box<Expression>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiteralExpression {
    pub value: Literal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionCallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyAccessExpression {
    pub base: Box<Expression>,
    pub property_name: Identifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputedPropertyAccessExpression {
    pub base: Box<Expression>,
    pub property: Box<Expression>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariableAccessExpression {
    pub var_name: Identifier,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Identifier(NonEmptyString);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum Literal {
    Boolean(bool),
    Numeric(NumericLiteral),
    // TODO: Store string literals in the constant pool.
    String(String),
    Array(Vec<Expression>),
    Function {
        name: Option<Identifier>,
        param_names: Vec<Identifier>,
        body: Block,
    },
    Object(HashMap<Identifier, Expression>),
    Null,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum NumericLiteral {
    /// Numeric literal tokens are **always unsigned** (but can be made negative at runtime with the
    /// negation unary operator).
    Int(u64),
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
    ComputedPropertyAccess,
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TernaryOperator;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GroupingOperator;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FunctionCallOperator;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PropertyAccessOperator;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ComputedPropertyAccessOperator;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum VariableDeclarationKind {
    Const,
    Let,
    Var,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl Program {
    pub fn new(body: Block) -> Self {
        Self { body }
    }

    pub fn body(&self) -> &Block {
        &self.body
    }
}

impl Node for Program {}

impl Block {
    pub fn new(hoisted_decls: Vec<DeclarationStatement>, stmts: Vec<Statement>) -> Self {
        Self {
            hoisted_decls,
            stmts,
        }
    }

    pub fn hoisted_declarations(&self) -> &[DeclarationStatement] {
        &self.hoisted_decls
    }

    pub fn statements(&self) -> &[Statement] {
        &self.stmts
    }
}

impl Node for Block {}

impl Node for Statement {}

impl DeclarationStatement {
    pub fn is_hoisted(&self) -> bool {
        match self {
            Self::Function(..) => true,
            Self::Variable(decl) => decl.is_hoisted(),
        }
    }
}

impl Node for DeclarationStatement {}

impl Node for IfStatement {}

impl Node for ForLoop {}

impl Node for WhileLoop {}

impl Node for BreakStatement {}

impl Node for ContinueStatement {}

impl Node for ReturnStatement {}

impl Node for ThrowStatement {}

impl Node for TryStatement {}

impl Node for CatchBlock {}

impl Node for FinallyBlock {}

impl Node for FunctionDeclaration {}

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
                    op: AssignmentOperator::Assign,
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

impl Node for Expression {}

impl Node for AssignmentExpression {}

impl Node for BinaryExpression {}

impl Node for UnaryExpression {}

impl Node for TernaryExpression {}

impl Node for GroupingExpression {}

impl Node for LiteralExpression {}

impl Node for FunctionCallExpression {}

impl Node for PropertyAccessExpression {}

impl Node for ComputedPropertyAccessExpression {}

impl Node for VariableAccessExpression {}

impl Identifier {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<NonEmptyString> for Identifier {
    fn from(s: NonEmptyString) -> Self {
        Self(s)
    }
}

impl From<i64> for Identifier {
    fn from(value: i64) -> Self {
        let s = value.to_string();
        // Safety: The string can't be empty because it was created from a number.
        Self(unsafe { NonEmptyString::from_unchecked(s) })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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
            Self::ComputedPropertyAccess => ComputedPropertyAccessOperator.associativity(),
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
            Self::ComputedPropertyAccess => ComputedPropertyAccessOperator.precedence(),
        }
    }
}

impl Op for AssignmentOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(3)
    }
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

impl Op for TernaryOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> Precedence {
        Precedence(4)
    }
}

impl Op for GroupingOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(21)
    }
}

impl Op for FunctionCallOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

impl Op for PropertyAccessOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

impl Op for ComputedPropertyAccessOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> Precedence {
        Precedence(20)
    }
}

impl Precedence {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(21);
}
