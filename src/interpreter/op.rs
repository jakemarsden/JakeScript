use crate::ast::Value;
use std::ops::*;

impl super::Interpreter {
    pub fn add(&mut self, lhs: &Value, rhs: &Value) -> Value {
        if matches!(lhs, Value::String(_)) {
            todo!("Interpreter::add: lhs={}, rhs={}", lhs, rhs)
        }
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::add)
    }

    pub fn sub(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::sub)
    }

    pub fn mul(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::mul)
    }

    pub fn div(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::div)
    }

    pub fn rem(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::rem)
    }

    pub fn pow(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, |lhs, rhs| {
            assert!(rhs >= (i32::MIN as i64));
            assert!(rhs <= (i32::MAX as i64));
            (lhs as f64).powi(rhs as i32) as i64
        })
    }

    pub fn eq(&mut self, lhs: &Value, rhs: &Value) -> Value {
        match lhs {
            Value::Boolean(_) => {
                let rhs = self.coerce_to_boolean(rhs);
                self.identical(lhs, &rhs)
            }
            Value::Numeric(_) => {
                let rhs = self.coerce_to_numeric(rhs);
                self.identical(lhs, &rhs)
            }
            Value::String(_) => {
                let rhs = self.coerce_to_string(rhs);
                self.identical(lhs, &rhs)
            }
            Value::Null | Value::Undefined => {
                Value::Boolean(matches!(rhs, Value::Null | Value::Undefined))
            }
        }
    }

    pub fn ne(&mut self, lhs: &Value, rhs: &Value) -> Value {
        Value::Boolean(self.eq(lhs, rhs).is_falsy())
    }

    pub fn identical(&mut self, lhs: &Value, rhs: &Value) -> Value {
        Value::Boolean(match (lhs, rhs) {
            (Value::Boolean(ref lhs), Value::Boolean(ref rhs)) => lhs == rhs,
            (Value::Numeric(ref lhs), Value::Numeric(ref rhs)) => lhs == rhs,
            (Value::String(ref lhs), Value::String(ref rhs)) => lhs == rhs,
            (Value::Null, Value::Null) => true,
            (Value::Undefined, Value::Undefined) => true,
            (_, _) => false,
        })
    }

    pub fn not_identical(&mut self, lhs: &Value, rhs: &Value) -> Value {
        Value::Boolean(self.identical(lhs, rhs).is_falsy())
    }

    pub fn lt(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Boolean, |lhs, rhs| lhs < rhs)
    }

    pub fn le(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Boolean, |lhs, rhs| lhs <= rhs)
    }

    pub fn gt(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Boolean, |lhs, rhs| lhs > rhs)
    }

    pub fn ge(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Boolean, |lhs, rhs| lhs >= rhs)
    }

    pub fn bitwise_and(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::bitand)
    }

    pub fn bitwise_or(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::bitor)
    }

    pub fn bitwise_xor(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::bitxor)
    }

    pub fn bitwise_shl(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Numeric, i64::shl)
    }

    pub fn bitwise_shr(&mut self, lhs: &Value, rhs: &Value) -> Value {
        todo!("Interpreter::bitwise_shr: lhs={}, rhs={}", lhs, rhs)
    }

    pub fn bitwise_shrr(&mut self, lhs: &Value, rhs: &Value) -> Value {
        todo!("Interpreter::bitwise_shrr: lhs={}, rhs={}", lhs, rhs)
    }

    pub fn not(&mut self, operand: &Value) -> Value {
        Value::Boolean(operand.is_falsy())
    }

    pub fn plus(&mut self, operand: &Value) -> Value {
        self.numeric_uni_op(operand, Value::Numeric, |operand| operand)
    }

    pub fn neg(&mut self, operand: &Value) -> Value {
        self.numeric_uni_op(operand, Value::Numeric, i64::neg)
    }

    pub fn coerce_to_boolean(&mut self, value: &Value) -> Value {
        Value::Boolean(value.is_truthy())
    }

    pub fn coerce_to_numeric(&mut self, value: &Value) -> Value {
        Value::Numeric(self.coerce_to_numeric_impl(value))
    }

    pub fn coerce_to_string(&mut self, value: &Value) -> Value {
        Value::String(self.coerce_to_string_impl(value))
    }

    fn coerce_to_numeric_impl(&mut self, value: &Value) -> i64 {
        match value {
            Value::Boolean(true) => 1,
            Value::Boolean(false) => 0,
            Value::Numeric(ref value) => *value,
            Value::String(_) | Value::Null | Value::Undefined => {
                todo!("Interpreter::coerce_to_numeric: {}", value)
            }
        }
    }

    fn coerce_to_string_impl(&mut self, value: &Value) -> String {
        match value {
            Value::Boolean(ref value) => value.to_string(),
            Value::Numeric(ref value) => value.to_string(),
            Value::String(ref value) => value.to_owned(),
            Value::Null => "null".to_owned(),
            Value::Undefined => "undefined".to_owned(),
        }
    }

    #[inline]
    fn numeric_uni_op<R>(
        &mut self,
        operand: &Value,
        to_value: impl FnOnce(R) -> Value,
        op: impl FnOnce(i64) -> R,
    ) -> Value {
        let operand = self.coerce_to_numeric_impl(operand);
        to_value(op(operand))
    }

    #[inline]
    fn numeric_bin_op<R>(
        &mut self,
        lhs: &Value,
        rhs: &Value,
        to_value: impl FnOnce(R) -> Value,
        op: impl FnOnce(i64, i64) -> R,
    ) -> Value {
        let lhs = self.coerce_to_numeric_impl(lhs);
        let rhs = self.coerce_to_numeric_impl(rhs);
        to_value(op(lhs, rhs))
    }
}