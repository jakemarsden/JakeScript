use super::{register_builtin, Builtin};
use crate::interpreter::{
    self, ErrorKind, Heap, InitialisationError, Object, Reference, Value, Vm,
};
use common_macros::hash_map;

pub struct Number;

impl Number {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        Ok(Value::Number(match arg {
            Some(arg) => arg.coerce_to_number(),
            None => interpreter::Number::Int(0),
        }))
    }
}

impl Builtin for Number {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}
