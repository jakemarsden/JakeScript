use super::block::Block;
use super::expression::{
    AssignmentExpression, AssignmentOperator, Expression, IdentifierReferenceExpression,
};
use super::identifier::Identifier;
use super::Node;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "declaration_type")]
pub enum Declaration {
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
    Lexical(LexicalDeclaration),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FunctionDeclaration {
    pub loc: SourceLocation,
    pub binding: Identifier,
    pub formal_parameters: Vec<Identifier>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VariableDeclaration {
    pub loc: SourceLocation,
    pub bindings: Vec<VariableBinding>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LexicalDeclaration {
    pub loc: SourceLocation,
    pub kind: LexicalDeclarationKind,
    pub bindings: Vec<VariableBinding>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LexicalDeclarationKind {
    Const,
    Let,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VariableBinding {
    pub loc: SourceLocation,
    pub identifier: Identifier,
    pub initialiser: Option<Expression>,
}

impl Declaration {
    pub fn is_hoisted(&self) -> bool {
        match self {
            Self::Function(..) | Self::Variable(..) => true,
            Self::Lexical(..) => false,
        }
    }

    pub fn into_declaration_and_initialiser(self) -> (Self, Vec<Expression>) {
        match self {
            Self::Function(..) | Self::Lexical(..) => (self, vec![]),
            Self::Variable(node) => {
                let (decl, initialisers) = node.into_declaration_and_initialiser();
                (Self::Variable(decl), initialisers)
            }
        }
    }
}

impl Node for Declaration {
    fn source_location(&self) -> &SourceLocation {
        match self {
            Self::Function(node) => node.source_location(),
            Self::Variable(node) => node.source_location(),
            Self::Lexical(node) => node.source_location(),
        }
    }
}

impl Node for FunctionDeclaration {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl VariableDeclaration {
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
                            loc: entry.source_location().clone(),
                        },
                    )),
                    rhs: Box::new(initialiser),
                    loc: entry.source_location().clone(),
                }));
            }
        }
        (self, initialisers)
    }
}

impl Node for VariableDeclaration {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl Node for LexicalDeclaration {
    fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl VariableBinding {
    pub fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}
