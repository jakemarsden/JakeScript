use crate::ast::*;
use std::assert_matches::assert_matches;
use std::hint::unreachable_unchecked;
use std::ops::Deref;

pub use error::*;
pub use stack::*;
pub use vm::*;

mod error;
mod op;
mod stack;
mod vm;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn vm(&mut self) -> &mut Vm {
        &mut self.vm
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
            if it.vm().execution_state().is_break_or_return() {
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
        if value.is_truthy() {
            Ok(Value::Undefined)
        } else {
            Err(AssertionFailedError::new(value).into())
        }
    }
}

impl Eval for IfStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let condition = self.condition.eval(it)?;
        if condition.is_truthy() {
            it.vm().stack().frame().push_scope();
            self.success_block.eval(it)?;
            it.vm().stack().frame().pop_scope()
        } else if let Some(ref else_block) = self.else_block {
            it.vm().stack().frame().push_scope();
            else_block.eval(it)?;
            it.vm().stack().frame().pop_scope();
        }
        Ok(Value::Undefined)
    }
}

impl Eval for WhileLoop {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        loop {
            let condition = self.condition.eval(it)?;
            if !condition.is_truthy() {
                break;
            }

            it.vm().stack().frame().push_scope();
            self.block.eval(it)?;
            it.vm().stack().frame().pop_scope();

            match it.vm().reset_execution_state() {
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
        it.vm().set_execution_state(ExecutionState::Break);
        Ok(Value::Undefined)
    }
}

impl Eval for ContinueStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        it.vm().set_execution_state(ExecutionState::BreakContinue);
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
        it.vm().set_execution_state(ExecutionState::Return(value));
        Ok(Value::Undefined)
    }
}

impl Eval for FunctionDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let declared_scope = it.vm().stack().frame().scope().clone();
        let function = Function::new(
            self.fn_name.clone(),
            declared_scope,
            self.param_names.clone(),
            self.body.clone(),
        );
        it.vm().stack().frame().scope().declare_function(function)?;
        Ok(Value::Undefined)
    }
}

impl Eval for VariableDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let var_name = self.var_name.to_owned();
        let variable = if let Some(ref initialiser) = self.initialiser {
            let initial_value = initialiser.eval(it)?;
            Variable::new(self.kind, var_name, initial_value)
        } else {
            Variable::new_unassigned(self.kind, var_name)
        };
        it.vm().stack().frame().scope().declare_variable(variable)?;
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
        assert_matches!(self.kind.associativity(), Associativity::RightToLeft);
        let rhs = self.rhs.eval(it)?;
        let mut lhs = match self.lhs.as_ref() {
            Expression::VariableAccess(node) => it
                .vm()
                .stack()
                .frame()
                .scope()
                .lookup_variable(&node.var_name)?,
            expr => todo!("AssignmentExpression::eval: lhs={:#?}", expr),
        };

        let value = match self.kind {
            AssignmentOp::Assign => rhs,
            AssignmentOp::AddAssign => it.add(lhs.value().deref(), &rhs),
            AssignmentOp::SubAssign => it.sub(lhs.value().deref(), &rhs),
            AssignmentOp::MulAssign => it.mul(lhs.value().deref(), &rhs),
            AssignmentOp::DivAssign => it.div(lhs.value().deref(), &rhs),
            AssignmentOp::ModAssign => it.rem(lhs.value().deref(), &rhs),
            AssignmentOp::PowAssign => it.pow(lhs.value().deref(), &rhs),
            kind => todo!("AssignmentExpression::eval: kind={:?}", kind),
        };
        lhs.set_value(value.clone())?;
        Ok(value)
    }
}

impl Eval for BinaryExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        Ok(match self.kind {
            // Get the boolean ops out of the way first, since they don't let us eval the RHS
            //  up-front (which is more ergonomic for all the other ops)
            BinaryOp::LogicalAnd => {
                assert_matches!(self.kind.associativity(), Associativity::LeftToRight);
                Value::Boolean(self.lhs.eval(it)?.is_truthy() && self.rhs.eval(it)?.is_truthy())
            }
            BinaryOp::LogicalOr => {
                assert_matches!(self.kind.associativity(), Associativity::LeftToRight);
                Value::Boolean(self.lhs.eval(it)?.is_truthy() || self.rhs.eval(it)?.is_truthy())
            }

            kind => {
                let (ref lhs, ref rhs) = match kind.associativity() {
                    Associativity::LeftToRight => {
                        let lhs = self.lhs.eval(it)?;
                        let rhs = self.rhs.eval(it)?;
                        (lhs, rhs)
                    }
                    Associativity::RightToLeft => {
                        let rhs = self.rhs.eval(it)?;
                        let lhs = self.lhs.eval(it)?;
                        (lhs, rhs)
                    }
                };
                match kind {
                    // SAFETY: This match arm is unreachable because the possible values are already
                    //  handled by a previous match arm of the outer match expression
                    BinaryOp::LogicalAnd | BinaryOp::LogicalOr => unsafe {
                        unreachable_unchecked()
                    },

                    BinaryOp::Add => it.add(lhs, rhs),
                    BinaryOp::Div => it.div(lhs, rhs),
                    BinaryOp::Mod => it.rem(lhs, rhs),
                    BinaryOp::Mul => it.mul(lhs, rhs),
                    BinaryOp::Pow => it.pow(lhs, rhs),
                    BinaryOp::Sub => it.sub(lhs, rhs),

                    BinaryOp::Equal => it.eq(lhs, rhs),
                    BinaryOp::NotEqual => it.ne(lhs, rhs),
                    BinaryOp::Identical => it.identical(lhs, rhs),
                    BinaryOp::NotIdentical => it.not_identical(lhs, rhs),

                    BinaryOp::LessThan => it.lt(lhs, rhs),
                    BinaryOp::LessThanOrEqual => it.le(lhs, rhs),
                    BinaryOp::MoreThan => it.gt(lhs, rhs),
                    BinaryOp::MoreThanOrEqual => it.ge(lhs, rhs),

                    BinaryOp::ShiftLeft => it.bitwise_shl(lhs, rhs),
                    BinaryOp::ShiftRight => it.bitwise_shr(lhs, rhs),
                    BinaryOp::ShiftRightUnsigned => it.bitwise_shrr(lhs, rhs),

                    BinaryOp::BitwiseAnd => it.bitwise_and(lhs, rhs),
                    BinaryOp::BitwiseOr => it.bitwise_or(lhs, rhs),
                    BinaryOp::BitwiseXOr => it.bitwise_xor(lhs, rhs),
                }
            }
        })
    }
}

impl Eval for UnaryExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let operand = &self.operand.eval(it)?;
        Ok(match self.kind {
            UnaryOp::LogicalNot => it.not(operand),
            UnaryOp::NumericNegate => it.neg(operand),
            UnaryOp::NumericPlus => it.plus(operand),
            kind => todo!("UnaryExpression::eval: kind={:?}", kind),
        })
    }
}

impl Eval for LiteralExpression {
    fn eval(&self, _it: &mut Interpreter) -> Result<Value> {
        Ok(self.value.clone())
    }
}

impl Eval for FunctionCallExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let function = it
            .vm()
            .stack()
            .frame()
            .scope()
            .lookup_function(&self.fn_name)?;
        let parameters = function.declared_parameters();

        if parameters.len() != self.arguments.len() {
            return Err(FunctionArgumentMismatchError.into());
        }
        let mut argument_variables = Vec::with_capacity(parameters.len());
        for idx in 0..parameters.len() {
            let parameter_name = parameters[idx].to_owned();
            let argument_expr = &self.arguments[idx];
            let argument_value = argument_expr.eval(it)?;
            argument_variables.push(Variable::new(
                VariableDeclarationKind::Let,
                parameter_name,
                argument_value,
            ))
        }

        let declared_scope = function.declared_scope().deref().clone();
        let fn_scope_ctx = ScopeCtx::new(argument_variables, Vec::with_capacity(0));
        it.vm().stack().push_frame(declared_scope);
        it.vm().stack().frame().push_scope_ctx(fn_scope_ctx);

        function.body().eval(it)?;

        it.vm().stack().frame().pop_scope();
        it.vm().stack().pop_frame();

        Ok(match it.vm().reset_execution_state() {
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
        let variable = it
            .vm()
            .stack()
            .frame()
            .scope()
            .lookup_variable(&self.var_name)?;
        let value = variable.value().deref().clone();
        Ok(value)
    }
}
