use super::Builtin;
use crate::interpreter::{
    AssertionError, ErrorKind, Extensible, Heap, InitialisationError, Interpreter, Object,
    ObjectData, Property, Reference, Value, Writable,
};
use crate::{builtin_fn, non_empty_str, prop_key};
use common_macros::hash_map;

pub struct Console {
    obj_ref: Reference,
}

impl Builtin for Console {
    fn init(heap: &mut Heap) -> Result<Self, InitialisationError> {
        let assert = ConsoleAssert::init(heap)?;
        let assert_equal = ConsoleAssertEqual::init(heap)?;
        let assert_not_reached = ConsoleAssertNotReached::init(heap)?;
        let log = ConsoleLog::init(heap)?;

        let props = hash_map![
            prop_key!("assert") => Property::new(assert.as_value(), Writable::Yes),
            prop_key!("assertEqual") => Property::new(assert_equal.as_value(), Writable::Yes),
            prop_key!("assertNotReached") => Property::new(
                assert_not_reached.as_value(),
                Writable::Yes
            ),
            prop_key!("log") => Property::new(log.as_value(), Writable::Yes),
        ];

        let obj_ref = heap.allocate(Object::new(None, props, ObjectData::None, Extensible::Yes))?;
        Ok(Self { obj_ref })
    }

    fn obj_ref(&self) -> &Reference {
        &self.obj_ref
    }
}

builtin_fn!(ConsoleAssert, Extensible::Yes, (it, _receiver, args) => {
    let mut args = args.iter();
    let assertion = args.next().unwrap_or(&Value::Undefined);
    if it.is_truthy(assertion) {
        Ok(Value::Undefined)
    } else {
        let detail_msg = build_msg(it, args);
        Err(ErrorKind::from(AssertionError::new(detail_msg)))
    }
});

builtin_fn!(ConsoleAssertEqual, Extensible::Yes, (it, _receiver, args) => {
    fn is_nan(v: &Value) -> bool {
        matches!(v, Value::Number(n) if n.is_nan())
    }

    let mut args = args.iter();
    let actual = args.next().unwrap_or(&Value::Undefined);
    let expected = args.next().unwrap_or(&Value::Boolean(true));
    if is_nan(expected) && is_nan(actual) || it.is_truthy(&it.strictly_equal(actual, expected)?)
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

builtin_fn!(ConsoleAssertNotReached, Extensible::Yes, (it, _receiver, args) => {
    let detail_msg = format!("entered unreachable code: {}", build_msg(it, args.iter()));
    Err(ErrorKind::from(AssertionError::new(detail_msg)))
});

builtin_fn!(ConsoleLog, Extensible::Yes, (it, _receiver, args) => {
    let msg = build_msg(it, args.iter());
    it.vm_mut().write_message(&msg);
    Ok(Value::Undefined)
});

fn build_msg<'a>(it: &Interpreter, values: impl Iterator<Item = &'a Value>) -> String {
    values
        .map(|arg| it.coerce_to_string(arg))
        .intersperse_with(|| " ".to_owned())
        .collect()
}
