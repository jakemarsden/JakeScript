use super::boolean::Boolean;
use super::console::Console;
use super::math::Math;
use super::number::Number;
use super::string::String;
use super::{register_builtin, Builtin};
use crate::interpreter::{
    self, ErrorKind, ExecutionState, Heap, InitialisationError, Interpreter, Object, Property,
    Reference, Value,
};
use crate::non_empty_str;
use common_macros::hash_map;

pub struct DefaultGlobalObject;
pub struct GlobalExit;
pub struct GlobalIsNan;

impl Builtin for DefaultGlobalObject {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let properties = hash_map![
            non_empty_str!("Boolean")
                => Property::new(true, Value::Object(Boolean::register(heap)?)),
            non_empty_str!("Infinity")
                => Property::new(false, Value::Number(interpreter::Number::POS_INF)),
            non_empty_str!("Math")
                => Property::new(true, Value::Object(Math::register(heap)?)),
            non_empty_str!("NaN")
                => Property::new(false, Value::Number(interpreter::Number::NAN)),
            non_empty_str!("Number")
                => Property::new(true, Value::Object(Number::register(heap)?)),
            non_empty_str!("String")
                => Property::new(true, Value::Object(String::register(heap)?)),

            non_empty_str!("console")
                => Property::new(true, Value::Object(Console::register(heap)?)),
            non_empty_str!("exit")
                => Property::new(true, Value::Object(GlobalExit::register(heap)?)),
            non_empty_str!("isNaN")
                => Property::new(true, Value::Object(GlobalIsNan::register(heap)?)),
            non_empty_str!("undefined")
                => Property::new(false, Value::Undefined),
        ];
        let obj = Object::new_builtin(false, properties, None);
        register_builtin(heap, obj)
    }
}

impl GlobalExit {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(it: &mut Interpreter, _: &[Value]) -> Result<Value, ErrorKind> {
        it.vm_mut().set_execution_state(ExecutionState::Exit);
        Ok(Value::Undefined)
    }
}

impl Builtin for GlobalExit {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}

impl GlobalIsNan {
    #[allow(clippy::unnecessary_wraps)]
    fn invoke(_: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first().unwrap_or(&Value::Undefined);
        Ok(Value::Boolean(match arg {
            Value::Boolean(_) | Value::Object(_) | Value::Null | Value::Undefined => true,
            Value::Number(arg) => arg.is_nan(),
        }))
    }
}

impl Builtin for GlobalIsNan {
    fn register(heap: &mut Heap) -> Result<Reference, InitialisationError> {
        let obj = Object::new_builtin(true, hash_map![], Some(&Self::invoke));
        register_builtin(heap, obj)
    }
}
