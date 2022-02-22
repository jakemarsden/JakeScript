use crate::ast::Identifier;
use crate::interpreter::{
    self, Heap, InitialisationError, Number, ScopeCtx, Value, Variable, VariableKind, Vm,
};
use crate::non_empty_str;
use crate::runtime::{native_fn, Builtin};
use crate::str::NonEmptyString;
use common_macros::hash_map;

pub struct MathBuiltin;

impl Builtin for MathBuiltin {
    fn register(&self, global: &mut ScopeCtx, heap: &mut Heap) -> Result<(), InitialisationError> {
        let math_obj_props = hash_map! {
            Identifier::from(non_empty_str!("sqrt")) => native_fn("sqrt", &builtin_sqrt),
        };
        let math_obj = heap
            .allocate_object(math_obj_props)
            .map_err(InitialisationError::from)?;

        global.declare_variable(Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("Math")),
            Value::Reference(math_obj),
        ));

        Ok(())
    }
}

fn builtin_sqrt(_: &mut Vm, args: &[Value]) -> interpreter::Result {
    Ok(Value::Number(match args.first().cloned() {
        Some(value) => value.coerce_to_number().sqrt(),
        None => Number::NAN,
    }))
}
