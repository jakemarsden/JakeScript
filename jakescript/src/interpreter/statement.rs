use super::error::Result;
use super::stack::{ScopeCtx, Variable, VariableKind};
use super::value::Value;
use super::vm::ExecutionState;
use super::{Eval, Interpreter};
use crate::ast::*;

impl Eval for Statement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Expression(node) => node.eval(it),
            Self::If(node) => node.eval(it),
            Self::WhileLoop(node) => node.eval(it),
            Self::ForLoop(node) => node.eval(it),
            Self::Continue(node) => node.eval(it),
            Self::Break(node) => node.eval(it),
            Self::Return(node) => node.eval(it),
            Self::Throw(node) => node.eval(it),
            Self::Try(node) => node.eval(it),
        }
    }
}

impl Eval for ExpressionStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.expression.eval(it).map(|_| ())
    }
}

impl Eval for IfStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let condition = self.condition.eval(it)?;
        if it.is_truthy(&condition) {
            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            self.body.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        } else if let Some(ref else_block) = self.else_body {
            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            else_block.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for WhileStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        loop {
            let condition = self.condition.eval(it)?;
            if !it.is_truthy(&condition) {
                break;
            }

            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            self.body.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();

            match it.vm().execution_state() {
                ExecutionState::Advance => {}
                ExecutionState::Break => {
                    it.vm_mut().reset_execution_state();
                    break;
                }
                ExecutionState::BreakContinue => {
                    it.vm_mut().reset_execution_state();
                    continue;
                }
                ExecutionState::Return(_) | ExecutionState::Exception(_) | ExecutionState::Exit => {
                    // Exit the loop, but don't reset the execution state just yet so that it can be
                    // handled/cleared by some calling AST node.
                    break;
                }
            }
        }
        Ok(())
    }
}

impl Eval for ForStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        if let Some(ref initialiser) = self.initialiser {
            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            initialiser.eval(it)?;
        }
        loop {
            if let Some(ref condition) = self.condition {
                let condition = condition.eval(it)?;
                if !it.is_truthy(&condition) {
                    break;
                }
            }

            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            self.body.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();

            if let Some(ref incrementor) = self.incrementor {
                incrementor.eval(it)?;
            }

            match it.vm().execution_state() {
                ExecutionState::Advance => {}
                ExecutionState::Break => {
                    it.vm_mut().reset_execution_state();
                    break;
                }
                ExecutionState::BreakContinue => {
                    it.vm_mut().reset_execution_state();
                    continue;
                }
                ExecutionState::Return(_) | ExecutionState::Exception(_) | ExecutionState::Exit => {
                    // Exit the loop, but don't reset the execution state just yet so that it can be
                    // handled/cleared by some calling AST node.
                    break;
                }
            }
        }
        if self.initialiser.is_some() {
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for ContinueStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut()
            .set_execution_state(ExecutionState::BreakContinue);
        Ok(())
    }
}

impl Eval for BreakStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut().set_execution_state(ExecutionState::Break);
        Ok(())
    }
}

impl Eval for ReturnStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let value = if let Some(ref expr) = self.value {
            expr.eval(it)?
        } else {
            Value::Undefined
        };
        it.vm_mut()
            .set_execution_state(ExecutionState::Return(value));
        Ok(())
    }
}

impl Eval for ThrowStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let ex = self.exception.eval(it)?;
        it.vm_mut()
            .set_execution_state(ExecutionState::Exception(ex));
        Ok(())
    }
}

impl Eval for TryStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.body.eval(it)?;
        if let Some(ref catch) = self.catch {
            if matches!(it.vm().execution_state(), ExecutionState::Exception(..)) {
                catch.eval(it)?;
            }
        }
        if let Some(ref finally) = self.finally {
            finally.eval(it)?;
        }
        Ok(())
    }
}

impl Eval for Catch {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let exception = it.vm_mut().clear_exception().unwrap();
        if let Some(ref exception_var_name) = self.parameter {
            let exception_var =
                Variable::new(VariableKind::Let, exception_var_name.clone(), exception);
            let scope_ctx = ScopeCtx::new(vec![exception_var]);
            it.vm_mut()
                .stack_mut()
                .frame_mut()
                .push_scope(scope_ctx, false);
        }
        self.body.eval(it)?;
        if self.parameter.is_some() {
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for Finally {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut().hide_current_exception();
        let result = self.body.eval(it).map(|_| ());
        it.vm_mut().restore_hidden_exception();
        result
    }
}
