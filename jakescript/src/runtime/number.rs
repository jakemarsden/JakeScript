use crate::builtin_fn;
use crate::interpreter::{Extensible, Number, Value};

builtin_fn!(pub NumberCtorBuiltin, Extensible::Yes, (it, _receiver, args) => {
    let arg = args.first();
    Ok(Value::Number(match arg {
        Some(arg) => it.coerce_to_number(*arg),
        None => Number::Int(0),
    }))
});
