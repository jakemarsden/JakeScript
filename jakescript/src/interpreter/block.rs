use super::error::Result;
use super::value::Value;
use super::vm::ExecutionState;
use super::{Eval, Interpreter};
use crate::ast::*;

impl Eval for Script {
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
        for node in self.body() {
            if let Statement::Declaration(decl) = node {
                assert!(!decl.is_hoisted());
            }
            if !matches!(it.vm().execution_state(), ExecutionState::Advance) {
                break;
            }
            result = match node {
                Statement::Expression(expr) => expr.expression.eval(it),
                node => node.eval(it).map(|()| Value::default()),
            }?;
        }
        Ok(result)
    }
}
