use super::block::Block;
use super::expression::{
    AssignmentExpression, AssignmentOperator, Expression, IdentifierReferenceExpression,
};
use super::identifier::Identifier;
use super::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "declaration_type")]
pub enum Declaration {
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FunctionDeclaration {
    pub binding: Identifier,
    pub formal_parameters: Vec<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub bindings: Vec<VariableBinding>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum VariableDeclarationKind {
    Const,
    Let,
    Var,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VariableBinding {
    pub identifier: Identifier,
    pub initialiser: Option<Expression>,
}

impl Declaration {
    pub fn is_hoisted(&self) -> bool {
        match self {
            Self::Function(..) => true,
            Self::Variable(node) => node.is_hoisted(),
        }
    }

    pub fn into_declaration_and_initialiser(self) -> (Self, Vec<Expression>) {
        match self {
            Self::Function(..) => (self, vec![]),
            Self::Variable(node) => {
                let (decl, initialisers) = node.into_declaration_and_initialiser();
                (Self::Variable(decl), initialisers)
            }
        }
    }
}

impl Node for Declaration {}

impl Node for FunctionDeclaration {}

impl VariableDeclaration {
    pub fn is_escalated(&self) -> bool {
        match self.kind {
            VariableDeclarationKind::Const | VariableDeclarationKind::Let => false,
            VariableDeclarationKind::Var => true,
        }
    }

    pub fn is_hoisted(&self) -> bool {
        match self.kind {
            VariableDeclarationKind::Const | VariableDeclarationKind::Let => false,
            VariableDeclarationKind::Var => true,
        }
    }

    /// Split the declaration into
    ///
    /// 1. a "main" [`VariableDeclaration`], sans initialisers, to declare each entry
    /// 2. a new, synthesised [`Expression`] to initialise each entry, for each entry which started
    /// with an initialiser.
    pub fn into_declaration_and_initialiser(mut self) -> (Self, Vec<Expression>) {
        let mut initialisers = Vec::with_capacity(self.bindings.len());
        for entry in &mut self.bindings {
            if let Some(initialiser) = entry.initialiser.take() {
                // Synthesise an assignment expression to initialise the variable
                initialisers.push(Expression::Assignment(AssignmentExpression {
                    op: AssignmentOperator::Assign,
                    lhs: Box::new(Expression::IdentifierReference(
                        IdentifierReferenceExpression {
                            identifier: entry.identifier.clone(),
                        },
                    )),
                    rhs: Box::new(initialiser),
                }));
            }
        }
        (self, initialisers)
    }
}

impl Node for VariableDeclaration {}
