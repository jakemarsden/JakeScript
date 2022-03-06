use crate::interpreter::{self, InitialisationError, Value, Vm};
use crate::runtime::{Builtin, NativeHeap, NativeRef};

pub struct String;

impl Builtin for String {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, vm: &mut Vm, args: &[Value]) -> interpreter::Result {
        let arg = args.first();
        Ok(Value::String(match arg {
            Some(arg) => arg.coerce_to_string(vm),
            None => "".to_owned(),
        }))
    }
}
