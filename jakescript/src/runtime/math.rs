use super::{register_builtin, Builtin};
use crate::interpreter::{
    ErrorKind, Heap, InitialisationError, Number, Object, Property, Reference, Value, Vm,
};
use crate::non_empty_str;
use common_macros::hash_map;

pub struct Math;
pub struct MathSqrt;

impl Builtin for Math {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let properties = hash_map![
            non_empty_str!("sqrt") => Property::new(true, Value::Object(MathSqrt::register(heap)?)),
        ];
        let obj = Object::new_builtin(true, properties, None);
        register_builtin(heap, obj)
    }
}

impl MathSqrt {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        Ok(Value::Number(match arg {
            Some(arg) => arg.coerce_to_number().sqrt(),
            None => Number::NAN,
        }))
    }
}

impl Builtin for MathSqrt {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}
