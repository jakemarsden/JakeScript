use super::{register_builtin, Builtin};
use crate::interpreter::{ErrorKind, Heap, InitialisationError, Object, Reference, Value, Vm};
use common_macros::hash_map;

pub struct String;

impl String {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(vm: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        Ok(Value::String(match arg {
            Some(arg) => arg.coerce_to_string(vm),
            None => "".to_owned(),
        }))
    }
}

impl Builtin for String {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}
