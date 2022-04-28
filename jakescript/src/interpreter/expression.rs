use super::error::{Error, ErrorKind, NotCallableError, Result, VariableNotDefinedError};
use super::object::PropertyKey;
use super::value::{Number, Value};
use super::{Eval, Interpreter};
use crate::ast::*;
use std::assert_matches::assert_matches;

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
            let global_obj = it.vm().heap().resolve(&receiver);
            global_obj
                .get(
                    it,
                    &PropertyKey::from(self.identifier.clone()),
                    receiver.clone(),
                )
                .map_err(|err| Error::new(err, self.source_location()))?
                .ok_or_else(|| Error::new(VariableNotDefinedError, self.source_location()))
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
        let map_err = |err: ErrorKind| Error::new(err, self.source_location());
        let compute_updated = |it: &mut Interpreter, lhs: &Value| match self.op {
            AssignmentOperator::Assign => {
                let rhs = self.rhs.eval(it)?;
                Ok((rhs.clone(), rhs))
            }
            AssignmentOperator::ComputeAssign(op) => {
                let rhs = self.rhs.eval(it)?;
                it.eval_binary_op(op, lhs, &rhs)
                    .map(|result_value| (result_value.clone(), result_value))
                    .map_err(map_err)
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
                        &lhs_ref,
                        &PropertyKey::from(lhs_node.member.clone()),
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
                        let prop_name = it.coerce_to_string(&prop_value);
                        let prop_key = PropertyKey::try_from(prop_name).unwrap_or_else(|_| {
                            // FIXME: Remove this restriction as I think it's actually OK to key an
                            //  object property by the empty string.
                            todo!("AssignmentExpression::eval: prop_name={}", prop_value)
                        });
                        it.update_object_property(&lhs_ref, &prop_key, compute_updated, map_err)
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
        Ok(match self.op {
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
                it.eval_binary_op(kind, lhs, rhs)
                    .map_err(|err| Error::new(err, self.source_location()))?
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
        it.eval_relational_op(self.op, lhs, rhs)
            .map_err(|err| Error::new(err, self.source_location()))
    }
}

impl Eval for UnaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let operand = &self.operand.eval(it)?;
        it.eval_unary_op(self.op, operand)
            .map_err(|err| Error::new(err, self.source_location()))
    }
}

impl Eval for UpdateExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let map_err = |err: ErrorKind| Error::new(err, self.source_location());
        let compute_updated = |it: &mut Interpreter, operand: &Value| {
            it.eval_update_op(self.op, operand).map_err(map_err)
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
                        &operand_ref,
                        &PropertyKey::from(operand_node.member.clone()),
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
            Value::Object(ref base_refr) => {
                let base_obj = it.vm().heap().resolve(base_refr);
                Ok(base_obj
                    .get(
                        it,
                        &PropertyKey::from(self.member.clone()),
                        base_refr.clone(),
                    )
                    .map_err(|err| Error::new(err, self.source_location()))?
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
            Value::Number(Number::Int(n)) => PropertyKey::from(n),
            property => todo!("ComputedPropertyExpression::eval: property={:?}", property),
        };
        Ok(base_obj
            .get(it, &property, base_refr.clone())
            .map_err(|err| Error::new(err, self.source_location()))?
            .unwrap_or_default())
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

        let mut supplied_args = Vec::with_capacity(self.arguments.len());
        for arg in &self.arguments {
            supplied_args.push(arg.eval(it)?);
        }
        let fn_obj = it.vm().heap().resolve(&fn_obj_ref);
        fn_obj
            .call(it, &fn_obj_ref, receiver, &supplied_args)
            .map_err(|err| Error::new(err, self.source_location()))
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
