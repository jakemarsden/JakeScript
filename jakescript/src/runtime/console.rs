use crate::ast::Identifier;
use crate::interpreter::{
    self, AssertionError, Heap, InitialisationError, ScopeCtx, Value, Variable, VariableKind, Vm,
};
use crate::non_empty_str;
use crate::runtime::{native_fn, Builtin};
use crate::str::NonEmptyString;
use common_macros::hash_map;

pub struct ConsoleBuiltin;

impl Builtin for ConsoleBuiltin {
    fn register(&self, global: &mut ScopeCtx, heap: &mut Heap) -> Result<(), InitialisationError> {
        let console_obj_props = hash_map! {
            Identifier::from(non_empty_str!("assert")) => native_fn("assert", &builtin_assert),
            Identifier::from(non_empty_str!("assertNotReached"))
                    => native_fn("assertNotReached", &builtin_assertnotreached),
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

fn builtin_assert(vm: &mut Vm, args: &[Value]) -> interpreter::Result {
    let mut args = args.iter();
    let assertion = args.next().unwrap_or(&Value::Undefined);
    if assertion.is_truthy() {
        Ok(Value::Undefined)
    } else {
        let detail_msg = build_msg(vm, args);
        Err(interpreter::Error::Assertion(AssertionError::new(
            detail_msg,
        )))
    }
}

fn builtin_assertnotreached(vm: &mut Vm, args: &[Value]) -> interpreter::Result {
    let detail_msg = build_msg(vm, args.iter());
    Err(interpreter::Error::Assertion(AssertionError::new(
        detail_msg,
    )))
}

fn builtin_log(vm: &mut Vm, args: &[Value]) -> interpreter::Result {
    let log_msg = build_msg(vm, args.iter());
    vm.write_message(&log_msg);
    Ok(Value::Undefined)
}

fn build_msg<'a>(vm: &Vm, values: impl Iterator<Item = &'a Value>) -> String {
    values
        .map(|arg| arg.coerce_to_string(vm))
        .intersperse_with(|| " ".to_owned())
        .collect()
}
