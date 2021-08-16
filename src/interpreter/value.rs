use crate::interpreter::heap::Reference;
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
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(ref value) => *value,
            Value::Number(ref value) => *value > 0,
            Value::String(ref value) => !value.is_empty(),
            Value::Reference(_) => true,
            Value::Null | Value::Undefined => false,
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }
}

impl super::Interpreter {
    pub fn add(&mut self, lhs: &Value, rhs: &Value) -> Value {
        if matches!(lhs, Value::String(_)) {
            todo!("Interpreter::add: lhs={:?}, rhs={:?}", lhs, rhs)
        }
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::add)
    }

    pub fn sub(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::sub)
    }

    pub fn mul(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::mul)
    }

    pub fn div(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::div)
    }

    pub fn rem(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::rem)
    }

    pub fn pow(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Number, |lhs, rhs| {
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
            Value::Number(_) => {
                let rhs = self.coerce_to_numeric(rhs);
                self.identical(lhs, &rhs)
            }
            Value::String(_) => {
                let rhs = self.coerce_to_string(rhs);
                self.identical(lhs, &rhs)
            }
            Value::Reference(_) => self.identical(lhs, rhs),
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
            (Value::Number(ref lhs), Value::Number(ref rhs)) => lhs == rhs,
            (Value::String(ref lhs), Value::String(ref rhs)) => lhs == rhs,
            (Value::Reference(ref lhs_refr), Value::Reference(ref rhs_refr)) => {
                let lhs_obj = self.vm().heap().resolve(lhs_refr);
                let rhs_obj = self.vm().heap().resolve(rhs_refr);
                lhs_obj.deref().js_equals(rhs_obj.deref())
            }
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
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::bitand)
    }

    pub fn bitwise_or(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::bitor)
    }

    pub fn bitwise_xor(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::bitxor)
    }

    pub fn bitwise_shl(&mut self, lhs: &Value, rhs: &Value) -> Value {
        self.numeric_bin_op(lhs, rhs, Value::Number, i64::shl)
    }

    pub fn bitwise_shr(&mut self, lhs: &Value, rhs: &Value) -> Value {
        todo!("Interpreter::bitwise_shr: lhs={:?}, rhs={:?}", lhs, rhs)
    }

    pub fn bitwise_shrr(&mut self, lhs: &Value, rhs: &Value) -> Value {
        todo!("Interpreter::bitwise_shrr: lhs={:?}, rhs={:?}", lhs, rhs)
    }

    pub fn not(&mut self, operand: &Value) -> Value {
        Value::Boolean(operand.is_falsy())
    }

    pub fn plus(&mut self, operand: &Value) -> Value {
        self.numeric_uni_op(operand, Value::Number, |operand| operand)
    }

    pub fn neg(&mut self, operand: &Value) -> Value {
        self.numeric_uni_op(operand, Value::Number, i64::neg)
    }

    pub fn coerce_to_boolean(&mut self, value: &Value) -> Value {
        Value::Boolean(value.is_truthy())
    }

    pub fn coerce_to_numeric(&mut self, value: &Value) -> Value {
        Value::Number(self.coerce_to_numeric_impl(value))
    }

    pub fn coerce_to_string(&mut self, value: &Value) -> Value {
        Value::String(self.coerce_to_string_impl(value))
    }

    fn coerce_to_numeric_impl(&mut self, value: &Value) -> i64 {
        match value {
            Value::Boolean(true) => 1,
            Value::Boolean(false) => 0,
            Value::Number(ref value) => *value,
            Value::String(_) | Value::Reference(_) | Value::Null | Value::Undefined => {
                todo!("Interpreter::coerce_to_numeric: {:?}", value)
            }
        }
    }

    fn coerce_to_string_impl(&mut self, value: &Value) -> String {
        match value {
            Value::Boolean(ref value) => value.to_string(),
            Value::Number(ref value) => value.to_string(),
            Value::String(ref value) => value.to_owned(),
            Value::Reference(ref refr) => {
                let value = self.vm().heap().resolve(refr);
                value.deref().js_to_string()
            }
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
