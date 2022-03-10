use super::block::Block;
use super::expression::{
    AssignmentExpression, AssignmentOperator, Expression, VariableAccessExpression,
};
use super::identifier::Identifier;
use super::Node;
use serde::{Deserialize, Serialize};

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
