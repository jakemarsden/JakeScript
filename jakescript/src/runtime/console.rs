use super::{register_builtin, Builtin};
use crate::interpreter::{
    AssertionError, ErrorKind, Heap, InitialisationError, Object, Property, Reference, Value, Vm,
};
use crate::non_empty_str;
use common_macros::hash_map;

pub struct Console;
pub struct ConsoleAssert;
pub struct ConsoleAssertNotReached;
pub struct ConsoleLog;

impl Builtin for Console {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let properties = hash_map![
            non_empty_str!("assert")
                => Property::new(true, Value::Object(ConsoleAssert::register(heap)?)),
            non_empty_str!("assertNotReached")
                => Property::new(true, Value::Object(ConsoleAssertNotReached::register(heap)?)),
            non_empty_str!("log")
                => Property::new(true, Value::Object(ConsoleLog::register(heap)?)),
        ];
        let obj = Object::new_builtin(true, properties, None);
        register_builtin(heap, obj)
    }
}

impl ConsoleAssert {
    fn invoke(vm: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let mut args = args.iter();
        let assertion = args.next().unwrap_or(&Value::Undefined);
        if assertion.is_truthy() {
            Ok(Value::Undefined)
        } else {
            let detail_msg = build_msg(vm, args);
            Err(ErrorKind::from(AssertionError::new(detail_msg)))
        }
    }
}

impl Builtin for ConsoleAssert {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

impl ConsoleAssertNotReached {
    fn invoke(vm: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let detail_msg = build_msg(vm, args.iter());
        Err(ErrorKind::from(AssertionError::new(detail_msg)))
    }
}

impl Builtin for ConsoleAssertNotReached {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

impl ConsoleLog {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(vm: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let msg = build_msg(vm, args.iter());
        vm.write_message(&msg);
        Ok(Value::Undefined)
    }
}

impl Builtin for ConsoleLog {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

fn build_msg<'a>(vm: &Vm, values: impl Iterator<Item = &'a Value>) -> String {
    values
        .map(|arg| arg.coerce_to_string(vm))
        .intersperse_with(|| " ".to_owned())
        .collect()
}
