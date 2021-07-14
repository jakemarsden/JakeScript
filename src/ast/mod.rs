use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    block: Block,
}

impl Program {
    pub fn new(block: Block) -> Self {
        Self { block }
    }

    pub fn block(&self) -> &Block {
        &self.block
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block(Vec<Node>);

impl Block {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self(nodes)
    }

    pub fn nodes(&self) -> &[Node] {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    BinaryOp(BinaryOp, Box<(Node, Node)>),
    Block(Block),
    Constant(Constant),
    /// Load a value from a variable visible from the current scope. Could be a local variable,
    /// function argument, etc.
    Local(String),
    /// Declare a local variable, optionally with an initialiser.
    LocalVarDecl(String, Option<Box<Node>>),
    While(Box<Node>, Block),

    Invalid(ParseError),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Assign,
    LessThan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Boolean(bool),
    Integer(u64),
    Null,
    String(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ParseError {}
