use super::error::Result;
use super::heap::Callable;
use super::stack::{Variable, VariableKind};
use super::value::Value;
use super::{Eval, Interpreter};
use crate::ast::*;

impl Eval for Declaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Function(node) => node.eval(it),
            Self::Variable(node) => node.eval(it),
        }
    }
}

impl Eval for FunctionDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let declared_scope = it.vm().stack().frame().scope().clone();
        let callable = Callable::new(
            self.formal_parameters.clone(),
            declared_scope,
            self.body.clone(),
        );
        let fn_obj_ref = it.vm_mut().heap_mut().allocate_callable_object(callable)?;
        let variable = Variable::new(
            VariableKind::Var,
            self.binding.clone(),
            Value::Reference(fn_obj_ref),
        );
        it.vm_mut()
            .stack_mut()
            .frame_mut()
            .scope_mut()
            .declare_variable(variable)?;
        Ok(())
    }
}

impl Eval for VariableDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let kind = VariableKind::from(self.kind);
        for entry in &self.bindings {
            let variable = if let Some(ref initialiser) = entry.initialiser {
                let initial_value = initialiser.eval(it)?;
                Variable::new(kind, entry.identifier.clone(), initial_value)
            } else {
                Variable::new_unassigned(kind, entry.identifier.clone())
            };
            let curr_scope = it.vm_mut().stack_mut().frame_mut().scope_mut();
            let mut declared_scope = if self.is_escalated() {
                curr_scope.ancestor(true)
            } else {
                curr_scope.clone()
            };
            declared_scope.declare_variable(variable)?;
        }
        Ok(())
    }
}
