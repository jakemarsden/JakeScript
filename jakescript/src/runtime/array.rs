use super::Builtin;
use crate::interpreter::{
    ErrorKind, Extensible, Heap, InitialisationError, Number, Object, ObjectData, Property,
    PropertyKey, Reference, Value,
};
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;

pub struct ArrayProtoBuiltin {
    obj_ref: Reference,
}

impl Builtin for ArrayProtoBuiltin {
    type InitArgs = (Reference, Reference);

    fn init(
        heap: &mut Heap,
        (obj_proto, fn_proto): Self::InitArgs,
    ) -> Result<Self, InitialisationError> {
        let length = GetLengthBuiltin::init(heap, fn_proto)?;
        let push = PushBuiltin::init(heap, fn_proto)?;

        let props = hash_map![
            prop_key!("length") => Property::new_const_accessor(length.obj_ref()),
            prop_key!("push") => Property::new_user(push.as_value()),
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

builtin_fn!(pub ArrayCtorBuiltin, Extensible::Yes, (it, _receiver, args) => {
    it.alloc_array(args.to_vec())
        .map(Value::Object)
        .map_err(ErrorKind::from)
});

builtin_fn!(GetLengthBuiltin, Extensible::No, (it, receiver, _args) => {
    let receiver = it.vm().heap().resolve(receiver);
    let length = receiver.as_ref().own_property_keys().count();
    let length = Number::try_from(length).unwrap_or_else(|_| {
        // TODO
        unreachable!()
    });
    Ok(Value::Number(length))
});

builtin_fn!(PushBuiltin, Extensible::Yes, (it, receiver, args) => {
    let mut array = it.vm_mut().heap_mut().resolve_mut(receiver);
    let start_len = array.as_ref().own_property_keys().count();
    args.iter()
        .copied()
        .enumerate()
        .map(|(idx, value)| {
            (
                PropertyKey::from(start_len + idx),
                Property::new_enumerable(value),
            )
        })
        .for_each(|(prop_key, prop_value)| {
            let defined = array.as_ref_mut().define_own_property(prop_key, prop_value);
            assert!(defined);
        });
    let finish_len = array.as_ref().own_property_keys().count();
    let finish_len = Number::try_from(finish_len).unwrap_or_else(|_| {
        // TODO
        unreachable!()
    });
    Ok(Value::Number(finish_len))
});
