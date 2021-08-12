use crate::ast::{Block, Value, VariableDeclarationKind};
use crate::interpreter::error::*;
use std::cell::{Ref, RefCell};
use std::mem;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Default)]
pub struct Vm {
    stack: CallStack,
}

impl Vm {
    pub fn stack(&mut self) -> &mut CallStack {
        &mut self.stack
    }

    pub fn frame(&mut self) -> &mut CallFrame {
        self.stack().frame()
    }

    pub fn scope(&mut self) -> &mut Scope {
        self.frame().scope()
    }
}

#[derive(Debug, Default)]
pub struct CallStack {
    frame: CallFrame,
}

impl CallStack {
    pub fn frame(&mut self) -> &mut CallFrame {
        &mut self.frame
    }

    pub fn push_frame(&mut self) {
        let new_frame = CallFrame::default();
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
        let new_child_scope = Scope::new_child_of(ScopeCtx::default(), &self.scope);
        self.scope = new_child_scope;
    }

    pub fn pop_scope(&mut self) {
        let parent_scope = self.scope.parent_scope();
        self.scope = parent_scope.expect("Cannot pop top-level scope context");
    }
}

#[derive(Clone, Default, Debug)]
pub struct Scope {
    inner: Rc<RefCell<ScopeInner>>,
}

impl Scope {
    pub fn new(ctx: ScopeCtx) -> Self {
        let parent = None;
        Self {
            inner: Rc::new(RefCell::new(ScopeInner { ctx, parent })),
        }
    }

    pub fn new_child_of(ctx: ScopeCtx, parent: &Self) -> Self {
        let parent = Some(Rc::clone(&parent.inner));
        Self {
            inner: Rc::new(RefCell::new(ScopeInner { ctx, parent })),
        }
    }

    pub fn parent_scope(&self) -> Option<Self> {
        if let Some(parent_ref) = &RefCell::borrow(&self.inner).parent {
            let new_parent_ref = Rc::clone(parent_ref);
            Some(Self {
                inner: new_parent_ref,
            })
        } else {
            None
        }
    }

    pub fn lookup_variable(&self, name: &str) -> Result<Variable, VariableNotDefinedError> {
        ScopeInner::lookup_variable(&self.inner, name).ok_or(VariableNotDefinedError)
    }

    pub fn declare_variable(
        &mut self,
        kind: VariableDeclarationKind,
        name: String,
        initial_value: Value,
    ) -> Result<(), VariableAlreadyDefinedError> {
        ScopeInner::declare_variable(&mut self.inner, kind, name, initial_value)
    }

    pub fn lookup_function(&self, name: &str) -> Result<Function, FunctionNotDefinedError> {
        ScopeInner::lookup_function(&self.inner, name).ok_or(FunctionNotDefinedError)
    }

    pub fn declare_function(
        &mut self,
        name: String,
        parameters: Vec<String>,
        body: Block,
    ) -> Result<(), FunctionAlreadyDefinedError> {
        let declared_scope = self.clone();
        ScopeInner::declare_function(&mut self.inner, name, declared_scope, parameters, body)
    }
}

#[derive(Default, Debug)]
struct ScopeInner {
    ctx: ScopeCtx,
    parent: Option<Rc<RefCell<ScopeInner>>>,
}

impl ScopeInner {
    fn lookup_variable(self_: &Rc<RefCell<ScopeInner>>, name: &str) -> Option<Variable> {
        let inner = RefCell::borrow(self_);
        if let Some(variable) = inner.ctx.find_variable(name) {
            Some(variable)
        } else if let Some(ref parent) = inner.parent {
            Self::lookup_variable(parent, name)
        } else {
            None
        }
    }

    fn declare_variable(
        self_: &mut Rc<RefCell<ScopeInner>>,
        kind: VariableDeclarationKind,
        name: String,
        initial_value: Value,
    ) -> Result<(), VariableAlreadyDefinedError> {
        if Self::lookup_variable(self_, &name).is_none() {
            let mut inner = RefCell::borrow_mut(self_);
            inner
                .ctx
                .declare_variable(Variable::new(kind, name, initial_value));
            Ok(())
        } else {
            Err(VariableAlreadyDefinedError)
        }
    }

    fn lookup_function(self_: &Rc<RefCell<ScopeInner>>, name: &str) -> Option<Function> {
        let inner = RefCell::borrow(self_);
        if let Some(function) = inner.ctx.find_function(name) {
            Some(function)
        } else if let Some(ref parent) = inner.parent {
            Self::lookup_function(parent, name)
        } else {
            None
        }
    }

    fn declare_function(
        self_: &mut Rc<RefCell<ScopeInner>>,
        name: String,
        declared_scope: Scope,
        parameters: Vec<String>,
        body: Block,
    ) -> Result<(), FunctionAlreadyDefinedError> {
        if Self::lookup_function(self_, &name).is_none() {
            let mut inner = RefCell::borrow_mut(self_);
            inner
                .ctx
                .declare_function(Function::new(name, declared_scope, parameters, body));
            Ok(())
        } else {
            Err(FunctionAlreadyDefinedError)
        }
    }
}

#[derive(Default, Debug)]
pub struct ScopeCtx {
    local_variables: Vec<Variable>,
    local_functions: Vec<Function>,
}

impl ScopeCtx {
    pub fn new(local_variables: Vec<Variable>, local_functions: Vec<Function>) -> Self {
        Self {
            local_variables,
            local_functions,
        }
    }

    pub fn find_variable(&self, var_name: &str) -> Option<Variable> {
        self.local_variables
            .iter()
            .find(|var| var.name().deref() == var_name)
            .cloned()
    }

    pub fn declare_variable(&mut self, variable: Variable) {
        self.local_variables.push(variable);
    }

    pub fn find_function(&self, fun_name: &str) -> Option<Function> {
        self.local_functions
            .iter()
            .find(|fun| fun.name().deref() == fun_name)
            .cloned()
    }

    pub fn declare_function(&mut self, function: Function) {
        self.local_functions.push(function);
    }
}

#[derive(Clone, Debug)]
pub struct Variable(Rc<RefCell<VariableInner>>);

impl Variable {
    pub fn new(kind: VariableDeclarationKind, name: String, initial_value: Value) -> Self {
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

    pub fn name(&self) -> Ref<str> {
        let inner = RefCell::borrow(&self.0);
        Ref::map(inner, |inner| inner.name.as_str())
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
    name: String,
    value: Value,
}

#[derive(Clone, Debug)]
pub struct Function(Rc<RefCell<FunctionInner>>);

impl Function {
    pub fn new(name: String, declared_scope: Scope, parameters: Vec<String>, body: Block) -> Self {
        Self(Rc::new(RefCell::new(FunctionInner {
            name,
            declared_scope,
            parameters,
            body,
        })))
    }

    pub fn name(&self) -> Ref<str> {
        let inner = RefCell::borrow(&self.0);
        Ref::map(inner, |inner| inner.name.as_str())
    }

    pub fn declared_scope(&self) -> Ref<Scope> {
        let inner = RefCell::borrow(&self.0);
        Ref::map(inner, |inner| &inner.declared_scope)
    }

    pub fn parameters(&self) -> Ref<[String]> {
        let inner = RefCell::borrow(&self.0);
        Ref::map(inner, |inner| inner.parameters.as_slice())
    }

    pub fn body(&self) -> Ref<Block> {
        let inner = RefCell::borrow(&self.0);
        Ref::map(inner, |inner| &inner.body)
    }
}

#[derive(Debug)]
struct FunctionInner {
    name: String,
    declared_scope: Scope,
    parameters: Vec<String>,
    body: Block,
}
