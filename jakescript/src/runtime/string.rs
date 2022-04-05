use super::{register_builtin, Builtin};
use crate::interpreter::{
    ErrorKind, Heap, InitialisationError, Interpreter, Object, Reference, Value,
};
use common_macros::hash_map;

pub struct String;

impl String {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(it: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        let str = arg.map_or_else(|| "".to_owned(), |arg| it.coerce_to_string(arg));
        it.vm_mut()
            .heap_mut()
            .allocate(Object::new_string(str))
            .map(Value::Object)
            .map_err(ErrorKind::from)
    }
}

impl Builtin for String {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}
