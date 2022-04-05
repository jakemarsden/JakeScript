use super::{register_builtin, Builtin};
use crate::interpreter::{
    ErrorKind, Heap, InitialisationError, Interpreter, Number, Object, Property, Reference, Value,
};
use crate::non_empty_str;
use common_macros::hash_map;

pub struct String;
pub struct StringCharAt;

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
        let properties = hash_map![
            non_empty_str!("charAt")
                => Property::new(true, Value::Object(StringCharAt::register(heap)?)),
        ];
        let obj = Object::new_builtin(true, properties, Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

impl StringCharAt {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(it: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        // TODO: Implement `this` expressions to make receivers a real thing.
        let receiver = args.first().cloned().unwrap_or_default();
        let arg = args.get(1).cloned().unwrap_or_default();
        let idx = {
            let n = it.coerce_to_number(&arg);
            if !n.is_nan() {
                n
            } else {
                Number::Int(0)
            }
        };
        let char_str = if idx >= Number::Int(0) {
            let idx = usize::try_from(idx.as_i64()).unwrap();
            it.coerce_to_string(&receiver)
                .chars()
                .nth(idx)
                .map(|ch| ch.to_string())
                .unwrap_or_default()
        } else {
            std::string::String::default()
        };
        it.vm_mut()
            .heap_mut()
            .allocate(Object::new_string(char_str))
            .map(Value::Object)
            .map_err(ErrorKind::from)
    }
}

impl Builtin for StringCharAt {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}
