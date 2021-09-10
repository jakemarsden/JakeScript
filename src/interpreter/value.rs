use crate::interpreter::heap::Reference;
use crate::interpreter::Interpreter;
use std::ops::*;

#[derive(Clone, Default, Debug)]
pub enum Value {
    Boolean(bool),
    Number(i64),
    String(String),
    Reference(Reference),
    Null,
    #[default]
    Undefined,
}

impl Value {
    pub fn add(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        if matches!(lhs, Self::String(_) | Self::Reference(_)) {
            todo!("Value::add: {:?} + {:?}", lhs, rhs)
        }
        match numeric_bin_op(it, lhs, rhs, i64::checked_add) {
            Some(result) => Self::Number(result),
            None => todo!("Value::add: {:?} + {:?} => Overflow", lhs, rhs),
        }
    }

    pub fn sub(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        match numeric_bin_op(it, lhs, rhs, i64::checked_sub) {
            Some(result) => Self::Number(result),
            None => todo!("Value::sub: {:?} - {:?} => Overflow", lhs, rhs),
        }
    }

    pub fn mul(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        match numeric_bin_op(it, lhs, rhs, i64::checked_mul) {
            Some(result) => Self::Number(result),
            None => todo!("Value::mul: {:?} * {:?} => Overflow", lhs, rhs),
        }
    }

    pub fn div(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        match numeric_bin_op(it, lhs, rhs, i64::checked_div) {
            Some(result) => Self::Number(result),
            None => todo!("Value::div: {:?} / {:?} => Overflow", lhs, rhs),
        }
    }

    pub fn rem(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        match numeric_bin_op(it, lhs, rhs, i64::checked_rem) {
            Some(result) => Self::Number(result),
            None => todo!("Value::rem: {:?} % {:?} => Overflow", lhs, rhs),
        }
    }

    pub fn pow(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        match numeric_bin_op(it, lhs, rhs, checked_pow) {
            Some(result) => Self::Number(result),
            None => todo!("Value::pow: {:?} ** {:?} => Overflow", lhs, rhs),
        }
    }

    pub fn identical(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        let result = match (lhs, rhs) {
            (Self::Boolean(lhs), Self::Boolean(rhs)) => lhs == rhs,
            (Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::Reference(lhs_ref), Self::Reference(rhs_ref)) => {
                lhs_ref == rhs_ref || {
                    let lhs_obj = it.vm().heap().resolve(lhs_ref);
                    let rhs_obj = it.vm().heap().resolve(rhs_ref);
                    lhs_obj.js_equals(rhs_obj.deref())
                }
            }
            (Self::Null, Self::Null) => true,
            (Self::Undefined, Self::Undefined) => true,
            (_, _) => false,
        };
        Self::Boolean(result)
    }

    pub fn not_identical(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        let identical = Self::identical(it, lhs, rhs);
        Self::not(it, &identical)
    }

    pub fn eq(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        match lhs {
            Self::Boolean(_) => {
                let rhs = rhs.coerce_to_boolean(it);
                Self::identical(it, lhs, &rhs)
            }
            Self::Number(_) => {
                let rhs = rhs.coerce_to_number(it);
                Self::identical(it, lhs, &rhs)
            }
            Self::String(_) => {
                let rhs = rhs.coerce_to_string(it);
                Self::identical(it, lhs, &rhs)
            }
            Self::Reference(_) => Self::identical(it, lhs, rhs),
            Self::Null | Self::Undefined => {
                Self::Boolean(matches!(rhs, Self::Null | Self::Undefined))
            }
        }
    }

    pub fn ne(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        let eq = Self::identical(it, lhs, rhs);
        Self::not(it, &eq)
    }

    pub fn lt(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Boolean(numeric_bin_op(it, lhs, rhs, |lhs, rhs| lhs < rhs))
    }

    pub fn le(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Boolean(numeric_bin_op(it, lhs, rhs, |lhs, rhs| lhs <= rhs))
    }

    pub fn gt(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Boolean(numeric_bin_op(it, lhs, rhs, |lhs, rhs| lhs > rhs))
    }

    pub fn ge(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Boolean(numeric_bin_op(it, lhs, rhs, |lhs, rhs| lhs >= rhs))
    }

    pub fn bitwise_not(it: &mut Interpreter, operand: &Self) -> Self {
        Self::Number(numeric_uni_op(it, operand, i64::not))
    }

    pub fn bitwise_and(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(numeric_bin_op(it, lhs, rhs, i64::bitand))
    }

    pub fn bitwise_or(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(numeric_bin_op(it, lhs, rhs, i64::bitor))
    }

    pub fn bitwise_xor(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(numeric_bin_op(it, lhs, rhs, i64::bitxor))
    }

    pub fn bitwise_shl(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(numeric_bin_op(it, lhs, rhs, i64::shl))
    }

    pub fn bitwise_shr(_it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        todo!("Interpreter::bitwise_shr: lhs={:?}, rhs={:?}", lhs, rhs)
    }

    pub fn bitwise_shrr(_it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        todo!("Interpreter::bitwise_shrr: lhs={:?}, rhs={:?}", lhs, rhs)
    }

    pub fn plus(it: &mut Interpreter, operand: &Self) -> Self {
        Self::Number(numeric_uni_op(it, operand, |operand| operand))
    }

    pub fn neg(it: &mut Interpreter, operand: &Self) -> Self {
        match numeric_uni_op(it, operand, i64::checked_neg) {
            Some(result) => Self::Number(result),
            None => todo!("Value::neg: -{:?} => Overflow", operand),
        }
    }

    pub fn not(it: &mut Interpreter, operand: &Self) -> Self {
        Self::Boolean(boolean_uni_op(it, operand, bool::not))
    }

    pub fn is_truthy(&self, it: &mut Interpreter) -> bool {
        self.coerce_to_boolean_impl(it)
    }

    pub fn is_falsy(&self, it: &mut Interpreter) -> bool {
        !self.coerce_to_boolean_impl(it)
    }

    pub fn coerce_to_boolean(&self, it: &mut Interpreter) -> Self {
        Self::Boolean(self.coerce_to_boolean_impl(it))
    }

    fn coerce_to_boolean_impl(&self, _it: &mut Interpreter) -> bool {
        match self {
            Self::Boolean(value) => *value,
            Self::Number(value) => *value > 0,
            Self::String(value) => !value.is_empty(),
            Self::Reference(_) => true,
            Self::Null | Self::Undefined => false,
        }
    }

    pub fn coerce_to_number(&self, it: &mut Interpreter) -> Self {
        Self::Number(self.coerce_to_number_impl(it))
    }

    fn coerce_to_number_impl(&self, _it: &mut Interpreter) -> i64 {
        match self {
            Self::Boolean(true) => 1,
            Self::Boolean(false) => 0,
            Self::Number(value) => *value,
            Self::String(_) | Self::Reference(_) | Self::Null | Self::Undefined => {
                todo!("Value::coerce_to_number: {:?}", self)
            }
        }
    }

    pub fn coerce_to_string(&self, it: &mut Interpreter) -> Self {
        Self::String(self.coerce_to_string_impl(it))
    }

    fn coerce_to_string_impl(&self, it: &mut Interpreter) -> String {
        match self {
            Self::Boolean(value) => value.to_string(),
            Self::Number(value) => value.to_string(),
            Self::String(value) => value.to_owned(),
            Self::Reference(obj_ref) => {
                let obj = it.vm().heap().resolve(obj_ref);
                obj.js_to_string()
            }
            Self::Null => "null".to_owned(),
            Self::Undefined => "undefined".to_owned(),
        }
    }
}

fn checked_pow(lhs: i64, rhs: i64) -> Option<i64> {
    if rhs >= (i32::MIN as i64) && rhs <= (i32::MAX as i64) {
        Some((lhs as f64).powi(rhs as i32) as i64)
    } else {
        None
    }
}

#[inline]
fn boolean_uni_op<R>(it: &mut Interpreter, operand: &Value, op: impl FnOnce(bool) -> R) -> R {
    let operand = operand.coerce_to_boolean_impl(it);
    op(operand)
}

#[inline]
fn numeric_uni_op<R>(it: &mut Interpreter, operand: &Value, op: impl FnOnce(i64) -> R) -> R {
    let operand = operand.coerce_to_number_impl(it);
    op(operand)
}

#[inline]
fn numeric_bin_op<R>(
    it: &mut Interpreter,
    lhs: &Value,
    rhs: &Value,
    op: impl FnOnce(i64, i64) -> R,
) -> R {
    let lhs = lhs.coerce_to_number_impl(it);
    let rhs = rhs.coerce_to_number_impl(it);
    op(lhs, rhs)
}
