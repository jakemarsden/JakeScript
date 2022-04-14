use super::Builtin;
use crate::interpreter::{
    ErrorKind, Extensible, Heap, InitialisationError, Interpreter, Number, Object, Property,
    Reference, Value,
};
use crate::prop_key;
use common_macros::hash_map;

pub struct ArrayBuiltin {
    obj_ref: Reference,
}

impl ArrayBuiltin {
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
}

impl Builtin for ArrayBuiltin {
    fn init(heap: &mut Heap) -> Result<Self, InitialisationError> {
        let props = hash_map![
            prop_key!("length") => Property::new_const_accessor(&Self::length),
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
