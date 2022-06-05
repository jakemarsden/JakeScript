use super::error::{Error, Result};
use super::object::UserFunction;
use super::stack::{Variable, VariableKind};
use super::value::Value;
use super::{Eval, Interpreter};
use crate::ast::*;

impl Eval for Declaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Function(node) => node.eval(it),
            Self::Variable(node) => node.eval(it),
            Self::Lexical(node) => node.eval(it),
        }
    }
}

impl Eval for FunctionDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let declared_scope = it.vm().stack().frame().scope().clone();
        let fn_obj_ref = it
            .alloc_function(UserFunction::new(
                None,
                self.formal_parameters.clone(),
                declared_scope,
                self.body.clone(),
            ))
            .map_err(|err| Error::new(err, self.source_location()))?;
        let variable = Variable::new(
            VariableKind::Var,
            self.binding.clone(),
            Value::Object(fn_obj_ref),
        );
        it.vm_mut()
            .stack_mut()
            .frame_mut()
            .scope_mut()
            .declare_variable(variable)
            .map_err(|err| Error::new(err, self.source_location()))?;
        Ok(())
    }
}

impl Eval for VariableDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        for entry in &self.bindings {
            let variable = if let Some(ref initialiser) = entry.initialiser {
                let initial_value = initialiser.eval(it)?;
                Variable::new(VariableKind::Var, entry.identifier.clone(), initial_value)
            } else {
                Variable::new_unassigned(VariableKind::Var, entry.identifier.clone())
            };
            it.vm_mut()
                .stack_mut()
                .frame_mut()
                .scope_mut()
                .ancestor(true)
                .declare_variable(variable)
                .map_err(|err| Error::new(err, self.source_location()))?;
        }
        Ok(())
    }
}

impl Eval for LexicalDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let kind = VariableKind::from(self.kind);
        for entry in &self.bindings {
            let variable = if let Some(ref initialiser) = entry.initialiser {
                let initial_value = initialiser.eval(it)?;
                Variable::new(kind, entry.identifier.clone(), initial_value)
            } else {
                Variable::new_unassigned(kind, entry.identifier.clone())
            };
            it.vm_mut()
                .stack_mut()
                .frame_mut()
                .scope_mut()
                .declare_variable(variable)
                .map_err(|err| Error::new(err, self.source_location()))?;
        }
        Ok(())
    }
}
