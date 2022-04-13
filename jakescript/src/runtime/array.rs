use super::Builtin;
use crate::interpreter::{
    Enumerable, ErrorKind, Extensible, Heap, InitialisationError, Interpreter, Number, Object,
    Property, Reference, Value, Writable,
};
use crate::prop_key;
use common_macros::hash_map;

pub struct Array {
    obj_ref: Reference,
}

impl Array {
    fn call(it: &mut Interpreter, _: Reference, args: &[Value]) -> Result<Value, ErrorKind> {
        it.alloc_array(args.to_vec())
            .map(Value::Object)
            .map_err(ErrorKind::from)
    }

    // needless_pass_by_value, unnecessary_wraps: Required to conform to `NativeGet`.
    #[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
    fn length(it: &Interpreter, receiver: Reference) -> Result<Value, ErrorKind> {
        let receiver = it.vm().heap().resolve(&receiver);
        let length = receiver.own_property_count();
        let length = Number::try_from(length).unwrap_or_else(|_| {
            // TODO
            unreachable!()
        });
        Ok(Value::Number(length))
    }

    // unnecessary_wraps: Required to conform to `NativeGet`.
    #[allow(clippy::unnecessary_wraps)]
    fn set_length(_: &mut Interpreter, _: Reference, _: Value) -> Result<bool, ErrorKind> {
        Ok(false)
    }
}

impl Builtin for Array {
    fn init(heap: &mut Heap) -> Result<Self, InitialisationError> {
        let props = hash_map![
            prop_key!("length") => Property::new_native(
                &Self::length,
                &Self::set_length,
                Writable::No,
                Enumerable::No,
            ),
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
