use crate::ast::Identifier;
use crate::interpreter::error::InitialisationError;
use crate::interpreter::heap::Heap;
use crate::interpreter::stack::{Scope, ScopeCtx, Variable, VariableKind};
use crate::interpreter::value::{NativeFunction, Number, Sign, Value};
use crate::interpreter::vm::Vm;
use crate::non_empty_str;
use crate::str::NonEmptyString;
use common_macros::hash_map;

// TODO: What's the difference between a property of the global object, and a variable which is
//  accessible from the global scope?
pub(crate) fn create(heap: &mut Heap) -> Result<Scope, InitialisationError> {
    let global_scope = ScopeCtx::new(vec![
        Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("Infinity")),
            Value::Number(Number::Inf(Sign::Pos)),
        ),
        Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("NaN")),
            Value::Number(Number::NaN),
        ),
        Variable::new(
            VariableKind::SilentReadOnly,
            Identifier::from(non_empty_str!("undefined")),
            Value::Undefined,
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("isNaN")),
            Value::NativeFunction(NativeFunction::new("isNaN", &builtin_isnan)),
        ),
        Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("Math")),
            Value::Reference(
                heap.allocate_object(hash_map! {
                    Identifier::from(non_empty_str!("sqrt"))
                    => Value::NativeFunction(NativeFunction::new("sqrt", &builtin_math_sqrt)),
                })
                .map_err(InitialisationError::from)?,
            ),
        ),
    ]);
    Ok(Scope::new(global_scope))
}

fn builtin_isnan(_: &mut Vm, args: &[Value]) -> Value {
    let value = args.first().cloned().unwrap_or_default();
    Value::Boolean(match value {
        Value::Number(Number::Int(_) | Number::Inf(_)) => false,
        Value::Boolean(_)
        | Value::Number(Number::NaN)
        | Value::String(_)
        | Value::Reference(_)
        | Value::NativeFunction(_)
        | Value::Null
        | Value::Undefined => true,
    })
}

fn builtin_math_sqrt(_: &mut Vm, args: &[Value]) -> Value {
    let value = args.first().cloned().unwrap_or_default().coerce_to_number();
    value.checked_sqrt().map_or(Value::Undefined, Value::Number)
}
