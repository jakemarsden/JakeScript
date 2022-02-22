#![allow(clippy::unnecessary_wraps)]

use crate::ast::Identifier;
use crate::interpreter::{
    self, ExecutionState, Heap, InitialisationError, NativeFunction, Number, ScopeCtx, Value,
    Variable, VariableKind, Vm,
};
use crate::non_empty_str;
use crate::str::NonEmptyString;

pub mod boolean;
pub mod console;
pub mod math;
pub mod number;
pub mod string;

pub trait Builtin {
    fn register(&self, global: &mut ScopeCtx, heap: &mut Heap) -> Result<(), InitialisationError>;
}

pub trait Runtime {
    fn create_global_context(&self, heap: &mut Heap) -> Result<ScopeCtx, InitialisationError>;
}

#[derive(Default)]
pub struct DefaultRuntime {}

impl Runtime for DefaultRuntime {
    fn create_global_context(&self, heap: &mut Heap) -> Result<ScopeCtx, InitialisationError> {
        let mut global = ScopeCtx::default();

        global.declare_variable(Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("Infinity")),
            Value::Number(Number::POS_INF),
        ));
        global.declare_variable(Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("NaN")),
            Value::Number(Number::NAN),
        ));
        global.declare_variable(Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("undefined")),
            Value::Undefined,
        ));

        global.declare_variable(Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("exit")),
            native_fn("exit", &builtin_exit),
        ));
        global.declare_variable(Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("isNaN")),
            native_fn("isNaN", &builtin_isnan),
        ));

        boolean::BooleanBuiltin.register(&mut global, heap)?;
        console::ConsoleBuiltin.register(&mut global, heap)?;
        math::MathBuiltin.register(&mut global, heap)?;
        number::NumberBuiltin.register(&mut global, heap)?;
        string::StringBuiltin.register(&mut global, heap)?;

        Ok(global)
    }
}

pub(crate) fn native_fn(
    name: &'static str,
    implementation: &'static dyn Fn(&mut Vm, &[Value]) -> interpreter::Result,
) -> Value {
    Value::NativeFunction(NativeFunction::new(name, implementation))
}

fn builtin_exit(vm: &mut Vm, _args: &[Value]) -> interpreter::Result {
    vm.set_execution_state(ExecutionState::Exit);
    Ok(Value::Undefined)
}

fn builtin_isnan(_: &mut Vm, args: &[Value]) -> interpreter::Result {
    Ok(Value::Boolean(
        match args.first().cloned().unwrap_or_default() {
            Value::Boolean(_)
            | Value::String(_)
            | Value::Reference(_)
            | Value::NativeFunction(_)
            | Value::Null
            | Value::Undefined => true,
            Value::Number(arg) => arg.is_nan(),
        },
    ))
}
