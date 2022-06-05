use super::error::{AssignToConstVariableError, VariableAlreadyDefinedError};
use super::heap::Reference;
use super::value::Value;
use crate::ast::{Identifier, LexicalDeclarationKind};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct CallStack {
    root: CallFrame,
    frames: Vec<CallFrame>,
}

#[derive(Debug, Default)]
pub struct CallFrame {
    scope: Scope,
    receiver: Option<Reference>,
}

#[derive(Clone, Debug)]
pub struct Scope(Rc<RefCell<ScopeInner>>);

#[derive(Debug)]
struct ScopeInner {
    ctx: ScopeCtx,
    escalation_boundary: bool,
    parent: Option<Rc<RefCell<ScopeInner>>>,
}

#[derive(Debug, Default)]
pub struct ScopeCtx {
    declared_variables: Vec<Variable>,
}

#[derive(Clone, Debug)]
pub struct Variable(Rc<RefCell<VariableInner>>);

#[derive(Debug)]
struct VariableInner {
    kind: VariableKind,
    name: Identifier,
    value: Value,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VariableKind {
    Const,
    Let,
    Var,
}

impl CallStack {
    pub fn frame(&self) -> &CallFrame {
        self.frames.last().unwrap_or(&self.root)
    }
    pub fn frame_mut(&mut self) -> &mut CallFrame {
        self.frames.last_mut().unwrap_or(&mut self.root)
    }

    pub fn push_frame(&mut self, scope: Scope, receiver: Option<Reference>) {
        self.frames.push(CallFrame { scope, receiver });
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop().expect("Cannot pop top-level call frame");
    }
}

impl CallFrame {
    pub fn scope(&self) -> &Scope {
        &self.scope
    }
    pub fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scope
    }

    pub fn receiver(&self) -> Option<&Reference> {
        self.receiver.as_ref()
    }

    pub fn push_empty_scope(&mut self) {
        self.push_scope(ScopeCtx::default(), false);
    }

    pub fn push_scope(&mut self, scope_ctx: ScopeCtx, escalation_boundary: bool) {
        let parent = self.scope.clone();
        self.scope = Scope::new_child_of(scope_ctx, escalation_boundary, parent);
    }

    pub fn pop_scope(&mut self) {
        let parent_scope = self.scope.parent();
        self.scope = parent_scope.expect("Cannot pop top-level scope context");
    }
}

impl Scope {
    fn new_child_of(ctx: ScopeCtx, escalation_boundary: bool, parent: Self) -> Self {
        Self(Rc::new(RefCell::new(ScopeInner {
            ctx,
            escalation_boundary,
            parent: Some(parent.0),
        })))
    }

    pub fn is_escalation_boundary(&self) -> bool {
        RefCell::borrow(&self.0).is_escalation_boundary()
    }

    pub fn parent(&self) -> Option<Self> {
        if let Some(parent_ref) = &RefCell::borrow(&self.0).parent {
            let new_parent_ref = Rc::clone(parent_ref);
            Some(Self(new_parent_ref))
        } else {
            None
        }
    }

    pub fn ancestor(&self, within_escalation_bounds: bool) -> Self {
        let mut ancestor_ref = self.clone();
        while let Some(parent_ref) = ancestor_ref.parent() {
            if within_escalation_bounds && ancestor_ref.is_escalation_boundary() {
                break;
            }
            ancestor_ref = parent_ref;
        }
        ancestor_ref
    }

    pub fn lookup_variable(&self, name: &Identifier) -> Option<Variable> {
        RefCell::borrow(&self.0).lookup_variable(name)
    }

    pub fn declare_variable(
        &mut self,
        variable: Variable,
    ) -> Result<(), VariableAlreadyDefinedError> {
        RefCell::borrow_mut(&self.0).declare_variable(variable)
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(ScopeInner {
            ctx: ScopeCtx::default(),
            escalation_boundary: true,
            parent: None,
        })))
    }
}

impl ScopeInner {
    fn is_escalation_boundary(&self) -> bool {
        self.escalation_boundary
    }

    fn lookup_variable(&self, name: &Identifier) -> Option<Variable> {
        if let Some(variable) = self.ctx.find_variable(name) {
            Some(variable)
        } else if let Some(ref parent) = self.parent {
            RefCell::borrow(parent).lookup_variable(name)
        } else {
            None
        }
    }

    fn declare_variable(&mut self, variable: Variable) -> Result<(), VariableAlreadyDefinedError> {
        if self.lookup_variable(&variable.name()).is_none() {
            self.ctx.declare_variable(variable);
            Ok(())
        } else {
            Err(VariableAlreadyDefinedError)
        }
    }
}

impl ScopeCtx {
    pub fn new(declared_variables: Vec<Variable>) -> Self {
        Self { declared_variables }
    }

    pub fn find_variable(&self, var_name: &Identifier) -> Option<Variable> {
        self.declared_variables
            .iter()
            .find(|var| &*var.name() == var_name)
            .cloned()
    }

    pub fn declare_variable(&mut self, variable: Variable) {
        self.declared_variables.push(variable);
    }
}

impl Variable {
    pub fn new_unassigned(kind: VariableKind, name: Identifier) -> Self {
        Self::new(kind, name, Value::default())
    }

    pub fn new(kind: VariableKind, name: Identifier, initial_value: Value) -> Self {
        Self(Rc::new(RefCell::new(VariableInner {
            kind,
            name,
            value: initial_value,
        })))
    }

    pub fn kind(&self) -> VariableKind {
        let inner = RefCell::borrow(&self.0);
        inner.kind
    }

    pub fn name(&self) -> Ref<Identifier> {
        let inner = RefCell::borrow(&self.0);
        Ref::map(inner, |inner| &inner.name)
    }

    pub fn value(&self) -> Ref<Value> {
        let inner = RefCell::borrow(&self.0);
        Ref::map(inner, |inner| &inner.value)
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), AssignToConstVariableError> {
        let mut inner = RefCell::borrow_mut(&self.0);
        match inner.kind {
            VariableKind::Let | VariableKind::Var => {
                (*inner).value = value;
                Ok(())
            }
            VariableKind::Const => Err(AssignToConstVariableError),
        }
    }
}

impl From<LexicalDeclarationKind> for VariableKind {
    fn from(decl_kind: LexicalDeclarationKind) -> Self {
        match decl_kind {
            LexicalDeclarationKind::Const => Self::Const,
            LexicalDeclarationKind::Let => Self::Let,
        }
    }
}
