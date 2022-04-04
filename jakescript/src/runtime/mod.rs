pub use global::DefaultGlobalObject;

use crate::interpreter::{ErrorKind, Heap, InitialisationError, Object, Reference, Value, Vm};

mod boolean;
mod console;
mod global;
mod math;
mod number;
mod string;

pub type NativeFn = dyn Fn(&mut Vm, &[Value]) -> Result<Value, ErrorKind>;

pub trait Builtin {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError>
    where
        Self: Sized;
}

pub struct Runtime {
    global_object: Reference,
}

impl Runtime {
    pub fn with_default_global_object(heap: &mut Heap) -> Result<Self, InitialisationError> {
        Self::with_custom_global_object::<DefaultGlobalObject>(heap)
    }

    pub fn with_custom_global_object<T: Builtin>(
        heap: &mut Heap,
    ) -> Result<Self, InitialisationError> {
        let global_object = T::register(heap)?;
        Ok(Self { global_object })
    }

    pub fn global_object_ref(&self) -> &Reference {
        &self.global_object
    }
}

fn register_builtin(heap: &mut Heap, obj: Object) -> Result<Reference, InitialisationError> {
    heap.allocate(obj).map_err(InitialisationError::from)
}
