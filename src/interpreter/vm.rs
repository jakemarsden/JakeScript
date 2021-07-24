use crate::ast::VariableDeclKind;
use crate::interpreter::error::*;
use std::collections::HashMap;
use std::{fmt, mem, ops};

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
    Numeric(i64),
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

    pub fn as_numeric(&self) -> i64 {
        match self {
            Self::Numeric(value) => *value,
            value => todo!("Value::as_numeric: {}", value),
        }
    }
}

impl ops::Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        Self::Boolean(!self.as_boolean())
    }
}

#[derive(Debug, Default)]
pub struct ScopeCtx {
    locals: HashMap<String, Variable>,
    parent: Option<Box<ScopeCtx>>,
}

impl ScopeCtx {
    pub fn init_variable(
        &mut self,
        kind: VariableDeclKind,
        name: String,
        initial_value: Value,
    ) -> Result<(), VariableAlreadyDefinedError> {
        let var = Variable {
            kind,
            value: initial_value,
        };
        self.locals
            .try_insert(name, var)
            .map(|_| ())
            .map_err(|err| VariableAlreadyDefinedError::new(err.entry.key().to_owned()))
    }

    pub fn resolve_variable(&self, name: &str) -> Result<&Value, VariableNotDefinedError> {
        self.lookup_variable(name).map(|var| &var.value)
    }

    pub fn set_variable(&mut self, name: &str, value: Value) -> Result<(), Error> {
        let mut var = self.lookup_variable_mut(name)?;
        match var.kind {
            VariableDeclKind::Let => {
                var.value = value;
                Ok(())
            }
            VariableDeclKind::Const => Err(VariableIsConstError::new(name.to_owned()).into()),
        }
    }

    fn lookup_variable(&self, name: &str) -> Result<&Variable, VariableNotDefinedError> {
        if let Some(variable) = self.locals.get(name) {
            Ok(variable)
        } else if let Some(ref parent) = self.parent {
            parent.lookup_variable(name)
        } else {
            Err(VariableNotDefinedError::new(name.to_owned()))
        }
    }

    fn lookup_variable_mut(
        &mut self,
        name: &str,
    ) -> Result<&mut Variable, VariableNotDefinedError> {
        if let Some(variable) = self.locals.get_mut(name) {
            Ok(variable)
        } else if let Some(ref mut parent) = self.parent {
            parent.lookup_variable_mut(name)
        } else {
            Err(VariableNotDefinedError::new(name.to_owned()))
        }
    }
}

#[derive(Debug)]
struct Variable {
    kind: VariableDeclKind,
    value: Value,
}
