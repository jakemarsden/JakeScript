use crate::ast::Identifier;
use crate::interpreter::error::InitialisationError;
use crate::interpreter::stack::{Scope, ScopeCtx, Variable, VariableKind};
use crate::interpreter::value::{NativeFunction, Number, Sign, Value};
use crate::interpreter::vm::Vm;
use crate::non_empty_str;
use crate::str::NonEmptyString;

// TODO: What's the difference between a property of the global object, and a variable which is
//  accessible from the global scope?
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn create() -> Result<Scope, InitialisationError> {
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
            Value::NativeFunction(NativeFunction::new("isNaN", &is_nan_builtin)),
        ),
    ]);
    Ok(Scope::new(global_scope))
}

fn is_nan_builtin(_: &mut Vm, args: &[Value]) -> Value {
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
