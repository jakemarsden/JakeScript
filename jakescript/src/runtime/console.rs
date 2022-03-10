use super::{Builtin, NativeHeap, NativeRef};
use crate::ast::Identifier;
use crate::interpreter::{self, AssertionError, Error, InitialisationError, Value, Vm};

pub struct Console {
    assert: Value,
    assert_not_reached: Value,
    log: Value,
}

impl Builtin for Console {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        let console = Self {
            assert: Value::NativeObject(ConsoleAssert::register(run)?),
            assert_not_reached: Value::NativeObject(ConsoleAssertNotReached::register(run)?),
            log: Value::NativeObject(ConsoleLog::register(run)?),
        };
        Ok(run.register_builtin(console)?)
    }

    fn to_js_string(&self) -> String {
        "[object Object]".to_owned()
    }

    fn property(&self, name: &Identifier) -> interpreter::Result<Option<Value>> {
        Ok(match name.as_str() {
            "assert" => Some(self.assert.clone()),
            "assertNotReached" => Some(self.assert_not_reached.clone()),
            "log" => Some(self.log.clone()),
            _ => None,
        })
    }

    fn set_property(&mut self, name: &Identifier, value: Value) -> interpreter::Result<Option<()>> {
        Ok(match name.as_str() {
            "assert" => {
                self.assert = value;
                Some(())
            }
            "assertNotReached" => {
                self.assert_not_reached = value;
                Some(())
            }
            "log" => {
                self.log = value;
                Some(())
            }
            _ => None,
        })
    }
}

pub struct ConsoleAssert;

impl Builtin for ConsoleAssert {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, vm: &mut Vm, args: &[Value]) -> interpreter::Result {
        let mut args = args.iter();
        let assertion = args.next().unwrap_or(&Value::Undefined);
        if assertion.is_truthy() {
            Ok(Value::Undefined)
        } else {
            let detail_msg = build_msg(vm, args);
            Err(Error::Assertion(AssertionError::new(detail_msg)))
        }
    }
}

pub struct ConsoleAssertNotReached;

impl Builtin for ConsoleAssertNotReached {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, vm: &mut Vm, args: &[Value]) -> interpreter::Result {
        let detail_msg = build_msg(vm, args.iter());
        Err(Error::Assertion(AssertionError::new(detail_msg)))
    }
}

pub struct ConsoleLog;

impl Builtin for ConsoleLog {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, vm: &mut Vm, args: &[Value]) -> interpreter::Result {
        let msg = build_msg(vm, args.iter());
        vm.write_message(&msg);
        Ok(Value::Undefined)
    }
}

fn build_msg<'a>(vm: &Vm, values: impl Iterator<Item = &'a Value>) -> String {
    values
        .map(|arg| arg.coerce_to_string(vm))
        .intersperse_with(|| " ".to_owned())
        .collect()
}
