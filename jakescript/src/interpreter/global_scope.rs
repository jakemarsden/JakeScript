use crate::ast::Identifier;
use crate::interpreter::stack::{Scope, ScopeCtx, Variable, VariableKind};
use crate::interpreter::value::{Number, Sign, Value};
use crate::non_empty_str;
use crate::str::NonEmptyString;

// TODO: What's the difference between a property of the global object, and a variable which is
//  accessible from the global scope?
pub(crate) fn create() -> Scope {
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
    ]);
    Scope::new(global_scope)
}
