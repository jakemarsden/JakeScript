use super::{Builtin, NativeHeap, NativeRef};
use crate::interpreter::{self, InitialisationError, Value, Vm};

pub struct Number;

impl Builtin for Number {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, _: &mut Vm, args: &[Value]) -> interpreter::Result {
        let arg = args.first();
        Ok(Value::Number(match arg {
            Some(arg) => arg.coerce_to_number(),
            None => interpreter::Number::Int(0),
        }))
    }
}
