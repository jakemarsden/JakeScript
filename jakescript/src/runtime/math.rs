use super::Builtin;
use crate::interpreter::{
    ErrorKind, Extensible, Heap, InitialisationError, Number, NumericOverflowError, Object,
    ObjectData, Property, Reference, Value,
};
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;
use std::f64::consts::*;

pub struct MathBuiltin {
    obj_ref: Reference,
}

impl Builtin for MathBuiltin {
    type InitArgs = (Reference, Reference);

    fn init(
        heap: &mut Heap,
        (obj_proto, fn_proto): Self::InitArgs,
    ) -> Result<Self, InitialisationError> {
        let abs = AbsBuiltin::init(heap, fn_proto.clone())?;
        let floor = FloorBuiltin::init(heap, fn_proto.clone())?;
        let max = MaxBuiltin::init(heap, fn_proto.clone())?;
        let min = MinBuiltin::init(heap, fn_proto.clone())?;
        let sqrt = SqrtBuiltin::init(heap, fn_proto.clone())?;
        let trunc = TruncBuiltin::init(heap, fn_proto)?;

        let props = hash_map![
            prop_key!("E") => Property::new_const(Value::Number(Number::Float(E))),
            prop_key!("LN2") => Property::new_const(Value::Number(Number::Float(LN_2))),
            prop_key!("LN10") => Property::new_const(Value::Number(Number::Float(LN_10))),
            prop_key!("LOG2E") => Property::new_const(Value::Number(Number::Float(LOG2_E))),
            prop_key!("LOG10E") => Property::new_const(Value::Number(Number::Float(LOG10_E))),
            prop_key!("PI") => Property::new_const(Value::Number(Number::Float(PI))),
            prop_key!("SQRT1_2") => Property::new_const(
                Value::Number(Number::Float(FRAC_1_SQRT_2))
            ),
            prop_key!("SQRT2") => Property::new_const(Value::Number(Number::Float(SQRT_2))),

            prop_key!("abs") => Property::new_user(abs.as_value()),
            prop_key!("floor") => Property::new_user(floor.as_value()),
            prop_key!("max") => Property::new_user(max.as_value()),
            prop_key!("min") => Property::new_user(min.as_value()),
            prop_key!("sqrt") => Property::new_user(sqrt.as_value()),
            prop_key!("trunc") => Property::new_user(trunc.as_value()),
        ];

        let obj_ref = heap.allocate(Object::new(
            Some(obj_proto),
            props,
            ObjectData::None,
            Extensible::Yes,
        ))?;
        Ok(Self { obj_ref })
    }

    fn obj_ref(&self) -> &Reference {
        &self.obj_ref
    }
}

builtin_fn!(AbsBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first().cloned().unwrap_or_default();
    it.coerce_to_number(&arg)
        .checked_abs()
        .map(Value::Number)
        .ok_or(ErrorKind::NumericOverflow(NumericOverflowError))
});

builtin_fn!(FloorBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first().cloned().unwrap_or_default();
    Ok(Value::Number(match it.coerce_to_number(&arg) {
        n if !n.is_finite() => n,
        Number::Int(n) => Number::Int(n),
        #[allow(clippy::cast_possible_truncation)]
        Number::Float(n) => Number::Int(n.floor() as i64),
    }))
});

builtin_fn!(MaxBuiltin, Extensible::Yes, (it, _receiver, args) => {
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

builtin_fn!(MinBuiltin, Extensible::Yes, (it, _receiver, args) => {
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

builtin_fn!(SqrtBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first().cloned().unwrap_or_default();
    Ok(Value::Number(it.coerce_to_number(&arg).sqrt()))
});

builtin_fn!(TruncBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first().cloned().unwrap_or_default();
    let n = it.coerce_to_number(&arg);
    Ok(Value::Number(if n.is_finite() {
        Number::Int(n.as_i64())
    } else {
        n
    }))
});
