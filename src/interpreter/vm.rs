use crate::ast::{Value, VariableDeclarationKind};
use crate::interpreter::error::*;
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

#[derive(Debug, Default)]
pub struct ScopeCtx {
    locals: HashMap<String, Variable>,
    parent: Option<Box<ScopeCtx>>,
}

impl ScopeCtx {
    pub fn init_variable(
        &mut self,
        kind: VariableDeclarationKind,
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
            VariableDeclarationKind::Let => {
                var.value = value;
                Ok(())
            }
            VariableDeclarationKind::Const => {
                Err(VariableIsConstError::new(name.to_owned()).into())
            }
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
    kind: VariableDeclarationKind,
    value: Value,
}
