use crate::interpreter::error::{VariableAlreadyDefinedError, VariableNotDefinedError};
use std::collections::HashMap;
use std::{fmt, mem};

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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Null => write!(f, "null"),
            Self::Numeric(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, r#""{}""#, value),
            Self::Undefined => write!(f, "undefined"),
        }
    }
}

impl Value {
    pub fn as_boolean(&self) -> bool {
        match self {
            Self::Boolean(value) => *value,
            Self::Numeric(value) => *value > 0,
            Self::String(value) => !value.is_empty(),
            Self::Null | Self::Undefined => false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ScopeCtx {
    locals: HashMap<String, Value>,
    parent: Option<Box<ScopeCtx>>,
}

impl ScopeCtx {
    pub fn lookup_local(&self, name: &str) -> Result<&Value, VariableNotDefinedError> {
        if let Some(value) = self.locals.get(name) {
            Ok(value)
        } else if let Some(ref parent) = self.parent {
            parent.lookup_local(name)
        } else {
            Err(VariableNotDefinedError::new(name.to_owned()))
        }
    }

    pub fn init_local(
        &mut self,
        name: String,
        value: Value,
    ) -> Result<(), VariableAlreadyDefinedError> {
        self.locals
            .try_insert(name, value)
            .map(|_| ())
            .map_err(|err| VariableAlreadyDefinedError::new(err.entry.key().to_owned()))
    }
}
