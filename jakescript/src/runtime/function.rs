use super::Builtin;
use crate::interpreter::{Extensible, Heap, InitialisationError, Object, ObjectData, Reference};
use common_macros::hash_map;

pub struct FunctionProtoBuiltin {
    obj_ref: Reference,
}

impl Builtin for FunctionProtoBuiltin {
    type InitArgs = Reference;

    fn init(heap: &mut Heap, obj_proto: Self::InitArgs) -> Result<Self, InitialisationError> {
        let obj_ref = heap.allocate(Object::new(
            Some(obj_proto),
            hash_map![],
            ObjectData::None,
            Extensible::Yes,
        ))?;
        Ok(Self { obj_ref })
    }

    fn obj_ref(&self) -> &Reference {
        &self.obj_ref
    }
}
