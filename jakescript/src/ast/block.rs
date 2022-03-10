use super::statement::{DeclarationStatement, Statement};
use super::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Program {
    body: Block,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Block {
    hoisted_decls: Vec<DeclarationStatement>,
    stmts: Vec<Statement>,
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
