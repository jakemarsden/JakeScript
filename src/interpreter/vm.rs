use std::collections::HashMap;
use std::mem;

#[derive(Default)]
pub struct Vm {
    scope: ScopeCtx,
}

impl Vm {
    pub fn peek_scope(&self) -> &ScopeCtx {
        &self.scope
    }

    pub fn peek_scope_mut(&mut self) -> &mut ScopeCtx {
        &mut self.scope
    }

    pub fn push_scope(&mut self) {
        let mut scope = ScopeCtx::default();
        mem::swap(&mut self.scope, &mut scope);
        assert!(self.scope.parent.is_none());
        self.scope.parent = Some(Box::new(scope));
    }

    pub fn pop_scope(&mut self) {
        let parent_scope = *self.scope.parent.take().expect("Cannot pop global scope");
        self.scope = parent_scope;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Null,
    Numeric(u64),
    String(String),
    Undefined,
}

impl Default for Value {
    fn default() -> Self {
        Self::Undefined
    }
}

#[derive(Clone, Debug, Default)]
pub struct ScopeCtx {
    locals: HashMap<String, Value>,
    parent: Option<Box<ScopeCtx>>,
}

impl ScopeCtx {
    pub fn lookup_local(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.locals.get(name) {
            Some(value)
        } else if let Some(ref parent) = self.parent {
            parent.lookup_local(name)
        } else {
            None
        }
    }

    pub fn init_local(&mut self, name: String, value: Value) -> Result<(), ()> {
        self.locals
            .try_insert(name, value)
            .map(|_| ())
            .map_err(|_| ())
    }
}
