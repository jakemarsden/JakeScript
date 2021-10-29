use crate::ast::*;
use std::assert_matches::assert_matches;
use std::hint::unreachable_unchecked;
use std::ops::Deref;

pub use error::*;
pub use heap::*;
pub use stack::*;
pub use value::*;
pub use vm::*;

mod error;
mod heap;
mod stack;
mod value;
mod vm;

pub type Result<T = Value> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn vm(&self) -> &Vm {
        &self.vm
    }
    pub fn vm_mut(&mut self) -> &mut Vm {
        &mut self.vm
    }
}

pub trait Eval: Node {
    type Output = ();

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output>;
}

impl Eval for Program {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let vm_constant_pool = it.vm_mut().constant_pool_mut();
        for (constant_id, constant_value) in self.constants() {
            let allocated_id = vm_constant_pool.allocate(constant_value.to_owned());
            // TODO: Make this less fragile?
            assert_eq!(allocated_id, *constant_id);
        }
        self.body().eval(it)
    }
}

impl Eval for Block {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let mut result = Value::default();
        for stmt in self.statements() {
            if it.vm().execution_state().is_break_or_return() {
                break;
            }
            result = match stmt {
                Statement::Expression(expr) => expr.eval(it),
                stmt => stmt.eval(it).map(|()| Value::default()),
            }?;
        }
        Ok(result)
    }
}

impl Eval for Statement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Assertion(node) => node.eval(it),
            Self::Break(node) => node.eval(it),
            Self::Continue(node) => node.eval(it),
            Self::Expression(node) => node.eval(it).map(|_| ()),
            Self::FunctionDeclaration(node) => node.eval(it),
            Self::IfStatement(node) => node.eval(it),
            Self::Print(node) => node.eval(it),
            Self::Return(node) => node.eval(it),
            Self::VariableDeclaration(node) => node.eval(it),
            Self::ForLoop(node) => node.eval(it),
            Self::WhileLoop(node) => node.eval(it),
        }
    }
}

impl Eval for Assertion {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let value = self.condition.eval(it)?;
        if value.is_truthy(it) {
            Ok(())
        } else {
            Err(AssertionFailedError::new(self.condition.clone(), value).into())
        }
    }
}

impl Eval for PrintStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let value = self.argument.eval(it)?;
        if let Value::String(ref string_value) = value.coerce_to_string(it) {
            if self.new_line {
                println!("{}", string_value);
            } else {
                print!("{}", string_value);
            }
        } else {
            unreachable!();
        }
        Ok(())
    }
}

impl Eval for IfStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let condition = self.condition.eval(it)?;
        if condition.is_truthy(it) {
            it.vm_mut().stack_mut().frame_mut().push_scope();
            self.success_block.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope()
        } else if let Some(ref else_block) = self.else_block {
            it.vm_mut().stack_mut().frame_mut().push_scope();
            else_block.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for ForLoop {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        if let Some(ref initialiser) = self.initialiser {
            it.vm_mut().stack_mut().frame_mut().push_scope();
            initialiser.eval(it)?;
        }
        loop {
            if let Some(ref condition) = self.condition {
                let condition = condition.eval(it)?;
                if condition.is_falsy(it) {
                    break;
                }
            }

            it.vm_mut().stack_mut().frame_mut().push_scope();
            self.block.eval(it)?;
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
                ExecutionState::Return(_) => {
                    // Exit the loop, but don't reset the execution state just yet so that it can be
                    // handled by an outer `FunctionCallExpression`.
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

impl Eval for WhileLoop {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        loop {
            let condition = self.condition.eval(it)?;
            if condition.is_falsy(it) {
                break;
            }

            it.vm_mut().stack_mut().frame_mut().push_scope();
            self.block.eval(it)?;
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
                ExecutionState::Return(_) => {
                    // Exit the loop, but don't reset the execution state just yet so that it can be
                    // handled by an outer `FunctionCallExpression`.
                    break;
                }
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
        it.vm_mut()
            .set_execution_state(ExecutionState::BreakContinue);
        Ok(())
    }
}

impl Eval for ReturnStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let value = if let Some(ref expr) = self.expr {
            expr.eval(it)?
        } else {
            Value::Undefined
        };
        it.vm_mut()
            .set_execution_state(ExecutionState::Return(value));
        Ok(())
    }
}

impl Eval for FunctionDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let declared_scope = it.vm().stack().frame().scope().clone();
        let callable = Callable::new(self.param_names.clone(), declared_scope, self.body.clone());
        let fn_obj_ref = it.vm_mut().heap_mut().allocate_callable_object(callable)?;
        let variable = Variable::new(
            VariableDeclarationKind::Var,
            self.fn_name,
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
        let var_name = self.var_name.to_owned();
        let variable = if let Some(ref initialiser) = self.initialiser {
            let initial_value = initialiser.eval(it)?;
            Variable::new(self.kind, var_name, initial_value)
        } else {
            Variable::new_unassigned(self.kind, var_name)
        };
        it.vm_mut()
            .stack_mut()
            .frame_mut()
            .scope_mut()
            .declare_variable(variable)?;
        Ok(())
    }
}

impl Eval for Expression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Assignment(ref node) => node.eval(it),
            Self::Binary(ref node) => node.eval(it),
            Self::Unary(ref node) => node.eval(it),
            Self::Ternary(ref node) => node.eval(it),
            Self::Grouping(ref node) => node.eval(it),

            Self::Literal(ref node) => node.eval(it),
            Self::FunctionCall(ref node) => node.eval(it),
            Self::PropertyAccess(ref node) => node.eval(it),
            Self::VariableAccess(ref node) => node.eval(it),
        }
    }
}

impl Eval for AssignmentExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        fn compute_new_value(
            self_: &AssignmentExpression,
            it: &mut Interpreter,
            getter: impl FnOnce() -> Result,
        ) -> Result {
            let rhs = self_.rhs.eval(it)?;
            Ok(match self_.kind {
                AssignmentOp::Assign => rhs,
                AssignmentOp::AddAssign => Value::add(it, &getter()?, &rhs),
                AssignmentOp::SubAssign => Value::sub(it, &getter()?, &rhs),
                AssignmentOp::MulAssign => Value::mul(it, &getter()?, &rhs),
                AssignmentOp::DivAssign => Value::div(it, &getter()?, &rhs),
                AssignmentOp::ModAssign => Value::rem(it, &getter()?, &rhs),
                AssignmentOp::PowAssign => Value::pow(it, &getter()?, &rhs),
                kind => todo!("AssignmentExpression::eval: kind={:?}", kind),
            })
        }

        assert_matches!(self.kind.associativity(), Associativity::RightToLeft);
        match self.lhs.as_ref() {
            Expression::VariableAccess(node) => {
                let mut variable = it
                    .vm()
                    .stack()
                    .frame()
                    .scope()
                    .lookup_variable(node.var_name)?;
                let new_value = compute_new_value(self, it, || Ok(variable.value().clone()))?;
                variable.set_value(new_value.clone())?;
                Ok(new_value)
            }
            Expression::PropertyAccess(node) => {
                let base_value = node.base.eval(it)?;
                let mut base_obj = match base_value {
                    Value::Reference(ref base_refr) => {
                        it.vm_mut().heap_mut().resolve_mut(base_refr)
                    }
                    base_value => todo!("AssignmentExpression::eval: base_value={:?}", base_value),
                };
                let property_name = it
                    .vm()
                    .constant_pool()
                    .lookup(node.property_name)
                    .to_owned();
                let new_value = compute_new_value(self, it, || {
                    Ok(base_obj
                        .property(&property_name)
                        .cloned()
                        .unwrap_or_default())
                })?;
                base_obj.set_property(property_name, new_value.clone());
                Ok(new_value)
            }
            expr => todo!("AssignmentExpression::eval: lhs={:#?}", expr),
        }
    }
}

impl Eval for BinaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        Ok(match self.kind {
            // Get the boolean ops out of the way first, since they don't let us eval the RHS
            // up-front (which is more ergonomic for all the other ops).
            BinaryOp::LogicalAnd => {
                assert_matches!(self.kind.associativity(), Associativity::LeftToRight);
                Value::Boolean(self.lhs.eval(it)?.is_truthy(it) && self.rhs.eval(it)?.is_truthy(it))
            }
            BinaryOp::LogicalOr => {
                assert_matches!(self.kind.associativity(), Associativity::LeftToRight);
                Value::Boolean(self.lhs.eval(it)?.is_truthy(it) || self.rhs.eval(it)?.is_truthy(it))
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
                    // Safety: Unreachable because the possible values are already handled by
                    // previous match arms in the outer match expression.
                    BinaryOp::LogicalAnd | BinaryOp::LogicalOr => unsafe {
                        unreachable_unchecked()
                    },

                    BinaryOp::Add => Value::add(it, lhs, rhs),
                    BinaryOp::Div => Value::div(it, lhs, rhs),
                    BinaryOp::Mod => Value::rem(it, lhs, rhs),
                    BinaryOp::Mul => Value::mul(it, lhs, rhs),
                    BinaryOp::Pow => Value::pow(it, lhs, rhs),
                    BinaryOp::Sub => Value::sub(it, lhs, rhs),

                    BinaryOp::Equal => Value::eq(it, lhs, rhs),
                    BinaryOp::NotEqual => Value::ne(it, lhs, rhs),
                    BinaryOp::Identical => Value::identical(it, lhs, rhs),
                    BinaryOp::NotIdentical => Value::not_identical(it, lhs, rhs),

                    BinaryOp::LessThan => Value::lt(it, lhs, rhs),
                    BinaryOp::LessThanOrEqual => Value::le(it, lhs, rhs),
                    BinaryOp::MoreThan => Value::gt(it, lhs, rhs),
                    BinaryOp::MoreThanOrEqual => Value::ge(it, lhs, rhs),

                    BinaryOp::ShiftLeft => Value::bitwise_shl(it, lhs, rhs),
                    BinaryOp::ShiftRight => Value::bitwise_shr(it, lhs, rhs),
                    BinaryOp::ShiftRightUnsigned => Value::bitwise_shrr(it, lhs, rhs),

                    BinaryOp::BitwiseAnd => Value::bitwise_and(it, lhs, rhs),
                    BinaryOp::BitwiseOr => Value::bitwise_or(it, lhs, rhs),
                    BinaryOp::BitwiseXOr => Value::bitwise_xor(it, lhs, rhs),
                }
            }
        })
    }
}

impl Eval for UnaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let operand = &self.operand.eval(it)?;
        Ok(match self.kind {
            UnaryOp::IncrementPrefix
            | UnaryOp::IncrementPostfix
            | UnaryOp::DecrementPrefix
            | UnaryOp::DecrementPostfix => {
                fn compute(
                    self_: &UnaryExpression,
                    it: &mut Interpreter,
                    getter: impl FnOnce() -> Result,
                ) -> Result<(Value, Value)> {
                    const ONE: Value = Value::Number(1);
                    let old_value = getter()?;

                    // The new value to assign to the variable or property
                    let new_value = match self_.kind {
                        UnaryOp::IncrementPrefix | UnaryOp::IncrementPostfix => {
                            Value::add(it, &old_value, &ONE)
                        }
                        UnaryOp::DecrementPrefix | UnaryOp::DecrementPostfix => {
                            Value::sub(it, &old_value, &ONE)
                        }
                        _ => unreachable!("{:?}", self_.kind),
                    };
                    // The value to use as the result of the expression
                    let result_value = match self_.kind {
                        UnaryOp::IncrementPrefix | UnaryOp::DecrementPrefix => new_value.clone(),
                        UnaryOp::IncrementPostfix | UnaryOp::DecrementPostfix => old_value,
                        _ => unreachable!("{:?}", self_.kind),
                    };
                    Ok((new_value, result_value))
                }

                assert_matches!(self.kind.associativity(), Associativity::RightToLeft);
                match self.operand.as_ref() {
                    Expression::VariableAccess(node) => {
                        let mut variable = it
                            .vm()
                            .stack()
                            .frame()
                            .scope()
                            .lookup_variable(node.var_name)?;
                        let (new_value, result_value) =
                            compute(self, it, || Ok(variable.value().clone()))?;
                        variable.set_value(new_value)?;
                        result_value
                    }
                    Expression::PropertyAccess(node) => {
                        let base_value = node.base.eval(it)?;
                        let mut base_obj = match base_value {
                            Value::Reference(ref base_refr) => {
                                it.vm_mut().heap_mut().resolve_mut(base_refr)
                            }
                            base_value => {
                                todo!("AssignmentExpression::eval: base_value={:?}", base_value)
                            }
                        };
                        let property_name = it
                            .vm()
                            .constant_pool()
                            .lookup(node.property_name)
                            .to_owned();
                        let (new_value, result_value) = compute(self, it, || {
                            Ok(base_obj
                                .property(&property_name)
                                .cloned()
                                .unwrap_or_default())
                        })?;
                        base_obj.set_property(property_name, new_value);
                        result_value
                    }
                    _ => todo!("UnaryExpression::eval: self={:#?}", self),
                }
            }

            UnaryOp::BitwiseNot => Value::bitwise_not(it, operand),
            UnaryOp::LogicalNot => Value::not(it, operand),
            UnaryOp::NumericNegate => Value::neg(it, operand),
            UnaryOp::NumericPlus => Value::plus(it, operand),
        })
    }
}

impl Eval for TernaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let condition = self.condition.eval(it)?;
        if condition.is_truthy(it) {
            self.lhs.eval(it)
        } else {
            self.rhs.eval(it)
        }
    }
}

impl Eval for GroupingExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.inner.eval(it)
    }
}

impl Eval for LiteralExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        Ok(match self.value {
            Literal::Boolean(ref value) => Value::Boolean(*value),
            Literal::Numeric(ref value) => Value::Number(*value),
            Literal::String(ref value) => Value::String(value.to_owned()),
            Literal::Object => {
                let obj_ref = it.vm_mut().heap_mut().allocate_empty_object()?;
                Value::Reference(obj_ref)
            }
            Literal::AnonFunction {
                ref param_names,
                ref body,
            } => {
                let declared_scope = it.vm().stack().frame().scope().clone();
                let fn_obj_ref = it
                    .vm_mut()
                    .heap_mut()
                    .allocate_callable_object(Callable::new(
                        param_names.clone(),
                        declared_scope,
                        body.clone(),
                    ))?;
                Value::Reference(fn_obj_ref)
            }
            Literal::Null => Value::Null,
            Literal::Undefined => Value::Undefined,
        })
    }
}

impl Eval for FunctionCallExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let fn_obj_ref = if let Value::Reference(fn_obj_ref) = self.function.eval(it)? {
            fn_obj_ref
        } else {
            return Err(Error::NotCallable(NotCallableError));
        };
        let fn_obj = it.vm().heap().resolve(&fn_obj_ref);
        let function = if let Some(callable) = fn_obj.callable() {
            callable
        } else {
            return Err(Error::NotCallable(NotCallableError));
        };

        let parameters = function.declared_parameters();
        if parameters.len() != self.arguments.len() {
            return Err(FunctionArgumentMismatchError.into());
        }
        let mut argument_variables = Vec::with_capacity(parameters.len());
        for (idx, parameter_name) in parameters.iter().enumerate() {
            let parameter_name = *parameter_name;
            let argument_expr = &self.arguments[idx];
            let argument_value = argument_expr.eval(it)?;
            argument_variables.push(Variable::new(
                VariableDeclarationKind::Let,
                parameter_name,
                argument_value,
            ))
        }

        let declared_scope = function.declared_scope().deref().clone();
        let fn_scope_ctx = ScopeCtx::new(argument_variables);

        it.vm_mut().stack_mut().push_frame(declared_scope);
        it.vm_mut()
            .stack_mut()
            .frame_mut()
            .push_scope_ctx(fn_scope_ctx);
        function.body().eval(it)?;
        it.vm_mut().stack_mut().frame_mut().pop_scope();
        it.vm_mut().stack_mut().pop_frame();

        Ok(match it.vm_mut().reset_execution_state() {
            ExecutionState::Advance => Value::Undefined,
            ExecutionState::Return(value) => value,
            execution_state => panic!("Unexpected execution state: {:?}", execution_state),
        })
    }
}

impl Eval for PropertyAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let base_value = self.base.eval(it)?;
        let base_obj = match base_value {
            Value::Reference(ref base_refr) => it.vm().heap().resolve(base_refr),
            base_value => todo!("PropertyExpression::eval: base={:?}", base_value),
        };
        let property_name = it.vm().constant_pool().lookup(self.property_name);
        Ok(base_obj
            .property(property_name)
            .cloned()
            .unwrap_or_default())
    }
}

impl Eval for VariableAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let variable = it
            .vm()
            .stack()
            .frame()
            .scope()
            .lookup_variable(self.var_name)?;
        let value = variable.value().clone();
        Ok(value)
    }
}
