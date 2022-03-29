use super::error::Result;
use super::value::Value;
use super::vm::ExecutionState;
use super::{Eval, Interpreter};
use crate::ast::*;

impl Eval for Program {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.body().eval(it)
    }
}

impl Eval for Block {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let mut result = Value::default();
        for decl in self.hoisted_declarations() {
            if !matches!(it.vm().execution_state(), ExecutionState::Advance) {
                break;
            }
            assert!(decl.is_hoisted());
            decl.eval(it)?;
        }
        for stmt in self.statements() {
            if !matches!(it.vm().execution_state(), ExecutionState::Advance) {
                break;
            }
            if let Statement::Declaration(decl) = stmt {
                assert!(!decl.is_hoisted());
            }
            result = match stmt {
                Statement::Expression(expr) => expr.eval(it),
                stmt => stmt.eval(it).map(|()| Value::default()),
            }?;
        }
        Ok(result)
    }
}
