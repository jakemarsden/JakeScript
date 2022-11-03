use super::error::{Error, Result};
use super::stack::{Variable, VariableKind};
use super::value::Value;
use super::vm::{ExecutionState, IterationDecision};
use super::{Eval, Interpreter};
use crate::ast::*;

impl Eval for Statement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Declaration(node) => node.eval(it),
            Self::Expression(node) => node.eval(it).map(|_| ()),

            Self::Empty(node) => node.eval(it),
            Self::Block(node) => node.eval(it),

            Self::If(node) => node.eval(it),
            Self::Switch(node) => node.eval(it),
            Self::Try(node) => node.eval(it),

            Self::Do(node) => node.eval(it),
            Self::For(node) => node.eval(it),
            Self::While(node) => node.eval(it),

            Self::Break(node) => node.eval(it),
            Self::Continue(node) => node.eval(it),
            Self::Return(node) => node.eval(it),
            Self::Throw(node) => node.eval(it),
        }
    }
}

impl Eval for EmptyStatement {
    fn eval(&self, _: &mut Interpreter) -> Result<Self::Output> {
        Ok(())
    }
}

impl Eval for BlockStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.block.eval(it).map(|_| ())
    }
}

impl Eval for IfStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let condition = self.condition.eval(it)?;
        if it.is_truthy(condition) {
            it.vm_mut()
                .stack_mut()
                .push_empty_scope(false)
                .map_err(|err| Error::new(err, self.source_location()))?;
            self.body.eval(it)?;
            it.vm_mut().stack_mut().pop_scope();
        } else if let Some(ref else_block) = self.else_body {
            it.vm_mut()
                .stack_mut()
                .push_empty_scope(false)
                .map_err(|err| Error::new(err, self.source_location()))?;
            else_block.eval(it)?;
            it.vm_mut().stack_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for SwitchStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let value = self.value.eval(it)?;
        let mut cases = self.cases.iter().peekable();

        // Skip cases while `actual != expected`.
        while let Some(case) = cases.peek() {
            let expected = case.pattern.eval(it)?;
            if it.equal(expected, value) {
                break;
            }
            cases.next().unwrap();
        }
        // Evaluate remaining cases in turn (may do nothing if any of the cases change
        // the execution state).
        for case in cases {
            case.eval(it)?;
        }
        if let Some(ref case) = self.default_case {
            case.eval(it)?;
        }

        match it.vm().execution_state() {
            ExecutionState::Break => {
                it.vm_mut().reset_execution_state();
            }
            ExecutionState::Advance
            | ExecutionState::Continue
            | ExecutionState::Return(_)
            | ExecutionState::Exception(_)
            | ExecutionState::Exit => {
                // Don't reset the execution state just yet so that it can be
                // handled/cleared by some calling AST node.
            }
        }
        Ok(())
    }
}

impl Eval for CaseStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        for stmt in &self.body {
            if !matches!(it.vm().execution_state(), ExecutionState::Advance) {
                break;
            }
            stmt.eval(it)?;
        }
        Ok(())
    }
}

impl Eval for DefaultCaseStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        for stmt in &self.body {
            if !matches!(it.vm().execution_state(), ExecutionState::Advance) {
                break;
            }
            stmt.eval(it)?;
        }
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

impl Eval for CatchStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let exception = it.vm_mut().clear_exception().unwrap();
        if let Some(ref exception_var_name) = self.exception_binding {
            let exception_var =
                Variable::new(VariableKind::Let, exception_var_name.clone(), exception);
            it.vm_mut()
                .stack_mut()
                .push_scope(false, vec![exception_var])
                .map_err(|err| Error::new(err, self.source_location()))?;
        }
        self.body.eval(it)?;
        if self.exception_binding.is_some() {
            it.vm_mut().stack_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for FinallyStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut().hide_current_exception();
        let result = self.body.eval(it).map(|_| ());
        it.vm_mut().restore_hidden_exception();
        result
    }
}

impl Eval for DoStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        loop {
            it.vm_mut()
                .stack_mut()
                .push_empty_scope(false)
                .map_err(|err| Error::new(err, self.source_location()))?;
            self.body.eval(it)?;
            it.vm_mut().stack_mut().pop_scope();

            match it.vm_mut().handle_loop_execution_state() {
                IterationDecision::Advance => {}
                IterationDecision::Break => break,
                IterationDecision::Continue => continue,
            }

            let condition = self.condition.eval(it)?;
            if !it.is_truthy(condition) {
                break;
            }
        }
        Ok(())
    }
}

impl Eval for ForStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        if let Some(ref initialiser) = self.initialiser {
            it.vm_mut()
                .stack_mut()
                .push_empty_scope(false)
                .map_err(|err| Error::new(err, self.source_location()))?;
            initialiser.eval(it)?;
        }
        loop {
            if let Some(ref condition) = self.condition {
                let condition = condition.eval(it)?;
                if !it.is_truthy(condition) {
                    break;
                }
            }

            it.vm_mut()
                .stack_mut()
                .push_empty_scope(false)
                .map_err(|err| Error::new(err, self.source_location()))?;
            self.body.eval(it)?;
            it.vm_mut().stack_mut().pop_scope();

            if let Some(ref incrementor) = self.incrementor {
                incrementor.eval(it)?;
            }

            match it.vm_mut().handle_loop_execution_state() {
                IterationDecision::Advance => {}
                IterationDecision::Break => break,
                IterationDecision::Continue => continue,
            }
        }
        if self.initialiser.is_some() {
            it.vm_mut().stack_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for ForInitialiser {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Expression(node) => node.eval(it).map(|_| ()),
            Self::VariableDeclaration(node) => node.eval(it),
            Self::LexicalDeclaration(node) => node.eval(it),
        }
    }
}

impl Eval for WhileStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        loop {
            let condition = self.condition.eval(it)?;
            if !it.is_truthy(condition) {
                break;
            }

            it.vm_mut()
                .stack_mut()
                .push_empty_scope(false)
                .map_err(|err| Error::new(err, self.source_location()))?;
            self.body.eval(it)?;
            it.vm_mut().stack_mut().pop_scope();

            match it.vm_mut().handle_loop_execution_state() {
                IterationDecision::Advance => {}
                IterationDecision::Break => break,
                IterationDecision::Continue => continue,
            }
        }
        Ok(())
    }
}

impl Eval for BreakStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut().set_execution_state(ExecutionState::Break);
        Ok(())
    }
}

impl Eval for ContinueStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut().set_execution_state(ExecutionState::Continue);
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
