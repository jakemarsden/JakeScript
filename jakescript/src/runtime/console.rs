use super::Builtin;
use crate::interpreter::{
    AssertionError, ErrorKind, Extensible, Heap, InitialisationError, Interpreter, Object,
    ObjectData, Property, Reference, Value,
};
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;

pub struct ConsoleBuiltin {
    obj_ref: Reference,
}

impl Builtin for ConsoleBuiltin {
    type InitArgs = (Reference, Reference);

    fn init(
        heap: &mut Heap,
        (obj_proto, fn_proto): Self::InitArgs,
    ) -> Result<Self, InitialisationError> {
        let assert = AssertBuiltin::init(heap, fn_proto)?;
        let assert_equal = AssertEqualBuiltin::init(heap, fn_proto)?;
        let assert_not_reached = AssertNotReachedBuiltin::init(heap, fn_proto)?;
        let log = LogBuiltin::init(heap, fn_proto)?;

        let props = hash_map![
            prop_key!("assert") => Property::new_user(assert.as_value()),
            prop_key!("assertEqual") => Property::new_user(assert_equal.as_value()),
            prop_key!("assertNotReached") => Property::new_user(assert_not_reached.as_value()),
            prop_key!("log") => Property::new_user(log.as_value()),
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

builtin_fn!(AssertBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let mut args = args.iter();
    let assertion = args.next().copied().unwrap_or(Value::Undefined);
    if it.is_truthy(assertion) {
        Ok(Value::Undefined)
    } else {
        let detail_msg = build_msg(it, args);
        Err(ErrorKind::from(AssertionError::new(detail_msg)))
    }
});

builtin_fn!(AssertEqualBuiltin, Extensible::Yes, (it, _receiver, args) => {
    fn is_nan(v: Value) -> bool {
        matches!(v, Value::Number(n) if n.is_nan())
    }

    let mut args = args.iter();
    let actual = args.next().copied().unwrap_or(Value::Undefined);
    let expected = args.next().copied().unwrap_or(Value::Boolean(true));
    if is_nan(expected) && is_nan(actual) || it.is_truthy(it.strictly_equal(actual, expected)?)
    {
        Ok(Value::Undefined)
    } else {
        let detail_msg = format!(
            "expected '{}' but was '{}': {}",
            it.coerce_to_string(expected),
            it.coerce_to_string(actual),
            build_msg(it, args)
        );
        Err(ErrorKind::from(AssertionError::new(detail_msg)))
    }
});

builtin_fn!(AssertNotReachedBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let detail_msg = format!("entered unreachable code: {}", build_msg(it, args.iter()));
    Err(ErrorKind::from(AssertionError::new(detail_msg)))
});

builtin_fn!(LogBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let msg = build_msg(it, args.iter());
    it.vm_mut().write_message(&msg);
    Ok(Value::Undefined)
});

fn build_msg<'a>(it: &Interpreter, values: impl Iterator<Item = &'a Value>) -> String {
    values
        .map(|&arg| it.coerce_to_string(arg))
        .intersperse_with(|| Box::from(" "))
        .collect()
}
