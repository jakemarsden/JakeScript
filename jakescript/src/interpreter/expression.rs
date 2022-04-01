use super::error::{Error, NotCallableError, Result};
use super::stack::{ScopeCtx, Variable, VariableKind};
use super::value::{Number, Value};
use super::vm::ExecutionState;
use super::{Eval, Interpreter};
use crate::ast::*;
use std::assert_matches::assert_matches;
use std::hint::unreachable_unchecked;

impl Eval for Expression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::IdentifierReference(ref node) => node.eval(it),
            Self::Literal(ref node) => node.eval(it),
            Self::Array(ref node) => node.eval(it),
            Self::Object(ref node) => node.eval(it),
            Self::Function(ref node) => node.eval(it),
            Self::Assignment(ref node) => node.eval(it),
            Self::Binary(ref node) => node.eval(it),
            Self::Relational(ref node) => node.eval(it),
            Self::Unary(ref node) => node.eval(it),
            Self::Update(ref node) => node.eval(it),
            Self::Member(ref node) => node.eval(it),
            Self::Grouping(ref node) => node.eval(it),
            Self::Ternary(ref node) => node.eval(it),
        }
    }
}

impl Eval for IdentifierReferenceExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        if let Some(variable) = it
            .vm()
            .stack()
            .frame()
            .scope()
            .lookup_variable(&self.identifier)
        {
            let value = variable.value().clone();
            Ok(value)
        } else {
            it.vm().runtime().global_object().property(&self.identifier)
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
            Ok(match self_.op {
                AssignmentOperator::Assign => rhs,
                AssignmentOperator::ComputeAssign(BinaryOperator::Addition) => {
                    Value::add_or_append(it.vm(), &getter()?, &rhs)?
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Subtraction) => {
                    Value::sub(&getter()?, &rhs)?
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Multiplication) => {
                    Value::mul(&getter()?, &rhs)?
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Division) => {
                    Value::div(&getter()?, &rhs)?
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Modulus) => {
                    Value::rem(&getter()?, &rhs)?
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Exponentiation) => {
                    Value::pow(&getter()?, &rhs)?
                }
                kind @ AssignmentOperator::ComputeAssign(..) => {
                    todo!("AssignmentExpression::eval: kind={:?}", kind)
                }
            })
        }

        assert_matches!(self.op.associativity(), Associativity::RightToLeft);
        match self.lhs.as_ref() {
            Expression::IdentifierReference(node) => {
                if let Some(mut variable) = it
                    .vm()
                    .stack()
                    .frame()
                    .scope()
                    .lookup_variable(&node.identifier)
                {
                    let new_value = compute_new_value(self, it, || Ok(variable.value().clone()))?;
                    variable.set_value(new_value.clone())?;
                    Ok(new_value)
                } else {
                    let value = it
                        .vm()
                        .runtime()
                        .global_object()
                        .property(&node.identifier)?;
                    let new_value = compute_new_value(self, it, || Ok(value.clone()))?;
                    it.vm_mut()
                        .runtime_mut()
                        .global_object_mut()
                        .set_property(&node.identifier, value.clone())?;
                    Ok(new_value)
                }
            }
            Expression::Member(MemberExpression::MemberAccess(node)) => {
                let base_value = node.base.eval(it)?;
                match base_value {
                    Value::Reference(ref base_refr) => {
                        let mut base_obj = it.vm_mut().heap_mut().resolve_mut(base_refr);
                        let new_value = compute_new_value(self, it, || {
                            Ok(base_obj.property(&node.member).cloned().unwrap_or_default())
                        })?;
                        base_obj.set_property(node.member.clone(), new_value.clone());
                        Ok(new_value)
                    }
                    Value::NativeObject(ref base_refr) => {
                        let mut base_obj = it.vm_mut().runtime_mut().resolve_mut(base_refr);
                        let new_value =
                            compute_new_value(self, it, || base_obj.property(&node.member))?;
                        base_obj.set_property(&node.member, new_value.clone())?;
                        Ok(new_value)
                    }
                    base_value => todo!("AssignmentExpression::eval: base_value={:?}", base_value),
                }
            }
            expr => todo!("AssignmentExpression::eval: lhs={:#?}", expr),
        }
    }
}

impl Eval for BinaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        Ok(match self.op {
            // Get the boolean ops out of the way first, since they don't let us eval the RHS
            // up-front (which is more ergonomic for all the other ops).
            BinaryOperator::LogicalAnd => {
                assert_matches!(self.op.associativity(), Associativity::LeftToRight);
                match self.lhs.eval(it)? {
                    lhs if lhs.is_truthy() => self.rhs.eval(it)?,
                    lhs => lhs,
                }
            }
            BinaryOperator::LogicalOr => {
                assert_matches!(self.op.associativity(), Associativity::LeftToRight);
                match self.lhs.eval(it)? {
                    lhs if lhs.is_falsy() => self.rhs.eval(it)?,
                    lhs => lhs,
                }
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
                    BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => unsafe {
                        unreachable_unchecked()
                    },

                    BinaryOperator::Addition => Value::add_or_append(it.vm(), lhs, rhs)?,
                    BinaryOperator::Subtraction => Value::sub(lhs, rhs)?,
                    BinaryOperator::Multiplication => Value::mul(lhs, rhs)?,
                    BinaryOperator::Division => Value::div(lhs, rhs)?,
                    BinaryOperator::Modulus => Value::rem(lhs, rhs)?,
                    BinaryOperator::Exponentiation => Value::pow(lhs, rhs)?,
                    BinaryOperator::BitwiseAnd => Value::bitand(lhs, rhs),
                    BinaryOperator::BitwiseOr => Value::bitor(lhs, rhs),
                    BinaryOperator::BitwiseXOr => Value::bitxor(lhs, rhs),
                    BinaryOperator::BitwiseLeftShift => Value::shl(lhs, rhs)?,
                    BinaryOperator::BitwiseRightShift => Value::shr_signed(lhs, rhs)?,
                    BinaryOperator::BitwiseRightShiftUnsigned => Value::shr_unsigned(lhs, rhs)?,
                }
            }
        })
    }
}

impl Eval for RelationalExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        assert_matches!(self.op.associativity(), Associativity::LeftToRight);
        let lhs = &self.lhs.eval(it)?;
        let rhs = &self.rhs.eval(it)?;
        Ok(match self.op {
            RelationalOperator::Equality => Value::eq(it.vm(), lhs, rhs),
            RelationalOperator::Inequality => Value::ne(it.vm(), lhs, rhs),
            RelationalOperator::StrictEquality => Value::identical(it.vm(), lhs, rhs),
            RelationalOperator::StrictInequality => Value::not_identical(it.vm(), lhs, rhs),
            RelationalOperator::GreaterThan => Value::gt(it.vm(), lhs, rhs),
            RelationalOperator::GreaterThanOrEqual => Value::ge(it.vm(), lhs, rhs),
            RelationalOperator::LessThan => Value::lt(it.vm(), lhs, rhs),
            RelationalOperator::LessThanOrEqual => Value::le(it.vm(), lhs, rhs),
        })
    }
}

impl Eval for UnaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let operand = &self.operand.eval(it)?;
        Ok(match self.op {
            UnaryOperator::BitwiseNot => Value::bitnot(operand),
            UnaryOperator::LogicalNot => Value::not(operand),
            UnaryOperator::NumericNegation => Value::neg(operand)?,
            UnaryOperator::NumericPlus => Value::plus(operand),
        })
    }
}

impl Eval for UpdateExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        fn compute(
            self_: &UpdateExpression,
            it: &mut Interpreter,
            getter: impl FnOnce() -> Result,
        ) -> Result<(Value, Value)> {
            const ONE: Value = Value::Number(Number::Int(1));
            let old_value = getter()?;

            // The new value to assign to the variable or property
            let new_value = match self_.op {
                UpdateOperator::GetAndIncrement | UpdateOperator::IncrementAndGet => {
                    Value::add_or_append(it.vm(), &old_value, &ONE)?
                }
                UpdateOperator::GetAndDecrement | UpdateOperator::DecrementAndGet => {
                    Value::sub(&old_value, &ONE)?
                }
            };
            // The value to use as the result of the expression
            let result_value = match self_.op {
                UpdateOperator::GetAndIncrement | UpdateOperator::GetAndDecrement => old_value,
                UpdateOperator::IncrementAndGet | UpdateOperator::DecrementAndGet => {
                    new_value.clone()
                }
            };
            Ok((new_value, result_value))
        }

        assert_matches!(self.op.associativity(), Associativity::RightToLeft);
        Ok(match self.operand.as_ref() {
            Expression::IdentifierReference(node) => {
                if let Some(mut variable) = it
                    .vm()
                    .stack()
                    .frame()
                    .scope()
                    .lookup_variable(&node.identifier)
                {
                    let (new_value, result_value) =
                        compute(self, it, || Ok(variable.value().clone()))?;
                    variable.set_value(new_value)?;
                    result_value
                } else {
                    let value = it
                        .vm()
                        .runtime()
                        .global_object()
                        .property(&node.identifier)?;
                    let (new_value, result_value) = compute(self, it, || Ok(value.clone()))?;
                    it.vm_mut()
                        .runtime_mut()
                        .global_object_mut()
                        .set_property(&node.identifier, new_value)?;
                    result_value
                }
            }
            Expression::Member(MemberExpression::MemberAccess(node)) => {
                let base_value = node.base.eval(it)?;
                match base_value {
                    Value::Reference(ref base_refr) => {
                        let mut base_obj = it.vm_mut().heap_mut().resolve_mut(base_refr);
                        let (new_value, result_value) = compute(self, it, || {
                            Ok(base_obj.property(&node.member).cloned().unwrap_or_default())
                        })?;
                        base_obj.set_property(node.member.clone(), new_value);
                        result_value
                    }
                    Value::NativeObject(ref base_refr) => {
                        let mut base_obj = it.vm_mut().runtime_mut().resolve_mut(base_refr);
                        let (new_value, result_value) =
                            compute(self, it, || base_obj.property(&node.member))?;
                        base_obj.set_property(&node.member, new_value)?;
                        result_value
                    }
                    base_value => {
                        todo!("AssignmentExpression::eval: base_value={:?}", base_value)
                    }
                }
            }
            _ => todo!("UnaryExpression::eval: self={:#?}", self),
        })
    }
}

impl Eval for MemberExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::FunctionCall(node) => node.eval(it),
            Self::MemberAccess(node) => node.eval(it),
            Self::ComputedMemberAccess(node) => node.eval(it),
        }
    }
}

impl Eval for FunctionCallExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self.function.eval(it)? {
            Value::Reference(fn_obj_ref) => {
                let fn_obj = it.vm().heap().resolve(&fn_obj_ref);
                let function = if let Some(callable) = fn_obj.callable() {
                    callable
                } else {
                    return Err(Error::NotCallable(NotCallableError));
                };

                let declared_param_names = function.declared_parameters();
                let mut supplied_args = self.arguments.iter();
                let mut variables = Vec::with_capacity(declared_param_names.len());

                for declared_param_name in declared_param_names.iter() {
                    let arg_value = match supplied_args.next() {
                        Some(supplied_arg) => supplied_arg.eval(it)?,
                        None => Value::Undefined,
                    };
                    variables.push(Variable::new(
                        VariableKind::Let,
                        declared_param_name.clone(),
                        arg_value,
                    ));
                }

                // Evaluate remaining arguments when more arguments are supplied than the function
                // has parameters.
                for arg in supplied_args {
                    arg.eval(it)?;
                }

                let declared_scope = function.declared_scope().clone();
                let fn_scope_ctx = ScopeCtx::new(variables);

                it.vm_mut().stack_mut().push_frame(declared_scope);
                if let Some(fn_name) = function.name() {
                    // Create an outer scope with nothing but the function's name, which points to
                    // itself, so that named function literals may recurse using their name, without
                    // making the name visible outside of the function body. It has its own outer
                    // scope so it can still be shadowed by parameters with the same name.
                    let fn_scope_ctx_outer = ScopeCtx::new(vec![Variable::new(
                        VariableKind::Var,
                        fn_name.clone(),
                        Value::Reference(fn_obj_ref.clone()),
                    )]);
                    it.vm_mut()
                        .stack_mut()
                        .frame_mut()
                        .push_scope(fn_scope_ctx_outer, false);
                }
                it.vm_mut()
                    .stack_mut()
                    .frame_mut()
                    .push_scope(fn_scope_ctx, true);
                function.body().eval(it)?;
                it.vm_mut().stack_mut().frame_mut().pop_scope();
                if function.name().is_some() {
                    it.vm_mut().stack_mut().frame_mut().pop_scope();
                }
                it.vm_mut().stack_mut().pop_frame();

                Ok(match it.vm_mut().reset_execution_state() {
                    ExecutionState::Advance | ExecutionState::Exit => Value::Undefined,
                    ExecutionState::Return(value) => value,
                    execution_state => panic!("Unexpected execution state: {:?}", execution_state),
                })
            }
            Value::NativeObject(fn_obj_ref) => {
                let fn_obj = it.vm().runtime().resolve(&fn_obj_ref);

                let supplied_args = &self.arguments;
                let mut args = Vec::with_capacity(supplied_args.len());
                for supplied_arg in supplied_args {
                    let arg_value = supplied_arg.eval(it)?;
                    args.push(arg_value);
                }

                fn_obj.invoke(it.vm_mut(), &args)
            }
            _ => Err(Error::NotCallable(NotCallableError)),
        }
    }
}

impl Eval for MemberAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let base_value = self.base.eval(it)?;
        match base_value {
            Value::Reference(ref base_refr) => {
                let base_obj = it.vm().heap().resolve(base_refr);
                Ok(base_obj.property(&self.member).cloned().unwrap_or_default())
            }
            Value::NativeObject(ref base_refr) => {
                let base_obj = it.vm().runtime().resolve(base_refr);
                base_obj.property(&self.member)
            }
            base_value => todo!("PropertyAccessExpression::eval: base={:?}", base_value),
        }
    }
}

impl Eval for ComputedMemberAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let base_value = self.base.eval(it)?;
        let base_obj = match base_value {
            Value::Reference(ref base_refr) => it.vm().heap().resolve(base_refr),
            base_value => todo!("ComputedPropertyExpression::eval: base={:?}", base_value),
        };
        let property_value = self.member.eval(it)?;
        let property = match property_value {
            Value::Number(Number::Int(n)) => Identifier::from(n),
            property => todo!("ComputedPropertyExpression::eval: property={:?}", property),
        };
        Ok(base_obj.property(&property).cloned().unwrap_or_default())
    }
}

impl Eval for GroupingExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.inner.eval(it)
    }
}

impl Eval for TernaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let condition = self.condition.eval(it)?;
        if condition.is_truthy() {
            self.lhs.eval(it)
        } else {
            self.rhs.eval(it)
        }
    }
}
