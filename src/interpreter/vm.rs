use crate::ast::{Value, VariableDeclarationKind};
use crate::interpreter::error::*;
use std::mem;

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

    pub fn scope(&mut self) -> &mut ScopeCtx {
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
    scope: ScopeCtx,
    parent: Option<Box<CallFrame>>,
}

impl CallFrame {
    pub fn scope(&mut self) -> &mut ScopeCtx {
        &mut self.scope
    }

    pub fn push_scope(&mut self) {
        let new_scope = ScopeCtx::default();
        let parent_scope = mem::replace(&mut self.scope, new_scope);
        self.scope.parent = Some(Box::new(parent_scope));
    }

    pub fn pop_scope(&mut self) {
        let parent_scope = self.scope.parent.take();
        self.scope = *parent_scope.expect("Cannot pop top-level scope context");
    }
}

#[derive(Debug, Default)]
pub struct ScopeCtx {
    locals: Vec<Variable>,
    parent: Option<Box<ScopeCtx>>,
}

impl ScopeCtx {
    pub fn init_local(
        &mut self,
        kind: VariableDeclarationKind,
        name: String,
        initial_value: Value,
    ) -> Result<(), VariableAlreadyDefinedError> {
        if matches!(self.lookup_local(&name), Err(VariableNotDefinedError)) {
            self.locals.push(Variable {
                kind,
                name,
                value: initial_value,
            });
            Ok(())
        } else {
            Err(VariableAlreadyDefinedError)
        }
    }

    pub fn lookup_local(&mut self, name: &str) -> Result<&mut Variable, VariableNotDefinedError> {
        if let Some(variable) = self.locals.iter_mut().find(|var| var.name == name) {
            Ok(variable)
        } else if let Some(ref mut parent) = self.parent {
            parent.lookup_local(name)
        } else {
            Err(VariableNotDefinedError)
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    kind: VariableDeclarationKind,
    name: String,
    value: Value,
}

impl Variable {
    pub fn kind(&self) -> VariableDeclarationKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), AssignToConstVariableError> {
        match self.kind {
            VariableDeclarationKind::Let => {
                self.value = value;
                Ok(())
            }
            VariableDeclarationKind::Const => Err(AssignToConstVariableError),
        }
    }
}
