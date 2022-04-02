use super::{Builtin, NativeHeap, NativeRef};
use crate::interpreter::{ErrorKind, InitialisationError, Value, Vm};

pub struct String;

impl Builtin for String {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, vm: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        Ok(Value::String(match arg {
            Some(arg) => arg.coerce_to_string(vm),
            None => "".to_owned(),
        }))
    }
}
