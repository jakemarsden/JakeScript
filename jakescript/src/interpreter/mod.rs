pub use error::*;
pub use heap::*;
pub use stack::*;
pub use value::*;
pub use vm::*;

use crate::ast::Node;
use std::cmp;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::str::FromStr;

mod block;
mod declaration;
mod error;
mod expression;
mod heap;
mod literal;
mod stack;
mod statement;
mod value;
mod vm;

pub trait Eval: Node {
    type Output = ();

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output>;
}

pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn new(vm: Vm) -> Self {
        Self { vm }
    }

    pub fn vm(&self) -> &Vm {
        &self.vm
    }
    pub fn vm_mut(&mut self) -> &mut Vm {
        &mut self.vm
    }

    pub fn add_or_concat(
        &mut self,
        lhs: &Value,
        rhs: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        if lhs.is_str_or_ref() || rhs.is_str_or_ref() {
            self.concat(lhs, rhs)
        } else {
            self.add(lhs, rhs)
        }
    }

    fn add(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_add)
    }

    // unnecessary_wraps: Future-proofing
    #[allow(clippy::unnecessary_wraps)]
    fn concat(&mut self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        let mut out = self.coerce_to_string(lhs);
        out.push_str(&self.coerce_to_string(rhs));
        Ok(Value::String(out))
    }

    pub fn sub(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_sub)
    }

    pub fn mul(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_mul)
    }

    pub fn div(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_div)
    }

    pub fn rem(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_rem)
    }

    pub fn pow(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_binary_op(lhs, rhs, Number::pow)
    }

    pub fn bitand(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_binary_op(lhs, rhs, Number::bitand)
    }

    pub fn bitor(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_binary_op(lhs, rhs, Number::bitor)
    }

    pub fn bitxor(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_binary_op(lhs, rhs, Number::bitxor)
    }

    pub fn shl(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_shl)
    }

    pub fn shr_signed(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_shr_signed)
    }

    pub fn shr_unsigned(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_shr_unsigned)
    }

    pub fn equal(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Boolean(match lhs {
            Value::Boolean(lhs) => *lhs == self.coerce_to_bool(rhs),
            Value::Number(lhs) => *lhs == self.coerce_to_number(rhs),
            Value::Object(lhs) => {
                if let Value::Object(rhs) = rhs {
                    lhs == rhs
                } else {
                    false
                }
            }
            Value::String(lhs) => lhs == &self.coerce_to_string(rhs),
            Value::Null | Value::Undefined => matches!(rhs, Value::Null | Value::Undefined),
        }))
    }

    pub fn not_equal(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.equal(lhs, rhs).and_then(|ref result| self.not(result))
    }

    // unused_self: Will be used when string values are stored on the heap.
    #[allow(clippy::unused_self)]
    pub fn strictly_equal(
        &self,
        lhs: &Value,
        rhs: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Boolean(match (lhs, rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => lhs == rhs,
            (Value::Number(lhs), Value::Number(rhs)) => lhs == rhs,
            (Value::Object(lhs), Value::Object(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::Null, Value::Null) | (Value::Undefined, Value::Undefined) => true,
            (_, _) => false,
        }))
    }

    pub fn not_strictly_equal(
        &self,
        lhs: &Value,
        rhs: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        self.strictly_equal(lhs, rhs)
            .and_then(|ref result| self.not(result))
    }

    pub fn gt(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.comparison_op(lhs, rhs, cmp::Ordering::is_gt)
    }

    pub fn ge(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.comparison_op(lhs, rhs, cmp::Ordering::is_ge)
    }

    pub fn lt(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.comparison_op(lhs, rhs, cmp::Ordering::is_lt)
    }

    pub fn le(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.comparison_op(lhs, rhs, cmp::Ordering::is_le)
    }

    pub fn plus(&self, operand: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_unary_op(operand, |operand| operand)
    }

    pub fn negate(&self, operand: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_unary_op(operand, Number::checked_neg)
    }

    pub fn bitnot(&self, operand: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_unary_op(operand, Number::not)
    }

    pub fn not(&self, operand: &Value) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Boolean(!self.coerce_to_bool(operand)))
    }

    pub fn is_truthy(&self, v: &Value) -> bool {
        self.coerce_to_bool(v)
    }

    // unused_self: Will be used when string values are stored on the heap.
    #[allow(clippy::unused_self)]
    pub fn coerce_to_bool(&self, v: &Value) -> bool {
        match v {
            Value::Boolean(v) => *v,
            Value::Number(v) => !v.is_zero() && !v.is_nan(),
            Value::Object(_) => true,
            Value::String(v) => !v.is_empty(),
            Value::Null | Value::Undefined => false,
        }
    }

    // unused_self: Will be used when string values are stored on the heap.
    #[allow(clippy::unused_self)]
    pub fn coerce_to_number(&self, v: &Value) -> Number {
        match v {
            Value::Boolean(v) => Number::Int(if *v { 1 } else { 0 }),
            Value::Number(v) => *v,
            Value::String(v) => Number::from_str(v).unwrap_or(Number::NAN),
            Value::Null => Number::Int(0),
            Value::Object(_) | Value::Undefined => Number::NAN,
        }
    }

    pub fn coerce_to_string(&self, v: &Value) -> String {
        match v {
            Value::Boolean(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            Value::Object(obj_ref) => {
                let obj = self.vm().heap().resolve(obj_ref);
                obj.js_to_string()
            }
            Value::String(v) => v.clone(),
            Value::Null => "null".to_owned(),
            Value::Undefined => "undefined".to_owned(),
        }
    }

    #[inline]
    fn checked_numeric_unary_op(
        &self,
        operand: &Value,
        checked_op: impl FnOnce(Number) -> Option<Number>,
    ) -> std::result::Result<Value, ErrorKind> {
        checked_op(self.coerce_to_number(operand))
            .map(Value::Number)
            .ok_or(ErrorKind::NumericOverflow(NumericOverflowError))
    }

    // unnecessary_wraps: Future-proofing
    #[allow(clippy::unnecessary_wraps)]
    #[inline]
    fn numeric_unary_op(
        &self,
        operand: &Value,
        op: impl FnOnce(Number) -> Number,
    ) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Number(op(self.coerce_to_number(operand))))
    }

    #[inline]
    fn checked_numeric_binary_op(
        &self,
        lhs: &Value,
        rhs: &Value,
        checked_op: impl FnOnce(Number, Number) -> Option<Number>,
    ) -> std::result::Result<Value, ErrorKind> {
        checked_op(self.coerce_to_number(lhs), self.coerce_to_number(rhs))
            .map(Value::Number)
            .ok_or(ErrorKind::NumericOverflow(NumericOverflowError))
    }

    // unnecessary_wraps: Future-proofing
    #[allow(clippy::unnecessary_wraps)]
    #[inline]
    fn numeric_binary_op(
        &self,
        lhs: &Value,
        rhs: &Value,
        op: impl FnOnce(Number, Number) -> Number,
    ) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Number(op(
            self.coerce_to_number(lhs),
            self.coerce_to_number(rhs),
        )))
    }

    // unnecessary_wraps: Future-proofing
    #[allow(clippy::unnecessary_wraps)]
    fn comparison_op(
        &self,
        lhs: &Value,
        rhs: &Value,
        op: impl FnOnce(cmp::Ordering) -> bool,
    ) -> std::result::Result<Value, ErrorKind> {
        let ord = if lhs.is_str_or_ref() || rhs.is_str_or_ref() {
            Some(self.coerce_to_string(lhs).cmp(&self.coerce_to_string(rhs)))
        } else {
            self.coerce_to_number(lhs)
                .partial_cmp(&self.coerce_to_number(rhs))
        };
        Ok(Value::Boolean(ord.map_or(false, op)))
    }
}
