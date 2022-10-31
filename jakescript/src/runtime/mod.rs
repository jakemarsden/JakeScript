use crate::interpreter::{ErrorKind, Heap, InitialisationError, Interpreter, Reference, Value};
pub use global::GlobalObject;
use std::fmt;

mod array;
mod boolean;
mod console;
mod function;
mod global;
mod math;
mod number;
mod object;
mod string;

#[macro_export]
macro_rules! builtin_fn {
    (
        $vis:vis
        $name:ident,
        $extensible:expr,
        ($it:ident, $receiver:ident, $args:ident$(,)?) => $fn_body:expr$(,)?
    ) => {
        $vis struct $name {
            obj_ref: $crate::interpreter::Reference,
        }

        impl $name {
            // unnecessary_wraps: Required to conform to `NativeCall`.
            #[allow(clippy::unnecessary_wraps)]
            fn call(
                $it: &mut $crate::interpreter::Interpreter,
                $receiver: $crate::interpreter::Reference,
                $args: &[$crate::interpreter::Value],
            ) -> ::std::result::Result<$crate::interpreter::Value, $crate::interpreter::ErrorKind>
            {
                $fn_body
            }
        }

        impl $crate::runtime::Builtin for $name {
            type InitArgs = $crate::interpreter::Reference;

            fn init(
                heap: &mut $crate::interpreter::Heap,
                fn_proto: Self::InitArgs,
            ) -> ::std::result::Result<Self, $crate::interpreter::InitialisationError> {
                let props = ::std::collections::HashMap::default();
                let obj_ref = heap.allocate($crate::interpreter::Object::new_native(
                    Some(fn_proto),
                    props,
                    &Self::call,
                    $extensible,
                ))?;
                Ok(Self { obj_ref })
            }

            fn obj_ref(&self) -> &$crate::interpreter::Reference {
                &self.obj_ref
            }
        }
    };
}

pub struct Runtime<T: Builtin = GlobalObject> {
    global_object: T,
}

impl Runtime {
    pub fn with_default_global_object(heap: &mut Heap) -> Result<Self, InitialisationError> {
        Runtime::<GlobalObject>::with_custom_global_object(heap, ())
    }
}

impl<T: Builtin> Runtime<T> {
    pub fn with_custom_global_object(
        heap: &mut Heap,
        args: T::InitArgs,
    ) -> Result<Self, InitialisationError> {
        let global_object = T::init(heap, args)?;
        Ok(Self { global_object })
    }

    pub fn global_object(&self) -> &T {
        &self.global_object
    }

    pub fn global_object_ref(&self) -> &Reference {
        self.global_object().obj_ref()
    }
}

pub trait Builtin {
    type InitArgs = ();

    fn init(heap: &mut Heap, _: Self::InitArgs) -> Result<Self, InitialisationError>
    where
        Self: Sized;

    fn obj_ref(&self) -> &Reference;

    fn as_obj_ref(&self) -> Reference {
        self.obj_ref().clone()
    }

    fn as_value(&self) -> Value {
        Value::Object(self.as_obj_ref())
    }
}

type NativeFn<'a> = &'a dyn Fn(&mut Interpreter, Reference, &[Value]) -> Result<Value, ErrorKind>;

#[derive(Clone)]
pub struct NativeCall(NativeFn<'static>);

impl NativeCall {
    pub fn call(
        &self,
        it: &mut Interpreter,
        receiver: Reference,
        args: &[Value],
    ) -> Result<Value, ErrorKind> {
        (self.0)(it, receiver, args)
    }
}

impl<F> From<&'static F> for NativeCall
where
    F: Fn(&mut Interpreter, Reference, &[Value]) -> Result<Value, ErrorKind>,
{
    fn from(f: &'static F) -> Self {
        Self(f)
    }
}

impl fmt::Debug for NativeCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NativeCall({:p})", self.0)
    }
}
