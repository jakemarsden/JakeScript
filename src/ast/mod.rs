use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    block: Vec<Node>,
}

impl Program {
    pub fn new(block: Vec<Node>) -> Self {
        Self { block }
    }

    pub fn block(&self) -> &[Node] {
        &self.block
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    BinaryOp(BinaryOp, Box<(Node, Node)>),
    Block(Vec<Node>),
    Constant(Constant),
    /// Load a value from a variable visible from the current scope. Could be a local variable,
    /// function argument, etc.
    Local(String),
    /// Declare a local variable, optionally with an initialiser.
    LocalVarDecl(String, Option<Box<Node>>),

    Invalid(ParseError),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Character(char),
    Integer(u64),
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
