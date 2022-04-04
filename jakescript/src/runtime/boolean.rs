use super::{register_builtin, Builtin};
use crate::interpreter::{ErrorKind, Heap, InitialisationError, Object, Reference, Value, Vm};
use common_macros::hash_map;

pub struct Boolean;

impl Boolean {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        Ok(Value::Boolean(match arg {
            Some(arg) => arg.coerce_to_bool(),
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
