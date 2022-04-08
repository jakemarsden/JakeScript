use super::{register_builtin, Builtin};
use crate::interpreter::{
    ErrorKind, Heap, InitialisationError, Interpreter, Object, Reference, Value,
};
use common_macros::hash_map;

pub struct Boolean;

impl Boolean {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(it: &mut Interpreter, _: &Value, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        Ok(Value::Boolean(match arg {
            Some(arg) => it.coerce_to_bool(arg),
            None => false,
        }))
    }
}

impl Builtin for Boolean {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}
