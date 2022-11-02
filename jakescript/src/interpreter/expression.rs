use super::error::{
    Error, ErrorKind, NotCallableError, NumericOverflowError, Result, VariableNotDefinedError,
};
use super::object::PropertyKey;
use super::value::{Number, Value};
use super::{Eval, Interpreter};
use crate::ast::*;
use std::assert_matches::assert_matches;
use std::cmp;
use std::ops::{BitAnd, BitOr, BitXor};

impl Eval for Expression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::IdentifierReference(ref node) => node.eval(it),
            Self::This(ref node) => node.eval(it),
            Self::New(ref node) => node.eval(it),
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
        Ok(
            if let Ok(variable) = it.vm().stack().lookup_variable(&self.identifier) {
                variable.value()
            } else {
                let receiver = it.vm().runtime().global_object_ref();
                let global_obj = it.vm().heap().resolve(receiver);
                let value = global_obj
                    .as_ref()
                    .get(it, &self.identifier, receiver)
                    .map_err(|err| Error::new(err, self.source_location()))?
                    .ok_or_else(|| {
                        Error::new(
                            VariableNotDefinedError::new(self.identifier.clone()),
                            self.source_location(),
                        )
                    })?;
                value
            },
        )
    }
}

impl Eval for ThisExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        Ok(Value::Object(
            it.vm()
                .stack()
                .receiver()
                .unwrap_or_else(|| it.vm().runtime().global_object_ref()),
        ))
    }
}

impl Eval for NewExpression {
    type Output = Value;

    fn eval(&self, _: &mut Interpreter) -> Result<Self::Output> {
        todo!("NewExpression::eval: {:#?}", self)
    }
}

impl Eval for AssignmentExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let map_err = |err: ErrorKind| Error::new(err, self.source_location());
        let compute_updated = |it: &mut Interpreter, lhs: Value| match self.op {
            AssignmentOperator::Assign => {
                let rhs = self.rhs.eval(it)?;
                Ok((rhs, rhs))
            }
            AssignmentOperator::ComputeAssign(op) => {
                eval_binary_op(it, op, |_| Ok(lhs), |it| self.rhs.eval(it))
                    .map_err(map_err)?
                    .map(|result_value| (result_value, result_value))
            }
        };

        assert_matches!(self.op.associativity(), Associativity::RightToLeft);
        match self.lhs.as_ref() {
            Expression::IdentifierReference(lhs_node) => it
                .update_variable_or_global_object_property(
                    &lhs_node.identifier,
                    compute_updated,
                    map_err,
                ),
            Expression::Member(MemberExpression::MemberAccess(lhs_node)) => {
                match lhs_node.base.eval(it)? {
                    Value::Object(lhs_ref) => it.update_object_property(
                        lhs_ref,
                        &lhs_node.member,
                        compute_updated,
                        map_err,
                    ),
                    lhs => todo!("AssignmentExpression::eval: base_value={:?}", lhs),
                }
            }
            Expression::Member(MemberExpression::ComputedMemberAccess(lhs_node)) => {
                match lhs_node.base.eval(it)? {
                    Value::Object(lhs_ref) => {
                        let prop_value = lhs_node.member.eval(it)?;
                        let prop_name = it.coerce_to_string(prop_value);
                        let prop_key =
                            PropertyKey::try_from(prop_name.as_ref()).unwrap_or_else(|_| {
                                // FIXME: Remove this restriction as I think it's actually OK to key
                                // an object property by the empty
                                // string.
                                todo!("AssignmentExpression::eval: prop_name={}", prop_value)
                            });
                        it.update_object_property(lhs_ref, &prop_key, compute_updated, map_err)
                    }
                    lhs => todo!("AssignmentExpression::eval: base_value={:?}", lhs),
                }
            }
            lhs => todo!("AssignmentExpression::eval: lhs={:#?}", lhs),
        }
    }
}

impl Eval for BinaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        eval_binary_op(it, self.op, |it| self.lhs.eval(it), |it| self.rhs.eval(it))
            .map_err(|err| Error::new(err, self.source_location()))?
    }
}

impl Eval for RelationalExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        assert_matches!(self.op.associativity(), Associativity::LeftToRight);
        let lhs = self.lhs.eval(it)?;
        let rhs = self.rhs.eval(it)?;

        Ok(Value::Boolean(match self.op {
            RelationalOperator::Equality => it.equal(lhs, rhs),
            RelationalOperator::Inequality => !it.equal(lhs, rhs),
            RelationalOperator::StrictEquality => it.strictly_equal(lhs, rhs),
            RelationalOperator::StrictInequality => !it.strictly_equal(lhs, rhs),
            RelationalOperator::GreaterThan => it.compare(lhs, rhs, cmp::Ordering::is_gt),
            RelationalOperator::GreaterThanOrEqual => it.compare(lhs, rhs, cmp::Ordering::is_ge),
            RelationalOperator::LessThan => it.compare(lhs, rhs, cmp::Ordering::is_lt),
            RelationalOperator::LessThanOrEqual => it.compare(lhs, rhs, cmp::Ordering::is_le),
        }))
    }
}

impl Eval for UnaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let operand = self.operand.eval(it)?;
        Ok(match self.op {
            UnaryOperator::NumericPlus => Value::Number(it.coerce_to_number(operand)),
            UnaryOperator::NumericNegation => it
                .coerce_to_number(operand)
                .checked_neg()
                .map(Value::Number)
                .ok_or_else(NumericOverflowError::new)
                .map_err(|err| Error::new(err, self.source_location()))?,
            UnaryOperator::BitwiseNot => Value::Number(!it.coerce_to_number(operand)),
            UnaryOperator::LogicalNot => Value::Boolean(!it.coerce_to_bool(operand)),
        })
    }
}

impl Eval for UpdateExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let map_err = |err: ErrorKind| Error::new(err, self.source_location());
        let compute_updated =
            |it: &mut Interpreter, operand: Value| -> std::result::Result<_, Error> {
                let new_value = match self.op {
                    UpdateOperator::GetAndIncrement | UpdateOperator::IncrementAndGet => it
                        .coerce_to_number(operand)
                        .checked_add(Number::ONE)
                        .map(Value::Number)
                        .ok_or_else(NumericOverflowError::new)
                        .map_err(|err| Error::new(err, self.source_location()))?,
                    UpdateOperator::GetAndDecrement | UpdateOperator::DecrementAndGet => it
                        .coerce_to_number(operand)
                        .checked_sub(Number::ONE)
                        .map(Value::Number)
                        .ok_or_else(NumericOverflowError::new)
                        .map_err(|err| Error::new(err, self.source_location()))?,
                };
                Ok(match self.op {
                    UpdateOperator::GetAndIncrement | UpdateOperator::GetAndDecrement => {
                        (operand, new_value)
                    }
                    UpdateOperator::IncrementAndGet | UpdateOperator::DecrementAndGet => {
                        (new_value, new_value)
                    }
                })
            };

        assert_matches!(self.op.associativity(), Associativity::RightToLeft);
        match self.operand.as_ref() {
            Expression::IdentifierReference(operand_node) => it
                .update_variable_or_global_object_property(
                    &operand_node.identifier,
                    compute_updated,
                    map_err,
                ),
            Expression::Member(MemberExpression::MemberAccess(operand_node)) => {
                match operand_node.base.eval(it)? {
                    Value::Object(operand_ref) => it.update_object_property(
                        operand_ref,
                        &operand_node.member,
                        compute_updated,
                        map_err,
                    ),
                    operand => todo!("UpdateExpression::eval: operand={:?}", operand),
                }
            }
            operand => todo!("UpdateExpression::eval: operand={:#?}", operand),
        }
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
            Value::Object(base_refr) => {
                let base_obj = it.vm().heap().resolve(base_refr);
                let value = base_obj
                    .as_ref()
                    .get(it, &self.member, base_refr)
                    .map_err(|err| Error::new(err, self.source_location()))?
                    .unwrap_or_default();
                Ok(value)
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
            Value::Object(base_refr) => (base_refr, it.vm().heap().resolve(base_refr)),
            base_value => todo!("ComputedPropertyExpression::eval: base={:?}", base_value),
        };
        let property_value = self.member.eval(it)?;
        let property = match property_value {
            Value::Number(Number::Int(n)) => PropertyKey::from(n),
            property => todo!("ComputedPropertyExpression::eval: property={:?}", property),
        };
        let value = base_obj
            .as_ref()
            .get(it, &property, base_refr)
            .map_err(|err| Error::new(err, self.source_location()))?
            .unwrap_or_default();
        Ok(value)
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
            _ => {
                return Err(Error::new(NotCallableError::new(), self.source_location()));
            }
        };

        let mut supplied_args = Vec::with_capacity(self.arguments.len());
        for arg in &self.arguments {
            supplied_args.push(arg.eval(it)?);
        }
        let fn_obj = it.vm().heap().resolve(fn_obj_ref);
        let result = fn_obj
            .as_ref()
            .call(it, fn_obj_ref, receiver, &supplied_args)
            .map_err(|err| Error::new(err, self.source_location()))?;
        Ok(result)
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
        if it.is_truthy(condition) {
            self.lhs.eval(it)
        } else {
            self.rhs.eval(it)
        }
    }
}

fn eval_binary_op(
    it: &mut Interpreter,
    op_kind: BinaryOperator,
    lhs: impl FnOnce(&mut Interpreter) -> std::result::Result<Value, Error>,
    rhs: impl FnOnce(&mut Interpreter) -> std::result::Result<Value, Error>,
) -> std::result::Result<std::result::Result<Value, Error>, ErrorKind> {
    match op_kind {
        BinaryOperator::LogicalAnd => {
            assert_eq!(op_kind.associativity(), Associativity::LeftToRight);
            let lhs = match lhs(it) {
                Ok(lhs) => lhs,
                Err(err) => return Ok(Err(err)),
            };
            return Ok(if it.is_truthy(lhs) { rhs(it) } else { Ok(lhs) });
        }
        BinaryOperator::LogicalOr => {
            assert_eq!(op_kind.associativity(), Associativity::LeftToRight);
            let lhs = match lhs(it) {
                Ok(lhs) => lhs,
                Err(err) => return Ok(Err(err)),
            };
            return Ok(if it.is_truthy(lhs) { Ok(lhs) } else { rhs(it) });
        }
        _ => {}
    }

    let (lhs, rhs) = match op_kind.associativity() {
        Associativity::LeftToRight => {
            let lhs = match lhs(it) {
                Ok(lhs) => lhs,
                Err(err) => return Ok(Err(err)),
            };
            let rhs = match rhs(it) {
                Ok(rhs) => rhs,
                Err(err) => return Ok(Err(err)),
            };
            (lhs, rhs)
        }
        Associativity::RightToLeft => {
            let rhs = match rhs(it) {
                Ok(rhs) => rhs,
                Err(err) => return Ok(Err(err)),
            };
            let lhs = match lhs(it) {
                Ok(lhs) => lhs,
                Err(err) => return Ok(Err(err)),
            };
            (lhs, rhs)
        }
    };

    if matches!(op_kind, BinaryOperator::Addition) && (lhs.is_object() || rhs.is_object()) {
        let lhs = it.coerce_to_string(lhs);
        let rhs = it.coerce_to_string(rhs);
        return Ok(Ok(Value::Object(it.concat(&lhs, &rhs)?)));
    }

    let lhs = it.coerce_to_number(lhs);
    let rhs = it.coerce_to_number(rhs);
    let result = match op_kind {
        BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => unreachable!(),
        BinaryOperator::Addition => lhs.checked_add(rhs),
        BinaryOperator::Division => lhs.checked_div(rhs),
        BinaryOperator::Modulus => lhs.checked_rem(rhs),
        BinaryOperator::Multiplication => lhs.checked_mul(rhs),
        BinaryOperator::Exponentiation => Some(lhs.pow(rhs)),
        BinaryOperator::Subtraction => lhs.checked_sub(rhs),
        BinaryOperator::BitwiseAnd => Some(lhs.bitand(rhs)),
        BinaryOperator::BitwiseOr => Some(lhs.bitor(rhs)),
        BinaryOperator::BitwiseXOr => Some(lhs.bitxor(rhs)),
        BinaryOperator::BitwiseLeftShift => lhs.checked_shl(rhs),
        BinaryOperator::BitwiseRightShift => lhs.checked_shr_signed(rhs),
        BinaryOperator::BitwiseRightShiftUnsigned => lhs.checked_shr_unsigned(rhs),
    };
    result
        .map(Value::Number)
        .map(Ok)
        .ok_or_else(|| ErrorKind::from(NumericOverflowError::new()))
}
