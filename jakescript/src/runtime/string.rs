use super::Builtin;
use crate::interpreter::{
    ErrorKind, Extensible, Heap, InitialisationError, Interpreter, Number, Object, Property,
    Reference, Value, Writable,
};
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;
use std::mem;

pub struct String {
    obj_ref: Reference,
}

impl String {
    fn call(it: &mut Interpreter, _: Reference, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        let str = arg.map_or_else(std::string::String::default, |arg| it.coerce_to_string(arg));
        it.alloc_string(str)
            .map(Value::Object)
            .map_err(ErrorKind::from)
    }

    // needless_pass_by_value, unnecessary_wraps: Required to conform to `NativeGet`.
    #[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
    fn length(it: &Interpreter, receiver: Reference) -> Result<Value, ErrorKind> {
        let receiver = it.vm().heap().resolve(&receiver);
        let length = receiver.string_data().unwrap().len();
        let length = Number::try_from(length).unwrap_or_else(|_| {
            // TODO
            unreachable!()
        });
        Ok(Value::Number(length))
    }

    // unnecessary_wraps: Required to conform to `NativeSet`.
    #[allow(clippy::unnecessary_wraps)]
    fn set_length(_: &mut Interpreter, _: Reference, _: Value) -> Result<(), ErrorKind> {
        Ok(())
    }
}

impl Builtin for String {
    fn init(heap: &mut Heap) -> Result<Self, InitialisationError> {
        let char_at = StringCharAt::init(heap)?;
        let substring = StringSubstring::init(heap)?;

        let props = hash_map![
            prop_key!("charAt") => Property::new(char_at.as_value(), Writable::Yes),
            prop_key!("length") => Property::new_native(
                &Self::length,
                &Self::set_length,
                Writable::No,
            ),
            prop_key!("substring") => Property::new(substring.as_value(), Writable::Yes),
        ];

        let obj_ref = heap.allocate(Object::new_native(
            None,
            props,
            &Self::call,
            Extensible::Yes,
        ))?;
        Ok(Self { obj_ref })
    }

    fn obj_ref(&self) -> &Reference {
        &self.obj_ref
    }
}

builtin_fn!(StringCharAt, Extensible::Yes, (it, _receiver, args) => {
    let mut args = args.iter();
    // TODO: Implement `this` expressions, add a `receiver` parameter to `NativeFn`.
    let receiver = args.next().cloned().unwrap_or_default();
    let arg = args.next().cloned().unwrap_or_default();
    let idx = {
        let n = it.coerce_to_number(&arg);
        if !n.is_nan() {
            n
        } else {
            Number::Int(0)
        }
    };
    let char_str = if idx >= Number::Int(0) {
        let idx = usize::try_from(idx.as_i64()).unwrap();
        it.coerce_to_string(&receiver)
            .chars()
            .nth(idx)
            .map(|ch| ch.to_string())
            .unwrap_or_default()
    } else {
        std::string::String::default()
    };
    it.alloc_string(char_str)
        .map(Value::Object)
        .map_err(ErrorKind::from)
});

builtin_fn!(StringSubstring, Extensible::Yes, (it, _receiver, args) => {
    let mut args = args.iter();
    // TODO: Implement `this` expressions, add a `receiver` parameter to `NativeFn`.
    let receiver = args.next().cloned().unwrap_or_default();
    let start_idx = args.next().cloned().unwrap_or_default();
    let end_idx = args.next().cloned().unwrap_or_default();

    let str = it.coerce_to_string(&receiver);
    let mut start_idx = match it.coerce_to_number(&start_idx) {
        n if n.is_nan() => 0,
        n if n < Number::Int(0) => 0,
        n if n > Number::Int(str.len() as i64) => str.len(),
        n => usize::try_from(n.as_i64()).unwrap_or_else(|_| unreachable!()),
    };
    let mut end_idx = match it.coerce_to_number(&end_idx) {
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
        .collect();
    it.alloc_string(substr)
        .map(Value::Object)
        .map_err(ErrorKind::from)
});
