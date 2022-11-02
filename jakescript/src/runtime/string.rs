use super::Builtin;
use crate::interpreter::{
    ErrorKind, Extensible, Heap, InitialisationError, Number, Object, ObjectData, Property,
    Reference, Value,
};
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;
use std::mem;

pub struct StringProtoBuiltin {
    obj_ref: Reference,
}

impl Builtin for StringProtoBuiltin {
    type InitArgs = (Reference, Reference);

    fn init(
        heap: &mut Heap,
        (obj_proto, fn_proto): Self::InitArgs,
    ) -> Result<Self, InitialisationError> {
        let length = GetLengthBuiltin::init(heap, fn_proto)?;
        let char_at = CharAtBuiltin::init(heap, fn_proto)?;
        let split = SplitBuiltin::init(heap, fn_proto)?;
        let substring = SubstringBuiltin::init(heap, fn_proto)?;

        let props = hash_map![
            prop_key!("length") => Property::new_const_accessor(length.obj_ref()),
            prop_key!("charAt") => Property::new_user(char_at.as_value()),
            prop_key!("split") => Property::new_user(split.as_value()),
            prop_key!("substring") => Property::new_user(substring.as_value()),
        ];

        let obj_ref = heap.allocate(Object::new(
            Some(obj_proto),
            props,
            ObjectData::None,
            Extensible::Yes,
        ))?;
        Ok(Self { obj_ref })
    }

    fn obj_ref(&self) -> Reference {
        self.obj_ref
    }
}

builtin_fn!(pub StringCtorBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first();
    let str = arg.map(|arg| it.coerce_to_string(*arg)).unwrap_or_default();
    it.alloc_string(str)
        .map(Value::Object)
        .map_err(ErrorKind::from)
});

builtin_fn!(GetLengthBuiltin, Extensible::No, (it, receiver, _args) => {
    let receiver = it.vm().heap().resolve(receiver);
    let length = receiver.as_ref().string_data().unwrap().len();
    let length = Number::try_from(length).unwrap_or_else(|_| {
        // TODO
        unreachable!()
    });
    Ok(Value::Number(length))
});

builtin_fn!(CharAtBuiltin, Extensible::Yes, (it, receiver, args) => {
    let arg = args.first().copied().unwrap_or_default();
    let idx = {
        let n = it.coerce_to_number(arg);
        if !n.is_nan() {
            n
        } else {
            Number::Int(0)
        }
    };
    let char_str = if idx >= Number::Int(0) {
        let idx = usize::try_from(idx.as_i64()).unwrap();
        it.coerce_to_string(Value::Object(receiver))
            .chars()
            .nth(idx)
            .map(|ch| ch.to_string().into_boxed_str())
            .unwrap_or_default()
    } else {
        Box::default()
    };
    it.alloc_string(char_str)
        .map(Value::Object)
        .map_err(ErrorKind::from)
});

builtin_fn!(SplitBuiltin, Extensible::Yes, (it, receiver, args) => {
    let receiver = it.coerce_to_string(Value::Object(receiver));
    let mut args = args.iter();
    let separator = if let Some(&arg) = args.next() {
        it.coerce_to_string(arg)
    } else {
        Box::from(",")
    };
    let limit = if let Some(&arg) = args.next() {
        it.coerce_to_number(arg)
    } else {
        Number::from(-1)
    };
    let limit = if !limit.is_negative() {
        usize::try_from(limit.as_i64()).expect("overflow")
    } else {
        usize::MAX
    };

    let mut parts = Vec::new();
    for part in receiver.split(separator.as_ref()).take(limit) {
        let part = it.alloc_string(Box::from(part))?;
        parts.push(Value::Object(part));
    }
    it.alloc_array(parts).map(Value::Object).map_err(ErrorKind::from)
});

builtin_fn!(SubstringBuiltin, Extensible::Yes, (it, receiver, args) => {
    let mut args = args.iter();
    let start_idx = args.next().copied().unwrap_or_default();
    let end_idx = args.next().copied().unwrap_or_default();

    let str = it.coerce_to_string(Value::Object(receiver));
    let mut start_idx = match it.coerce_to_number(start_idx) {
        n if n.is_nan() => 0,
        n if n < Number::Int(0) => 0,
        n if n > Number::Int(str.len() as i64) => str.len(),
        n => usize::try_from(n.as_i64()).unwrap_or_else(|_| unreachable!()),
    };
    let mut end_idx = match it.coerce_to_number(end_idx) {
        n if n.is_nan() => str.len(),
        n if n < Number::Int(0) => 0,
        n if n > Number::Int(str.len() as i64) => str.len(),
        n => usize::try_from(n.as_i64()).unwrap_or_else(|_| unreachable!()),
    };
    if start_idx > end_idx {
        mem::swap(&mut start_idx, &mut end_idx);
    }
    let substr = str
        .chars()
        .skip(start_idx)
        .take(end_idx - start_idx)
        .collect::<String>()
        .into_boxed_str();
    it.alloc_string(substr)
        .map(Value::Object)
        .map_err(ErrorKind::from)
});
