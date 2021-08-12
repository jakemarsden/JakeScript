use crate::ast::*;
use std::cmp::Ordering;
use std::mem;

pub use error::*;
use std::ops::Deref;
pub use vm::*;

mod error;
mod vm;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Interpreter {
    vm: Vm,
    execution_state: ExecutionState,
}

impl Interpreter {
    pub fn add(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() + rhs.as_numeric())
    }

    pub fn sub(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() - rhs.as_numeric())
    }

    pub fn mul(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() * rhs.as_numeric())
    }

    pub fn r#mod(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() % rhs.as_numeric())
    }

    pub fn div(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() / rhs.as_numeric())
    }

    pub fn pow(&self, lhs: Value, rhs: Value) -> Value {
        let lhs = lhs.as_numeric();
        let rhs = rhs.as_numeric();
        assert!(rhs >= i32::MIN as i64 && rhs <= i32::MAX as i64);
        let result = (lhs as f64).powi(rhs as i32);
        Value::Numeric(result as i64)
    }

    pub fn compare(&self, lhs: Value, rhs: Value) -> Ordering {
        lhs.as_numeric().cmp(&rhs.as_numeric())
    }

    pub fn is_identical(&self, lhs: Value, rhs: Value) -> Value {
        Value::Boolean(match (lhs, rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => lhs == rhs,
            (Value::Null, Value::Null) => true,
            (Value::Numeric(lhs), Value::Numeric(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::Undefined, Value::Undefined) => true,
            (_, _) => false,
        })
    }

    pub fn vm(&mut self) -> &mut Vm {
        &mut self.vm
    }

    pub fn execution_state(&self) -> &ExecutionState {
        &self.execution_state
    }

    pub fn take_execution_state(&mut self) -> ExecutionState {
        mem::take(&mut self.execution_state)
    }

    pub fn set_execution_state(&mut self, execution_state: ExecutionState) {
        if matches!(self.execution_state, ExecutionState::Advance) {
            self.execution_state = execution_state;
        } else {
            panic!(
                "Unexpected execution state (expected {:?} but was {:?}): Cannot set to {:?}",
                ExecutionState::Advance,
                self.execution_state,
                execution_state
            );
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExecutionState {
    Advance,
    Break,
    BreakContinue,
    Return(Value),
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self::Advance
    }
}

pub trait Eval: Node {
    fn eval(&self, it: &mut Interpreter) -> Result<Value>;
}

impl Eval for Program {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        self.block().eval(it)
    }
}

impl Eval for Block {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let mut result = Value::Undefined;
        for stmt in self.iter() {
            if !matches!(it.execution_state(), ExecutionState::Advance) {
                break;
            }
            result = stmt.eval(it)?;
        }
        Ok(result)
    }
}

impl Eval for Statement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            Self::Assertion(node) => node.eval(it),
            Self::Block(node) => node.eval(it),
            Self::Break(node) => node.eval(it),
            Self::Continue(node) => node.eval(it),
            Self::Expression(node) => node.eval(it),
            Self::FunctionDeclaration(node) => node.eval(it),
            Self::IfStatement(node) => node.eval(it),
            Self::Return(node) => node.eval(it),
            Self::VariableDeclaration(node) => node.eval(it),
            Self::WhileLoop(node) => node.eval(it),
        }
    }
}

impl Eval for Assertion {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let value = self.condition.eval(it)?;
        if value.as_boolean() {
            Ok(Value::Undefined)
        } else {
            Err(AssertionFailedError::new(value).into())
        }
    }
}

impl Eval for IfStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let condition = self.condition.eval(it)?;
        if condition.as_boolean() {
            it.vm().frame().push_scope();
            self.success_block.eval(it)?;
            it.vm.frame().pop_scope()
        } else if let Some(ref else_block) = self.else_block {
            it.vm().frame().push_scope();
            else_block.eval(it)?;
            it.vm().frame().pop_scope();
        }
        Ok(Value::Undefined)
    }
}

impl Eval for WhileLoop {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        loop {
            let condition = self.condition.eval(it)?;
            if !condition.as_boolean() {
                break;
            }

            it.vm().frame().push_scope();
            self.block.eval(it)?;
            it.vm().frame().pop_scope();

            match it.take_execution_state() {
                ExecutionState::Advance => {}
                ExecutionState::Break => break,
                ExecutionState::BreakContinue => continue,
                execution_state => panic!("Unexpected execution state: {:?}", execution_state),
            }
        }
        Ok(Value::Undefined)
    }
}

impl Eval for BreakStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        it.set_execution_state(ExecutionState::Break);
        Ok(Value::Undefined)
    }
}

impl Eval for ContinueStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        it.set_execution_state(ExecutionState::BreakContinue);
        Ok(Value::Undefined)
    }
}

impl Eval for ReturnStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let value = if let Some(ref expr) = self.expr {
            expr.eval(it)?
        } else {
            Value::Undefined
        };
        it.set_execution_state(ExecutionState::Return(value));
        Ok(Value::Undefined)
    }
}

impl Eval for FunctionDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        it.vm().scope().declare_function(
            self.fn_name.clone(),
            self.param_names.clone(),
            self.body.clone(),
        )?;
        Ok(Value::Undefined)
    }
}

impl Eval for VariableDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let initial_value = if let Some(ref initialiser) = self.initialiser {
            initialiser.eval(it)?
        } else {
            Value::Undefined
        };
        it.vm()
            .scope()
            .declare_variable(self.kind, self.var_name.to_owned(), initial_value)?;
        Ok(Value::Undefined)
    }
}

impl Eval for Expression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            Self::Assignment(ref node) => node.eval(it),
            Self::Binary(ref node) => node.eval(it),
            Self::Unary(ref node) => node.eval(it),

            Self::Literal(ref node) => node.eval(it),
            Self::FunctionCall(ref node) => node.eval(it),
            Self::PropertyAccess(ref node) => node.eval(it),
            Self::VariableAccess(ref node) => node.eval(it),
        }
    }
}

impl Eval for AssignmentExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let var_name = match self.lhs.as_ref() {
            Expression::VariableAccess(node) => &node.var_name,
            lhs => todo!("Expression::eval: assignment_op: lhs={:#?}", lhs),
        };
        let lhs = it.vm().scope().lookup_variable(var_name)?.value().clone();
        let rhs = self.rhs.eval(it)?;
        let value = match self.kind {
            AssignmentOp::Assign => rhs,
            AssignmentOp::AddAssign => it.add(lhs, rhs),
            AssignmentOp::SubAssign => it.sub(lhs, rhs),
            AssignmentOp::MulAssign => it.mul(lhs, rhs),
            AssignmentOp::DivAssign => it.div(lhs, rhs),
            AssignmentOp::ModAssign => it.r#mod(lhs, rhs),
            AssignmentOp::PowAssign => it.pow(lhs, rhs),
            kind => todo!("Expression::eval: kind={:?}", kind),
        };
        it.vm()
            .scope()
            .lookup_variable(var_name)?
            .set_value(value.clone())?;
        Ok(value)
    }
}

impl Eval for BinaryExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let lhs = self.lhs.eval(it)?;
        let rhs = self.rhs.eval(it)?;
        Ok(match self.kind {
            BinaryOp::Add => it.add(lhs, rhs),
            BinaryOp::Sub => it.sub(lhs, rhs),
            BinaryOp::Mul => it.mul(lhs, rhs),
            BinaryOp::Div => it.div(lhs, rhs),
            BinaryOp::Mod => it.r#mod(lhs, rhs),
            BinaryOp::Pow => it.pow(lhs, rhs),
            BinaryOp::Identical => it.is_identical(lhs, rhs),
            BinaryOp::LessThan => Value::Boolean(it.compare(lhs, rhs).is_lt()),
            BinaryOp::LessThanOrEqual => Value::Boolean(it.compare(lhs, rhs).is_le()),
            BinaryOp::LogicalAnd => Value::Boolean(lhs.as_boolean() && rhs.as_boolean()),
            BinaryOp::LogicalOr => Value::Boolean(lhs.as_boolean() || rhs.as_boolean()),
            BinaryOp::MoreThan => Value::Boolean(it.compare(lhs, rhs).is_gt()),
            BinaryOp::MoreThanOrEqual => Value::Boolean(it.compare(lhs, rhs).is_ge()),
            BinaryOp::NotIdentical => !it.is_identical(lhs, rhs),
            kind => todo!("Expression::eval: kind={:?}", kind),
        })
    }
}

impl Eval for UnaryExpression {
    fn eval(&self, _it: &mut Interpreter) -> Result<Value> {
        todo!("UnaryExpression::eval: {:#?}", self)
    }
}

impl Eval for LiteralExpression {
    fn eval(&self, _it: &mut Interpreter) -> Result<Value> {
        Ok(self.value.clone())
    }
}

impl Eval for FunctionCallExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let function = it.vm().scope().lookup_function(&self.fn_name)?;
        if function.parameters().len() != self.arguments.len() {
            return Err(FunctionArgumentMismatchError.into());
        }

        it.vm().stack().push_frame();

        // Add function arguments to the scope of the newly-created frame
        for (idx, argument) in self.arguments.iter().enumerate() {
            let parameter_name = function.parameters()[idx].clone();
            let argument_value = argument.eval(it)?;
            it.vm().scope().declare_variable(
                VariableDeclarationKind::Let,
                parameter_name,
                argument_value,
            )?;
        }
        function.body().eval(it)?;

        it.vm.stack().pop_frame();

        Ok(match it.take_execution_state() {
            ExecutionState::Advance => Value::Undefined,
            ExecutionState::Return(value) => value,
            execution_state => panic!("Unexpected execution state: {:?}", execution_state),
        })
    }
}

impl Eval for PropertyAccessExpression {
    fn eval(&self, _it: &mut Interpreter) -> Result<Value> {
        todo!("PropertyExpression::eval: {:#?}", self)
    }
}

impl Eval for VariableAccessExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let variable = it.vm().scope().lookup_variable(&self.var_name)?;
        let value = variable.value().deref().clone();
        Ok(value)
    }
}
