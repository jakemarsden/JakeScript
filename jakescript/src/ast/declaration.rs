use super::expression::{AssignmentExpression, Expression, IdentifierReferenceExpression};
use super::identifier::Identifier;
use super::op::AssignmentOperator;
use super::{Block, Node};
use crate::ast_node;
use crate::token::SourceLocation;
use serde::{Deserialize, Serialize};

ast_node!(
    #[serde(tag = "declaration_type")]
    pub enum Declaration {
        Function(FunctionDeclaration),
        Lexical(LexicalDeclaration),
        Variable(VariableDeclaration),
    }
);

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

ast_node!(
    pub struct FunctionDeclaration {
        pub loc: SourceLocation,
        pub binding: Identifier,
        pub parameters: Vec<Identifier>,
        pub body: Block,
    }
);

ast_node!(
    pub struct LexicalDeclaration {
        pub loc: SourceLocation,
        pub kind: LexicalDeclarationKind,
        pub bindings: Vec<Binding>,
    }
);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LexicalDeclarationKind {
    Const,
    Let,
}

ast_node!(
    pub struct VariableDeclaration {
        pub loc: SourceLocation,
        pub bindings: Vec<Binding>,
    }
);

impl VariableDeclaration {
    /// Split the declaration into
    ///
    /// 1. a "main" [`VariableDeclaration`], sans initialisers, to declare each
    /// entry.
    /// 2. a new, synthesised [`Expression`] to initialise each
    /// entry, for each entry which started with an initialiser.
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

ast_node!(
    pub struct Binding {
        pub loc: SourceLocation,
        pub identifier: Identifier,
        pub initialiser: Option<Expression>,
    }
);
