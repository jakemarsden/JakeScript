use super::declaration::Declaration;
use super::statement::Statement;
use super::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Script {
    body: Block,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Block {
    hoisted_declarations: Vec<Declaration>,
    body: Vec<BlockItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Statement),
}

impl Script {
    pub fn new(body: Block) -> Self {
        Self { body }
    }

    pub fn body(&self) -> &Block {
        &self.body
    }
}

impl Node for Script {}

impl Block {
    pub fn new(hoisted_declarations: Vec<Declaration>, body: Vec<BlockItem>) -> Self {
        Self {
            hoisted_declarations,
            body,
        }
    }

    pub fn hoisted_declarations(&self) -> &[Declaration] {
        &self.hoisted_declarations
    }

    pub fn body(&self) -> &[BlockItem] {
        &self.body
    }
}

impl Node for Block {}

impl Node for BlockItem {}
