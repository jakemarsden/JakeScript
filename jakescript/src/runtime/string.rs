use crate::ast::Identifier;
use crate::interpreter::{
    self, Heap, InitialisationError, ScopeCtx, Value, Variable, VariableKind, Vm,
};
use crate::non_empty_str;
use crate::runtime::{native_fn, Builtin};
use crate::str::NonEmptyString;

pub struct StringBuiltin;

impl Builtin for StringBuiltin {
    fn register(&self, global: &mut ScopeCtx, _: &mut Heap) -> Result<(), InitialisationError> {
        global.declare_variable(Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("String")),
            native_fn("String", &builtin_string),
        ));
        Ok(())
    }
}

fn builtin_string(vm: &mut Vm, args: &[Value]) -> interpreter::Result {
    Ok(Value::String(match args.first().cloned() {
        Some(arg) => arg.coerce_to_string(vm),
        None => "".to_owned(),
    }))
}
