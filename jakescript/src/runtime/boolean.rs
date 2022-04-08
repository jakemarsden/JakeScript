use crate::builtin_fn;
use crate::interpreter::{Extensible, Value};

builtin_fn!(Boolean, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first();
    Ok(Value::Boolean(match arg {
        Some(arg) => it.coerce_to_bool(arg),
        None => false,
    }))
});
