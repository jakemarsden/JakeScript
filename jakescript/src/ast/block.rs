use super::declaration::Declaration;
use super::statement::Statement;
use super::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Program {
    body: Block,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Block {
    hoisted_decls: Vec<Declaration>,
    body: Vec<BlockItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Statement),
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

impl Block {
    pub fn new(hoisted_decls: Vec<Declaration>, body: Vec<BlockItem>) -> Self {
        Self {
            hoisted_decls,
            body,
        }
    }

    pub fn hoisted_declarations(&self) -> &[Declaration] {
        &self.hoisted_decls
    }

    pub fn body(&self) -> &[BlockItem] {
        &self.body
    }
}

impl Node for Block {}

impl Node for BlockItem {}
