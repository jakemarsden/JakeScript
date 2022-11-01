use super::error::{AssignToConstVariableError, OutOfStackSpaceError, VariableAlreadyDefinedError};
use super::value::Value;
use crate::ast::{Identifier, LexicalDeclarationKind};
use crate::interpreter::{Reference, VariableNotDefinedError};

pub struct CallStack {
    root: CallFrame,
    frames: Vec<CallFrame>,
    scopes: ScopeStack,
}

impl Default for CallStack {
    fn default() -> Self {
        let mut scopes = ScopeStack::default();
        let root_scope = scopes.create_root(Vec::default()).unwrap();
        let root = CallFrame {
            scope: root_scope,
            receiver: None,
        };
        Self {
            root,
            frames: Vec::default(),
            scopes,
        }
    }
}

impl CallStack {
    fn frame(&self) -> &CallFrame {
        self.frames.last().unwrap_or(&self.root)
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        self.frames.last_mut().unwrap_or(&mut self.root)
    }

    pub fn push_empty_frame(&mut self) -> Result<(), OutOfStackSpaceError> {
        let root_scope = self.scopes.create_root(Vec::default())?;
        self.push_frame(root_scope, None)
    }

    pub fn push_frame_with_variables_in_scope(
        &mut self,
        variables: Vec<Variable>,
    ) -> Result<(), OutOfStackSpaceError> {
        let root_scope = self.scopes.create_root(variables)?;
        self.push_frame(root_scope, None)
    }

    pub fn push_frame_with_existing_scope(
        &mut self,
        existing_scope: ScopeId,
        receiver: Option<Reference>,
    ) -> Result<(), OutOfStackSpaceError> {
        self.push_frame(existing_scope, receiver)
    }

    fn push_frame(
        &mut self,
        root_scope: ScopeId,
        receiver: Option<Reference>,
    ) -> Result<(), OutOfStackSpaceError> {
        if self.frames.len() != usize::MAX {
            self.frames.push(CallFrame {
                scope: root_scope,
                receiver,
            });
            Ok(())
        } else {
            Err(OutOfStackSpaceError::new())
        }
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop().expect("cannot pop the root call frame");
    }

    pub fn scope(&self) -> ScopeId {
        self.frame().scope
    }

    pub fn receiver(&self) -> Option<&Reference> {
        self.frame().receiver.as_ref()
    }

    pub fn push_empty_scope(
        &mut self,
        escalation_boundary: bool,
    ) -> Result<(), OutOfStackSpaceError> {
        self.push_scope(escalation_boundary, Vec::default())
    }

    pub fn push_scope(
        &mut self,
        escalation_boundary: bool,
        variables: Vec<Variable>,
    ) -> Result<(), OutOfStackSpaceError> {
        let parent = self.frame().scope;
        let scope = self
            .scopes
            .create_child(parent, escalation_boundary, variables)?;
        self.frame_mut().scope = scope;
        Ok(())
    }

    pub fn pop_scope(&mut self) {
        self.frame_mut().scope = self.scopes.pop(self.frame().scope);
    }

    pub fn lookup_variable(&self, name: &Identifier) -> Result<&Variable, VariableNotDefinedError> {
        self.scopes.lookup_variable(self.frame().scope, name)
    }

    pub fn with_variable_mut<R>(
        &mut self,
        name: &Identifier,
        op: impl FnOnce(&mut Variable) -> R,
    ) -> Result<R, VariableNotDefinedError> {
        self.scopes.with_variable_mut(self.frame().scope, name, op)
    }

    pub fn declare_variable(
        &mut self,
        variable: Variable,
    ) -> Result<(), VariableAlreadyDefinedError> {
        self.scopes.declare_variable(self.frame().scope, variable)
    }

    pub fn declare_variable_within_escalation_boundary(
        &mut self,
        variable: Variable,
    ) -> Result<(), VariableAlreadyDefinedError> {
        self.scopes
            .declare_variable_within_escalation_boundary(self.frame().scope, variable)
    }
}

struct CallFrame {
    scope: ScopeId,
    receiver: Option<Reference>,
}

#[derive(Default)]
struct ScopeStack {
    scopes: Vec<Scope>,
}

impl ScopeStack {
    fn create_root(&mut self, variables: Vec<Variable>) -> Result<ScopeId, OutOfStackSpaceError> {
        let scope = Scope::new_root(variables);
        self.allocate(scope)
    }

    fn create_child(
        &mut self,
        parent: ScopeId,
        escalation_boundary: bool,
        variables: Vec<Variable>,
    ) -> Result<ScopeId, OutOfStackSpaceError> {
        let scope = Scope::new_child(parent, escalation_boundary, variables);
        self.allocate(scope)
    }

    fn pop(&mut self, id: ScopeId) -> ScopeId {
        let scope = self.lookup_mut(id);
        scope.parent.expect("cannot pop the root scope")
    }

    fn lookup_variable(
        &self,
        mut search_id: ScopeId,
        name: &Identifier,
    ) -> Result<&Variable, VariableNotDefinedError> {
        loop {
            let scope = self.lookup(search_id);
            let parent = scope.parent;
            if let Some(variable) = scope.lookup_variable(name) {
                break Ok(variable);
            }
            search_id = parent.ok_or_else(|| VariableNotDefinedError::new(name.clone()))?;
        }
    }

    fn with_variable_mut<R>(
        &mut self,
        mut search_id: ScopeId,
        name: &Identifier,
        op: impl FnOnce(&mut Variable) -> R,
    ) -> Result<R, VariableNotDefinedError> {
        loop {
            let scope = self.lookup_mut(search_id);
            let parent = scope.parent;
            if let Some(variable) = scope.lookup_variable_mut(name) {
                break Ok(op(variable));
            }
            search_id = parent.ok_or_else(|| VariableNotDefinedError::new(name.clone()))?;
        }
    }

    fn declare_variable(
        &mut self,
        scope: ScopeId,
        variable: Variable,
    ) -> Result<(), VariableAlreadyDefinedError> {
        self.lookup_mut(scope).declare_variable(variable)
    }

    fn declare_variable_within_escalation_boundary(
        &mut self,
        scope: ScopeId,
        variable: Variable,
    ) -> Result<(), VariableAlreadyDefinedError> {
        let mut id = scope;
        loop {
            let scope = self.lookup_mut(id);
            if scope.is_escalation_boundary() {
                break scope.declare_variable(variable);
            }
            id = scope
                .parent
                .expect("the root scope should always be an escalation boundary");
        }
    }

    fn lookup(&self, id: ScopeId) -> &Scope {
        &self.scopes[id.0]
    }

    fn lookup_mut(&mut self, id: ScopeId) -> &mut Scope {
        &mut self.scopes[id.0]
    }

    fn allocate(&mut self, scope: Scope) -> Result<ScopeId, OutOfStackSpaceError> {
        let idx = self.scopes.len();
        if idx != usize::MAX {
            self.scopes.push(scope);
            Ok(ScopeId(idx))
        } else {
            Err(OutOfStackSpaceError::new())
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScopeId(usize);

#[derive(Debug)]
struct Scope {
    parent: Option<ScopeId>,
    escalation_boundary: bool,
    slots: Vec<Variable>,
}

impl Scope {
    fn new_root(variables: Vec<Variable>) -> Self {
        Self {
            parent: None,
            escalation_boundary: true,
            slots: variables,
        }
    }

    fn new_child(parent: ScopeId, escalation_boundary: bool, variables: Vec<Variable>) -> Self {
        Self {
            parent: Some(parent),
            escalation_boundary,
            slots: variables,
        }
    }

    fn is_escalation_boundary(&self) -> bool {
        self.escalation_boundary
    }

    fn lookup_variable(&self, name: &Identifier) -> Option<&Variable> {
        self.slots.iter().find(|var| &var.name == name)
    }

    fn lookup_variable_mut(&mut self, name: &Identifier) -> Option<&mut Variable> {
        self.slots.iter_mut().find(|var| &var.name == name)
    }

    fn declare_variable(&mut self, variable: Variable) -> Result<(), VariableAlreadyDefinedError> {
        if self.lookup_variable(variable.name()).is_none() {
            self.slots.push(variable);
            Ok(())
        } else {
            Err(VariableAlreadyDefinedError::new(variable.name().clone()))
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    kind: VariableKind,
    name: Identifier,
    value: Value,
}

impl Variable {
    pub fn new_unassigned(kind: VariableKind, name: Identifier) -> Self {
        Self::new(kind, name, Value::default())
    }

    pub fn new(kind: VariableKind, name: Identifier, initial_value: Value) -> Self {
        Self {
            kind,
            name,
            value: initial_value,
        }
    }

    pub fn kind(&self) -> VariableKind {
        self.kind
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), AssignToConstVariableError> {
        match self.kind {
            VariableKind::Let | VariableKind::Var => {
                self.value = value;
                Ok(())
            }
            VariableKind::Const => Err(AssignToConstVariableError::new(self.name().clone())),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VariableKind {
    Const,
    Let,
    Var,
}

impl From<LexicalDeclarationKind> for VariableKind {
    fn from(decl_kind: LexicalDeclarationKind) -> Self {
        match decl_kind {
            LexicalDeclarationKind::Const => Self::Const,
            LexicalDeclarationKind::Let => Self::Let,
        }
    }
}
