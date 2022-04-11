use super::error::{Error, ErrorKind, NotCallableError, Result};
use super::heap::Reference;
use super::object::{Call, PropertyKey, UserFunction};
use super::stack::{ScopeCtx, Variable, VariableKind};
use super::value::{Number, Value};
use super::vm::ExecutionState;
use super::{Eval, Interpreter};
use crate::ast::*;
use crate::runtime::NativeCall;
use std::assert_matches::assert_matches;
use std::hint::unreachable_unchecked;

impl Eval for Expression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::IdentifierReference(ref node) => node.eval(it),
            Self::This(ref node) => node.eval(it),
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
            let receiver = it.vm().runtime().global_object_ref().clone();
            it.vm()
                .global_object()
                .get(it, &PropertyKey::from(&self.identifier), receiver)
                .map_err(|err| Error::new(err, self.source_location()))
        }
    }
}

impl Eval for ThisExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        Ok(Value::Object(
            it.vm()
                .stack()
                .frame()
                .receiver()
                .cloned()
                .unwrap_or_else(|| it.vm().runtime().global_object_ref().clone()),
        ))
    }
}

impl Eval for AssignmentExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        fn compute_new_value(
            self_: &AssignmentExpression,
            it: &mut Interpreter,
            getter: impl FnOnce(&Interpreter) -> std::result::Result<Value, ErrorKind>,
        ) -> Result {
            let rhs = self_.rhs.eval(it)?;
            let getter = |it| getter(it).map_err(|err| Error::new(err, self_.source_location()));
            let result = match self_.op {
                AssignmentOperator::Assign => Ok(rhs),
                AssignmentOperator::ComputeAssign(BinaryOperator::Addition) => {
                    it.add_or_concat(&getter(it)?, &rhs)
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Subtraction) => {
                    it.sub(&getter(it)?, &rhs)
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Multiplication) => {
                    it.mul(&getter(it)?, &rhs)
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Division) => {
                    it.div(&getter(it)?, &rhs)
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Modulus) => {
                    it.rem(&getter(it)?, &rhs)
                }
                AssignmentOperator::ComputeAssign(BinaryOperator::Exponentiation) => {
                    it.pow(&getter(it)?, &rhs)
                }
                kind @ AssignmentOperator::ComputeAssign(..) => {
                    todo!("AssignmentExpression::eval: kind={:?}", kind)
                }
            };
            result.map_err(|err| Error::new(err, self_.source_location()))
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
                    let new_value = compute_new_value(self, it, |_| Ok(variable.value().clone()))?;
                    variable
                        .set_value(new_value.clone())
                        .map_err(|err| Error::new(err, self.source_location()))?;
                    Ok(new_value)
                } else {
                    let key = PropertyKey::from(&node.identifier);
                    let global_obj_ref = it.vm().runtime().global_object_ref().clone();
                    let value = it
                        .vm()
                        .global_object()
                        .get(it, &key, global_obj_ref.clone())
                        .map_err(|err| Error::new(err, self.source_location()))?;
                    let new_value = compute_new_value(self, it, |_| Ok(value.clone()))?;

                    it.vm_mut()
                        .heap_mut()
                        .resolve_mut(&global_obj_ref.clone())
                        .set(it, &key, global_obj_ref, value.clone())
                        .map_err(|err| Error::new(err, self.source_location()))?;
                    Ok(new_value)
                }
            }
            Expression::Member(MemberExpression::MemberAccess(node)) => {
                let base_value = node.base.eval(it)?;
                match base_value {
                    Value::Object(ref base_refr) => {
                        let key = PropertyKey::from(&node.member);
                        let mut base_obj = it.vm_mut().heap_mut().resolve_mut(base_refr);
                        let new_value = compute_new_value(self, it, |it| {
                            Ok(base_obj
                                .get(it, &key, base_refr.clone())
                                .unwrap_or_default())
                        })?;
                        base_obj
                            .set(it, &key, base_refr.clone(), new_value.clone())
                            .map_err(|err| Error::new(err, self.source_location()))?;
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
                    lhs if it.is_truthy(&lhs) => self.rhs.eval(it)?,
                    lhs => lhs,
                }
            }
            BinaryOperator::LogicalOr => {
                assert_matches!(self.op.associativity(), Associativity::LeftToRight);
                match self.lhs.eval(it)? {
                    lhs if !it.is_truthy(&lhs) => self.rhs.eval(it)?,
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
                let result = match kind {
                    // Safety: Unreachable because the possible values are already handled by
                    // previous match arms in the outer match expression.
                    BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => unsafe {
                        unreachable_unchecked()
                    },

                    BinaryOperator::Addition => it.add_or_concat(lhs, rhs),
                    BinaryOperator::Subtraction => it.sub(lhs, rhs),
                    BinaryOperator::Multiplication => it.mul(lhs, rhs),
                    BinaryOperator::Division => it.div(lhs, rhs),
                    BinaryOperator::Modulus => it.rem(lhs, rhs),
                    BinaryOperator::Exponentiation => it.pow(lhs, rhs),
                    BinaryOperator::BitwiseAnd => it.bitand(lhs, rhs),
                    BinaryOperator::BitwiseOr => it.bitor(lhs, rhs),
                    BinaryOperator::BitwiseXOr => it.bitxor(lhs, rhs),
                    BinaryOperator::BitwiseLeftShift => it.shl(lhs, rhs),
                    BinaryOperator::BitwiseRightShift => it.shr_signed(lhs, rhs),
                    BinaryOperator::BitwiseRightShiftUnsigned => it.shr_unsigned(lhs, rhs),
                };
                result.map_err(|err| Error::new(err, self.source_location()))?
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
        let result = match self.op {
            RelationalOperator::Equality => it.equal(lhs, rhs),
            RelationalOperator::Inequality => it.not_equal(lhs, rhs),
            RelationalOperator::StrictEquality => it.strictly_equal(lhs, rhs),
            RelationalOperator::StrictInequality => it.not_strictly_equal(lhs, rhs),
            RelationalOperator::GreaterThan => it.gt(lhs, rhs),
            RelationalOperator::GreaterThanOrEqual => it.ge(lhs, rhs),
            RelationalOperator::LessThan => it.lt(lhs, rhs),
            RelationalOperator::LessThanOrEqual => it.le(lhs, rhs),
        };
        result.map_err(|err| Error::new(err, self.source_location()))
    }
}

impl Eval for UnaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let operand = &self.operand.eval(it)?;
        let result = match self.op {
            UnaryOperator::BitwiseNot => it.bitnot(operand),
            UnaryOperator::LogicalNot => it.not(operand),
            UnaryOperator::NumericNegation => it.negate(operand),
            UnaryOperator::NumericPlus => it.plus(operand),
        };
        result.map_err(|err| Error::new(err, self.source_location()))
    }
}

impl Eval for UpdateExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        fn compute(
            self_: &UpdateExpression,
            it: &mut Interpreter,
            getter: impl FnOnce(&Interpreter) -> std::result::Result<Value, ErrorKind>,
        ) -> Result<(Value, Value)> {
            let old_value = getter(it).map_err(|err| Error::new(err, self_.source_location()))?;

            // The new value to assign to the variable or property
            let new_value = match self_.op {
                UpdateOperator::GetAndIncrement | UpdateOperator::IncrementAndGet => {
                    it.add(&old_value, &Value::Number(Number::Int(1)))
                }
                UpdateOperator::GetAndDecrement | UpdateOperator::DecrementAndGet => {
                    it.sub(&old_value, &Value::Number(Number::Int(1)))
                }
            }
            .map_err(|err| Error::new(err, self_.source_location()))?;
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
                        compute(self, it, |_| Ok(variable.value().clone()))?;
                    variable
                        .set_value(new_value)
                        .map_err(|err| Error::new(err, self.source_location()))?;
                    result_value
                } else {
                    let global_obj_ref = it.vm().runtime().global_object_ref().clone();
                    let value = it
                        .vm()
                        .global_object()
                        .get(
                            it,
                            &PropertyKey::from(&node.identifier),
                            global_obj_ref.clone(),
                        )
                        .map_err(|err| Error::new(err, self.source_location()))?;
                    let (new_value, result_value) = compute(self, it, |_| Ok(value.clone()))?;

                    let mut global_obj = it.vm_mut().heap_mut().resolve_mut(&global_obj_ref);
                    global_obj
                        .set(
                            it,
                            &PropertyKey::from(&node.identifier),
                            global_obj_ref.clone(),
                            new_value,
                        )
                        .map_err(|err| Error::new(err, self.source_location()))?;
                    result_value
                }
            }
            Expression::Member(MemberExpression::MemberAccess(node)) => {
                let base_value = node.base.eval(it)?;
                match base_value {
                    Value::Object(ref base_refr) => {
                        let mut base_obj = it.vm_mut().heap_mut().resolve_mut(base_refr);
                        let (new_value, result_value) = compute(self, it, |it| {
                            Ok(base_obj
                                .get(it, &PropertyKey::from(&node.member), base_refr.clone())
                                .unwrap_or_default())
                        })?;
                        base_obj
                            .set(
                                it,
                                &PropertyKey::from(&node.member),
                                base_refr.clone(),
                                new_value,
                            )
                            .map_err(|err| Error::new(err, self.source_location()))?;
                        result_value
                    }
                    base_value => {
                        todo!("AssignmentExpression::eval: base_value={:?}", base_value)
                    }
                }
            }
            expr => todo!("UnaryExpression::eval: operand={:#?}", expr),
        })
    }
}

impl Eval for MemberExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::MemberAccess(node) => node.eval(it),
            Self::ComputedMemberAccess(node) => node.eval(it),
            Self::FunctionCall(node) => node.eval(it),
        }
    }
}

impl Eval for MemberAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let base_value = self.base.eval(it)?;
        match base_value {
            Value::Object(ref base_refr) => {
                let base_obj = it.vm().heap().resolve(base_refr);
                Ok(base_obj
                    .get(it, &PropertyKey::from(&self.member), base_refr.clone())
                    .unwrap_or_default())
            }
            base_value => todo!("PropertyAccessExpression::eval: base={:?}", base_value),
        }
    }
}

impl Eval for ComputedMemberAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let base_value = self.base.eval(it)?;
        let (base_refr, base_obj) = match base_value {
            Value::Object(ref base_refr) => (base_refr, it.vm().heap().resolve(base_refr)),
            base_value => todo!("ComputedPropertyExpression::eval: base={:?}", base_value),
        };
        let property_value = self.member.eval(it)?;
        let property = match property_value {
            Value::Number(Number::Int(n)) => Identifier::from(n),
            property => todo!("ComputedPropertyExpression::eval: property={:?}", property),
        };
        Ok(base_obj
            .get(it, &PropertyKey::from(&property), base_refr.clone())
            .unwrap_or_default())
    }
}

impl FunctionCallExpression {
    fn call_user_fn(
        &self,
        it: &mut Interpreter,
        function: &UserFunction,
        fn_obj_ref: &Reference,
        receiver: Option<Reference>,
    ) -> Result {
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

        // Evaluate remaining arguments when more arguments are supplied than the function has
        // parameters.
        for arg in supplied_args {
            arg.eval(it)?;
        }

        let declared_scope = function.declared_scope().clone();
        let fn_scope_ctx = ScopeCtx::new(variables);

        it.vm_mut().stack_mut().push_frame(declared_scope, receiver);
        if let Some(fn_name) = function.name() {
            // Create an outer scope with nothing but the function's name, which points to itself,
            // so that named function literals may recurse using their name without making the name
            // visible outside of the function body. It has its own outer scope so it can still be
            // shadowed by parameters with the same name.
            let fn_scope_ctx_outer = ScopeCtx::new(vec![Variable::new(
                VariableKind::Var,
                fn_name.clone(),
                Value::Object(fn_obj_ref.clone()),
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

    fn call_native_fn(
        &self,
        it: &mut Interpreter,
        function: &NativeCall,
        receiver: Option<Reference>,
    ) -> Result {
        let mut supplied_args = Vec::with_capacity(self.arguments.len());
        for arg in &self.arguments {
            supplied_args.push(arg.eval(it)?);
        }
        let receiver = receiver.unwrap_or_else(|| it.vm().runtime().global_object_ref().clone());
        function
            .call(it, receiver, &supplied_args)
            .map_err(|err| Error::new(err, self.source_location()))
    }
}

impl Eval for FunctionCallExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let (receiver, function) = match self.function {
            box Expression::Member(ref node) => match node {
                MemberExpression::MemberAccess(node) => {
                    // FIXME: Don't evaluate `node.base` twice!!
                    (Some(node.base.eval(it)?), node.eval(it)?)
                }
                MemberExpression::ComputedMemberAccess(node) => {
                    // FIXME: Don't evaluate `node.base` twice!!
                    (Some(node.base.eval(it)?), node.eval(it)?)
                }
                MemberExpression::FunctionCall(node) => {
                    // FIXME: Don't evaluate `node.function` twice!!
                    (Some(node.function.eval(it)?), node.eval(it)?)
                }
            },
            box ref node => (None, node.eval(it)?),
        };
        let receiver = match receiver {
            Some(Value::Object(receiver)) => Some(receiver),
            Some(receiver) => todo!("FunctionCallExpression: receiver={:?}", receiver),
            None => None,
        };

        let fn_obj_ref = match function {
            Value::Object(fn_obj_ref) => fn_obj_ref,
            _ => return Err(Error::new(NotCallableError, self.source_location())),
        };
        let fn_obj = it.vm().heap().resolve(&fn_obj_ref);
        match fn_obj.call_data() {
            Some(Call::User(user_fn)) => {
                // Must drop the `fn_obj` we have borrowed from the heap before calling it, because
                // the function body may need to take a unique reference to it within the call (e.g.
                // to evaluate a `this` expression).
                let user_fn = user_fn.clone();
                drop(fn_obj);
                self.call_user_fn(it, &user_fn, &fn_obj_ref, receiver)
            }
            Some(Call::Native(native_fn)) => self.call_native_fn(it, native_fn, receiver),
            None => Err(Error::new(NotCallableError, self.source_location())),
        }
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
        if it.is_truthy(&condition) {
            self.lhs.eval(it)
        } else {
            self.rhs.eval(it)
        }
    }
}
