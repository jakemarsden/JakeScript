use super::{register_builtin, Builtin};
use crate::interpreter::{
    ErrorKind, Heap, InitialisationError, Interpreter, Number, NumericOverflowError, Object,
    Property, Reference, Value,
};
use crate::non_empty_str;
use common_macros::hash_map;
use std::f64::consts::*;

pub struct Math;
pub struct MathAbs;
pub struct MathMax;
pub struct MathMin;
pub struct MathSqrt;
pub struct MathTrunc;

impl Builtin for Math {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let properties = hash_map![
            non_empty_str!("E")
                => Property::new(false, Value::Number(Number::Float(E))),
            non_empty_str!("LN2")
                => Property::new(false, Value::Number(Number::Float(LN_2))),
            non_empty_str!("LN10")
                => Property::new(false, Value::Number(Number::Float(LN_10))),
            non_empty_str!("LOG2E")
                => Property::new(false, Value::Number(Number::Float(LOG2_E))),
            non_empty_str!("LOG10E")
                => Property::new(false, Value::Number(Number::Float(LOG10_E))),
            non_empty_str!("PI")
                => Property::new(false, Value::Number(Number::Float(PI))),
            non_empty_str!("SQRT1_2")
                => Property::new(false, Value::Number(Number::Float(FRAC_1_SQRT_2))),
            non_empty_str!("SQRT2")
                => Property::new(false, Value::Number(Number::Float(SQRT_2))),

            non_empty_str!("abs")
                => Property::new(true, Value::Object(MathAbs::register(heap)?)),
            non_empty_str!("max")
                => Property::new(true, Value::Object(MathMax::register(heap)?)),
            non_empty_str!("min")
                => Property::new(true, Value::Object(MathMin::register(heap)?)),
            non_empty_str!("sqrt")
                => Property::new(true, Value::Object(MathSqrt::register(heap)?)),
            non_empty_str!("trunc")
                => Property::new(true, Value::Object(MathTrunc::register(heap)?)),
        ];
        let obj = Object::new_builtin(true, properties, None);
        register_builtin(heap, obj)
    }
}

impl MathAbs {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        let n = args.first().cloned().unwrap_or_default().coerce_to_number();
        Ok(Value::Number(n.checked_abs().ok_or(NumericOverflowError)?))
    }
}

impl Builtin for MathAbs {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

impl MathMax {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        let mut acc = Number::NEG_INF;
        for arg in args {
            let n = arg.coerce_to_number();
            if n.is_nan() {
                return Ok(Value::Number(Number::NAN));
            }
            if n > acc {
                acc = n;
            }
        }
        Ok(Value::Number(acc))
    }
}

impl Builtin for MathMax {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

impl MathMin {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        let mut acc = Number::POS_INF;
        for arg in args {
            let n = arg.coerce_to_number();
            if n.is_nan() {
                return Ok(Value::Number(Number::NAN));
            }
            if n < acc {
                acc = n;
            }
        }
        Ok(Value::Number(acc))
    }
}

impl Builtin for MathMin {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

impl MathSqrt {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        let n = args.first().cloned().unwrap_or_default().coerce_to_number();
        Ok(Value::Number(n.sqrt()))
    }
}

impl Builtin for MathSqrt {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

impl MathTrunc {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        let n = args.first().cloned().unwrap_or_default().coerce_to_number();
        Ok(Value::Number(if n.is_finite() {
            Number::Int(n.as_i64())
        } else {
            n
        }))
    }
}

impl Builtin for MathTrunc {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}
