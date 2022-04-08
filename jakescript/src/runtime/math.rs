use super::Builtin;
use crate::interpreter::{
    ErrorKind, Extensible, Heap, InitialisationError, Number, NumericOverflowError, Object,
    ObjectData, Property, Reference, Value, Writable,
};
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;
use std::f64::consts::*;

pub struct Math {
    obj_ref: Reference,
}

impl Builtin for Math {
    fn init(heap: &mut Heap) -> Result<Self, InitialisationError> {
        let abs = MathAbs::init(heap)?;
        let max = MathMax::init(heap)?;
        let min = MathMin::init(heap)?;
        let sqrt = MathSqrt::init(heap)?;
        let trunc = MathTrunc::init(heap)?;

        let props = hash_map![
            prop_key!("E") => Property::new(Value::Number(Number::Float(E)), Writable::No),
            prop_key!("LN2") => Property::new(Value::Number(Number::Float(LN_2)), Writable::No),
            prop_key!("LN10") => Property::new(Value::Number(Number::Float(LN_10)), Writable::No),
            prop_key!("LOG2E") => Property::new(Value::Number(Number::Float(LOG2_E)), Writable::No),
            prop_key!("LOG10E") => Property::new(
                Value::Number(Number::Float(LOG10_E)),
                Writable::No
            ),
            prop_key!("PI") => Property::new(Value::Number(Number::Float(PI)), Writable::No),
            prop_key!("SQRT1_2") => Property::new(
                Value::Number(Number::Float(FRAC_1_SQRT_2)),
                Writable::No
            ),
            prop_key!("SQRT2") => Property::new(Value::Number(Number::Float(SQRT_2)), Writable::No),

            prop_key!("abs") => Property::new(abs.as_value(), Writable::Yes),
            prop_key!("max") => Property::new(max.as_value(), Writable::Yes),
            prop_key!("min") => Property::new(min.as_value(), Writable::Yes),
            prop_key!("sqrt") => Property::new(sqrt.as_value(), Writable::Yes),
            prop_key!("trunc") => Property::new(trunc.as_value(), Writable::Yes),
        ];

        let obj_ref = heap.allocate(Object::new(None, props, ObjectData::None, Extensible::Yes))?;
        Ok(Self { obj_ref })
    }

    fn obj_ref(&self) -> &Reference {
        &self.obj_ref
    }
}

builtin_fn!(MathAbs, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first().cloned().unwrap_or_default();
    it.coerce_to_number(&arg)
        .checked_abs()
        .map(Value::Number)
        .ok_or(ErrorKind::NumericOverflow(NumericOverflowError))
});

builtin_fn!(MathMax, Extensible::Yes, (it, _receiver, args) => {
    let mut acc = Number::NEG_INF;
    for arg in args {
        let n = it.coerce_to_number(arg);
        if n.is_nan() {
            return Ok(Value::Number(Number::NAN));
        }
        if n > acc {
            acc = n;
        }
    }
    Ok(Value::Number(acc))
});

builtin_fn!(MathMin, Extensible::Yes, (it, _receiver, args) => {
    let mut acc = Number::POS_INF;
    for arg in args {
        let n = it.coerce_to_number(arg);
        if n.is_nan() {
            return Ok(Value::Number(Number::NAN));
        }
        if n < acc {
            acc = n;
        }
    }
    Ok(Value::Number(acc))
});

builtin_fn!(MathSqrt, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first().cloned().unwrap_or_default();
    Ok(Value::Number(it.coerce_to_number(&arg).sqrt()))
});

builtin_fn!(MathTrunc, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first().cloned().unwrap_or_default();
    let n = it.coerce_to_number(&arg);
    Ok(Value::Number(if n.is_finite() {
        Number::Int(n.as_i64())
    } else {
        n
    }))
});
