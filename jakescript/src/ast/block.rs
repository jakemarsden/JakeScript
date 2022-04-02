use super::declaration::Declaration;
use super::statement::Statement;
use super::Node;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Script {
    body: Block,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Block {
    hoisted_declarations: Vec<Declaration>,
    body: Vec<BlockItem>,
    loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

impl Node for Script {
    fn source_location(&self) -> &SourceLocation {
        self.body.source_location()
    }
}

impl Block {
    pub fn new(
        hoisted_declarations: Vec<Declaration>,
        body: Vec<BlockItem>,
        loc: SourceLocation,
    ) -> Self {
        Self {
            hoisted_declarations,
            body,
            loc,
        }
    }

    pub fn hoisted_declarations(&self) -> &[Declaration] {
        &self.hoisted_declarations
    }

    pub fn body(&self) -> &[BlockItem] {
        &self.body
    }
}

impl Node for Block {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for BlockItem {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::Declaration(node) => node.source_location(),
            Self::Statement(node) => node.source_location(),
        }
    }
}
