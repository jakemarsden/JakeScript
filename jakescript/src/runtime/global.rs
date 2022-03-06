use crate::ast::Identifier;
use crate::interpreter;
use crate::interpreter::{
    Error, ExecutionState, InitialisationError, Value, VariableNotDefinedError, Vm,
};
use crate::runtime::boolean::Boolean;
use crate::runtime::console::Console;
use crate::runtime::math::Math;
use crate::runtime::number::Number;
use crate::runtime::string::String;
use crate::runtime::{Builtin, NativeHeap, NativeRef};

pub struct DefaultGlobalObject {
    boolean: Value,
    console: Value,
    exit: Value,
    is_nan: Value,
    math: Value,
    number: Value,
    string: Value,
}

impl Builtin for DefaultGlobalObject {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        let global = Self {
            exit: Value::NativeObject(GlobalExit::register(run)?),
            boolean: Value::NativeObject(Boolean::register(run)?),
            console: Value::NativeObject(Console::register(run)?),
            is_nan: Value::NativeObject(GlobalIsNan::register(run)?),
            math: Value::NativeObject(Math::register(run)?),
            number: Value::NativeObject(Number::register(run)?),
            string: Value::NativeObject(String::register(run)?),
        };
        Ok(run.register_builtin(global)?)
    }

    fn to_js_string(&self) -> std::string::String {
        "[object Window]".to_owned()
    }

    fn property(&self, name: &Identifier) -> interpreter::Result<Option<Value>> {
        Ok(match name.as_str() {
            "Infinity" => Some(Value::Number(interpreter::Number::POS_INF)),
            "NaN" => Some(Value::Number(interpreter::Number::NAN)),
            "undefined" => Some(Value::Undefined),

            "Boolean" => Some(self.boolean.clone()),
            "Math" => Some(self.math.clone()),
            "Number" => Some(self.number.clone()),
            "String" => Some(self.string.clone()),
            "console" => Some(self.console.clone()),
            "exit" => Some(self.exit.clone()),
            "isNaN" => Some(self.is_nan.clone()),
            _ => return Err(Error::VariableNotDefined(VariableNotDefinedError)),
        })
    }

    fn set_property(&mut self, name: &Identifier, value: Value) -> interpreter::Result<Option<()>> {
        // TODO: Silently ignore setting: `Infinity`, `NaN`, `undefined`
        Ok(match name.as_str() {
            "Infinity" | "NaN" | "undefined" => {
                // Silently ignore
                Some(())
            }

            "Boolean" => {
                self.boolean = value;
                Some(())
            }
            "Math" => {
                self.math = value;
                Some(())
            }
            "Number" => {
                self.number = value;
                Some(())
            }
            "String" => {
                self.string = value;
                Some(())
            }
            "console" => {
                self.console = value;
                Some(())
            }
            "exit" => {
                self.exit = value;
                Some(())
            }
            "isNaN" => {
                self.is_nan = value;
                Some(())
            }
            _ => return Err(Error::VariableNotDefined(VariableNotDefinedError)),
        })
    }
}

pub struct GlobalExit;

impl Builtin for GlobalExit {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, vm: &mut Vm, _: &[Value]) -> interpreter::Result {
        vm.set_execution_state(ExecutionState::Exit);
        Ok(Value::Undefined)
    }
}

pub struct GlobalIsNan;

impl Builtin for GlobalIsNan {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, _: &mut Vm, args: &[Value]) -> interpreter::Result {
        let arg = args.first().unwrap_or(&Value::Undefined);
        Ok(Value::Boolean(match arg {
            Value::Boolean(_)
            | Value::String(_)
            | Value::Reference(_)
            | Value::NativeObject(_)
            | Value::Null
            | Value::Undefined => true,
            Value::Number(arg) => arg.is_nan(),
        }))
    }
}
