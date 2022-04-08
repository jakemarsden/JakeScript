use crate::builtin_fn;
use crate::interpreter::{self, Extensible, Value};

builtin_fn!(Number, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first();
    Ok(Value::Number(match arg {
        Some(arg) => it.coerce_to_number(arg),
        None => interpreter::Number::Int(0),
    }))
});
