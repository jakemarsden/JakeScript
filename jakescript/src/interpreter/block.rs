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
        for node in self.body() {
            if !matches!(it.vm().execution_state(), ExecutionState::Advance) {
                break;
            }
            if let BlockItem::Declaration(decl) = node {
                assert!(!decl.is_hoisted());
            }
            result = match node {
                BlockItem::Statement(box Statement::Expression(expr)) => expr.eval(it),
                item => item.eval(it).map(|()| Value::default()),
            }?;
        }
        Ok(result)
    }
}

impl Eval for BlockItem {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Declaration(node) => node.eval(it),
            Self::Statement(node) => node.eval(it),
        }
    }
}
