use crate::ast::{ConstantId, VariableDeclarationKind};
use crate::interpreter::error::*;
use crate::interpreter::value::Value;
use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct CallStack {
    frame: CallFrame,
}

impl CallStack {
    pub fn frame(&mut self) -> &mut CallFrame {
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

#[derive(Debug, Default)]
pub struct CallFrame {
    scope: Scope,
    parent: Option<Box<CallFrame>>,
}

impl CallFrame {
    pub fn scope(&mut self) -> &mut Scope {
        &mut self.scope
    }

    pub fn push_scope(&mut self) {
        self.push_scope_ctx(ScopeCtx::default());
    }

    pub fn push_scope_ctx(&mut self, scope_ctx: ScopeCtx) {
        let new_child_scope = Scope::new_child_of(scope_ctx, self.scope.clone());
        self.scope = new_child_scope;
    }

    pub fn pop_scope(&mut self) {
        let parent_scope = self.scope.parent_scope();
        self.scope = parent_scope.expect("Cannot pop top-level scope context");
    }
}

#[derive(Clone, Default, Debug)]
pub struct Scope(Rc<RefCell<ScopeInner>>);

impl Scope {
    pub fn new(ctx: ScopeCtx) -> Self {
        Self(Rc::new(RefCell::new(ScopeInner { ctx, parent: None })))
    }

    pub fn new_child_of(ctx: ScopeCtx, parent: Self) -> Self {
        Self(Rc::new(RefCell::new(ScopeInner {
            ctx,
            parent: Some(parent.0),
        })))
    }

    pub fn parent_scope(&self) -> Option<Self> {
        if let Some(parent_ref) = &RefCell::borrow(&self.0).parent {
            let new_parent_ref = Rc::clone(parent_ref);
            Some(Self(new_parent_ref))
        } else {
            None
        }
    }

    pub fn lookup_variable(&self, name: ConstantId) -> Result<Variable, VariableNotDefinedError> {
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

#[derive(Default, Debug)]
struct ScopeInner {
    ctx: ScopeCtx,
    parent: Option<Rc<RefCell<ScopeInner>>>,
}

impl ScopeInner {
    fn lookup_variable(&self, name: ConstantId) -> Option<Variable> {
        if let Some(variable) = self.ctx.find_variable(name) {
            Some(variable)
        } else if let Some(ref parent) = self.parent {
            RefCell::borrow(parent).lookup_variable(name)
        } else {
            None
        }
    }

    fn declare_variable(&mut self, variable: Variable) -> Result<(), VariableAlreadyDefinedError> {
        if self.lookup_variable(variable.name()).is_none() {
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

    pub fn find_variable(&self, var_name: ConstantId) -> Option<Variable> {
        self.declared_variables
            .iter()
            .find(|var| var.name() == var_name)
            .cloned()
    }

    pub fn declare_variable(&mut self, variable: Variable) {
        self.declared_variables.push(variable);
    }
}

#[derive(Clone, Debug)]
pub struct Variable(Rc<RefCell<VariableInner>>);

impl Variable {
    pub fn new_unassigned(kind: VariableDeclarationKind, name: ConstantId) -> Self {
        Self::new(kind, name, Value::default())
    }

    pub fn new(kind: VariableDeclarationKind, name: ConstantId, initial_value: Value) -> Self {
        Self(Rc::new(RefCell::new(VariableInner {
            kind,
            name,
            value: initial_value,
        })))
    }

    pub fn kind(&self) -> VariableDeclarationKind {
        let inner = RefCell::borrow(&self.0);
        inner.kind
    }

    pub fn name(&self) -> ConstantId {
        let inner = RefCell::borrow(&self.0);
        inner.name
    }

    pub fn value(&self) -> Ref<Value> {
        let inner = RefCell::borrow(&self.0);
        Ref::map(inner, |inner| &inner.value)
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), AssignToConstVariableError> {
        let mut inner = RefCell::borrow_mut(&self.0);
        match inner.kind {
            VariableDeclarationKind::Let => {
                (*inner).value = value;
                Ok(())
            }
            VariableDeclarationKind::Const => Err(AssignToConstVariableError),
        }
    }
}

#[derive(Debug)]
struct VariableInner {
    kind: VariableDeclarationKind,
    name: ConstantId,
    value: Value,
}
