use super::{Builtin, NativeHeap, NativeRef};
use crate::interpreter::{self, InitialisationError, Value, Vm};

pub struct Boolean;

impl Builtin for Boolean {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, _: &mut Vm, args: &[Value]) -> interpreter::Result {
        let arg = args.first();
        Ok(Value::Boolean(match arg {
            Some(arg) => arg.coerce_to_bool(),
            None => false,
        }))
    }
}
