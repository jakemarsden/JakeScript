use crate::ast::Identifier;
use crate::interpreter::error::InitialisationError;
use crate::interpreter::heap::Heap;
use crate::interpreter::stack::{Scope, ScopeCtx, Variable, VariableKind};
use crate::interpreter::value::{NativeFunction, Number, Value};
use crate::interpreter::vm::{ExecutionState, Vm};
use crate::non_empty_str;
use crate::str::NonEmptyString;
use common_macros::hash_map;

// TODO: What's the difference between a property of the global object, and a variable which is
//  accessible from the global scope?
pub(crate) fn create(heap: &mut Heap) -> Result<Scope, InitialisationError> {
    let console_obj = heap
        .allocate_object(hash_map! {
            Identifier::from(non_empty_str!("log"))
            => Value::NativeFunction(NativeFunction::new("log", &builtin_console_log)),
        })
        .map_err(InitialisationError::from)?;

    let math_obj = heap
        .allocate_object(hash_map! {
            Identifier::from(non_empty_str!("sqrt"))
            => Value::NativeFunction(NativeFunction::new("sqrt", &builtin_math_sqrt)),
        })
        .map_err(InitialisationError::from)?;

    let global_scope = ScopeCtx::new(vec![
        Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("Infinity")),
            Value::Number(Number::POS_INF),
        ),
        Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("NaN")),
            Value::Number(Number::NAN),
        ),
        Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("undefined")),
            Value::Undefined,
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("Boolean")),
            Value::NativeFunction(NativeFunction::new("Boolean", &builtin_boolean)),
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("Number")),
            Value::NativeFunction(NativeFunction::new("Number", &builtin_number)),
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("String")),
            Value::NativeFunction(NativeFunction::new("String", &builtin_string)),
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("exit")),
            Value::NativeFunction(NativeFunction::new("exit", &builtin_exit)),
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("isNaN")),
            Value::NativeFunction(NativeFunction::new("isNaN", &builtin_isnan)),
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("console")),
            Value::Reference(console_obj),
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("Math")),
            Value::Reference(math_obj),
        ),
    ]);
    Ok(Scope::new(global_scope))
}

fn builtin_boolean(_: &mut Vm, args: &[Value]) -> Value {
    match args.first().cloned() {
        Some(value) => Value::Boolean(value.coerce_to_bool()),
        None => Value::Boolean(false),
    }
}

fn builtin_number(_: &mut Vm, args: &[Value]) -> Value {
    match args.first().cloned() {
        Some(value) => Value::Number(value.coerce_to_number()),
        None => Value::Number(Number::Int(0)),
    }
}

fn builtin_string(vm: &mut Vm, args: &[Value]) -> Value {
    match args.first().cloned() {
        Some(value) => Value::String(value.coerce_to_string(vm)),
        None => Value::String("".to_owned()),
    }
}

fn builtin_isnan(_: &mut Vm, args: &[Value]) -> Value {
    let value = args.first().cloned().unwrap_or_default();
    Value::Boolean(match value {
        Value::Boolean(_)
        | Value::String(_)
        | Value::Reference(_)
        | Value::NativeFunction(_)
        | Value::Null
        | Value::Undefined => true,
        Value::Number(value) => value.is_nan(),
    })
}

fn builtin_console_log(vm: &mut Vm, args: &[Value]) -> Value {
    let msg: String = args
        .iter()
        .map(|arg| arg.coerce_to_string(vm))
        .intersperse_with(|| " ".to_owned())
        .collect();
    vm.write_message(&msg);
    Value::Undefined
}

fn builtin_exit(vm: &mut Vm, _args: &[Value]) -> Value {
    vm.set_execution_state(ExecutionState::Exit);
    Value::Undefined
}

fn builtin_math_sqrt(_: &mut Vm, args: &[Value]) -> Value {
    Value::Number(match args.first().cloned() {
        Some(value) => value.coerce_to_number().sqrt(),
        None => Number::NAN,
    })
}
