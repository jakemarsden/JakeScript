use crate::ast::Identifier;
use crate::interpreter::{Heap, InitialisationError, ScopeCtx, Value, Variable, VariableKind, Vm};
use crate::non_empty_str;
use crate::runtime::{native_fn, Builtin};
use crate::str::NonEmptyString;

pub struct BooleanBuiltin;

impl Builtin for BooleanBuiltin {
    fn register(&self, global: &mut ScopeCtx, _: &mut Heap) -> Result<(), InitialisationError> {
        global.declare_variable(Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("Boolean")),
            native_fn("Boolean", &builtin_boolean),
        ));
        Ok(())
    }
}

fn builtin_boolean(_: &mut Vm, args: &[Value]) -> Value {
    Value::Boolean(match args.first().cloned() {
        Some(arg) => arg.coerce_to_bool(),
        None => false,
    })
}
