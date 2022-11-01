use super::declaration::Declaration;
use super::statement::Statement;
use crate::ast_node;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

ast_node!(
    #[derive(Default)]
    ##[source_location = |self| self.body.source_location()]
    pub struct Script {
        body: Block,
    }
);

impl Script {
    pub fn new(body: Block) -> Self {
        Self { body }
    }

    pub fn body(&self) -> &Block {
        &self.body
    }
}

ast_node!(
    #[derive(Default)]
    pub struct Block {
        loc: SourceLocation,
        hoisted_declarations: Vec<Declaration>,
        body: Vec<Statement>,
    }
);

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
