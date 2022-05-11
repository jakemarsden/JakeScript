use super::Builtin;
use crate::interpreter::{Extensible, Heap, InitialisationError, Object, ObjectData, Reference};
use common_macros::hash_map;

pub struct ObjectProtoBuiltin {
    obj_ref: Reference,
}

impl Builtin for ObjectProtoBuiltin {
    fn init(heap: &mut Heap, (): Self::InitArgs) -> Result<Self, InitialisationError> {
        let obj_ref = heap.allocate(Object::new(
            None,
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
