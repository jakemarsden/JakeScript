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
    loc: SourceLocation,
    hoisted_declarations: Vec<Declaration>,
    body: Vec<Statement>,
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
        loc: SourceLocation,
        hoisted_declarations: Vec<Declaration>,
        body: Vec<Statement>,
    ) -> Self {
        Self {
            loc,
            hoisted_declarations,
            body,
        }
    }

    pub fn hoisted_declarations(&self) -> &[Declaration] {
        &self.hoisted_declarations
    }

    pub fn body(&self) -> &[Statement] {
        &self.body
    }
}

impl Node for Block {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}
