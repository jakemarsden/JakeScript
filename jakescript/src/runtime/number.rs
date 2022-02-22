use crate::ast::Identifier;
use crate::interpreter::{
    Heap, InitialisationError, Number, ScopeCtx, Value, Variable, VariableKind, Vm,
};
use crate::non_empty_str;
use crate::runtime::{native_fn, Builtin};
use crate::str::NonEmptyString;

pub struct NumberBuiltin;

impl Builtin for NumberBuiltin {
    fn register(&self, global: &mut ScopeCtx, _: &mut Heap) -> Result<(), InitialisationError> {
        global.declare_variable(Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("Number")),
            native_fn("Number", &builtin_number),
        ));
        Ok(())
    }
}

fn builtin_number(_: &mut Vm, args: &[Value]) -> Value {
    Value::Number(match args.first().cloned() {
        Some(arg) => arg.coerce_to_number(),
        None => Number::Int(0),
    })
}
