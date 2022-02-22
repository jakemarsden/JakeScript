use crate::ast::Identifier;
use crate::interpreter::{
    self, Heap, InitialisationError, ScopeCtx, Value, Variable, VariableKind, Vm,
};
use crate::non_empty_str;
use crate::runtime::{native_fn, Builtin};
use crate::str::NonEmptyString;
use common_macros::hash_map;

pub struct ConsoleBuiltin;

impl Builtin for ConsoleBuiltin {
    fn register(&self, global: &mut ScopeCtx, heap: &mut Heap) -> Result<(), InitialisationError> {
        let console_obj_props = hash_map! {
            Identifier::from(non_empty_str!("log")) => native_fn("log", &builtin_log),
        };
        let console_obj = heap
            .allocate_object(console_obj_props)
            .map_err(InitialisationError::from)?;

        global.declare_variable(Variable::new(
            VariableKind::Var,
            Identifier::from(non_empty_str!("console")),
            Value::Reference(console_obj),
        ));

        Ok(())
    }
}

fn builtin_log(vm: &mut Vm, args: &[Value]) -> interpreter::Result {
    let msg: String = args
        .iter()
        .map(|arg| arg.coerce_to_string(vm))
        .intersperse_with(|| " ".to_owned())
        .collect();
    vm.write_message(&msg);
    Ok(Value::Undefined)
}
