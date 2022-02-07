use crate::ast::{Identifier, VariableDeclarationKind};
use crate::interpreter::error::{
    AssignToConstVariableError, VariableAlreadyDefinedError, VariableNotDefinedError,
};
use crate::interpreter::value::Value;
use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;

#[derive(Debug)]
pub struct CallStack {
    frame: CallFrame,
}

impl CallStack {
    pub fn new(root_frame: CallFrame) -> Self {
        Self { frame: root_frame }
    }

    pub fn frame(&self) -> &CallFrame {
        &self.frame
    }
    pub fn frame_mut(&mut self) -> &mut CallFrame {
        &mut self.frame
    }

    pub fn push_frame(&mut self, scope: Scope) {
        let new_frame = CallFrame {
            scope,
            parent: None,
        };
        let parent_frame = mem::replace(&mut self.frame, new_frame);
        self.frame.parent = Some(Box::new(parent_frame));
    }

    pub fn pop_frame(&mut self) {
        let parent_frame = self.frame.parent.take();
        self.frame = *parent_frame.expect("Cannot pop top-level call frame");
    }
}

#[derive(Debug)]
pub struct CallFrame {
    scope: Scope,
    parent: Option<Box<CallFrame>>,
}

impl CallFrame {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            parent: None,
        }
    }

    pub fn scope(&self) -> &Scope {
        &self.scope
    }
    pub fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scope
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

#[derive(Clone, Debug)]
pub struct Scope(Rc<RefCell<ScopeInner>>);

impl Scope {
    pub fn new(ctx: ScopeCtx) -> Self {
        Self(Rc::new(RefCell::new(ScopeInner {
            ctx,
            escalation_boundary: true,
            parent: None,
        })))
    }

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

    pub fn lookup_variable(&self, name: &Identifier) -> Result<Variable, VariableNotDefinedError> {
        RefCell::borrow(&self.0)
            .lookup_variable(name)
            .ok_or(VariableNotDefinedError)
    }

    pub fn declare_variable(
        &mut self,
        variable: Variable,
    ) -> Result<(), VariableAlreadyDefinedError> {
        RefCell::borrow_mut(&self.0).declare_variable(variable)
    }
}

#[derive(Debug)]
struct ScopeInner {
    ctx: ScopeCtx,
    escalation_boundary: bool,
    parent: Option<Rc<RefCell<ScopeInner>>>,
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

#[derive(Default, Debug)]
pub struct ScopeCtx {
    declared_variables: Vec<Variable>,
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

#[derive(Clone, Debug)]
pub struct Variable(Rc<RefCell<VariableInner>>);

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
            VariableKind::SilentReadOnly => Ok(()),
        }
    }
}

#[derive(Debug)]
struct VariableInner {
    kind: VariableKind,
    name: Identifier,
    value: Value,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VariableKind {
    Const,
    Let,
    Var,
    /// The assignment operator completes successfully, and returns the value you might expect, but
    /// the value of the variable isn't actually changed.
    SilentReadOnly,
}

impl From<VariableDeclarationKind> for VariableKind {
    fn from(decl_kind: VariableDeclarationKind) -> Self {
        match decl_kind {
            VariableDeclarationKind::Const => Self::Const,
            VariableDeclarationKind::Let => Self::Let,
            VariableDeclarationKind::Var => Self::Var,
        }
    }
}
