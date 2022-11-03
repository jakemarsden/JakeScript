use crate::ast::*;
use crate::runtime::NativeCall;
pub use error::*;
pub use heap::*;
pub use object::*;
pub use stack::*;
use std::borrow::Cow;
use std::cmp;
use std::str::FromStr;
pub use value::*;
pub use vm::*;

mod block;
mod declaration;
mod error;
mod expression;
mod heap;
mod literal;
mod object;
mod stack;
mod statement;
mod value;
mod vm;

pub trait Eval {
    type Output = ();

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output>;
}

pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn new(vm: Vm) -> Self {
        Self { vm }
    }

    pub fn vm(&self) -> &Vm {
        &self.vm
    }

    pub fn vm_mut(&mut self) -> &mut Vm {
        &mut self.vm
    }

    pub fn update_variable_or_global_object_property(
        &mut self,
        key: &Identifier,
        f: impl FnOnce(&mut Self, Value) -> Result<(Value, Value)>,
        e: impl FnOnce(ErrorKind) -> Error,
    ) -> Result {
        // TODO: Performance: Avoid repeated variable lookup.
        match self.vm().stack().lookup_variable(key) {
            Ok(variable) => {
                let curr_value = variable.value();
                let (result_value, updated_value) = f(self, curr_value)?;
                self.vm_mut()
                    .stack_mut()
                    .with_variable_mut(key, |variable| variable.set_value(updated_value))
                    .expect("variable somehow disappeared while computing the new value")
                    .map_err(ErrorKind::from)
                    .map_err(e)?;
                Ok(result_value)
            }
            Err(VariableNotDefinedError { .. }) => {
                let global_obj_ref = self.vm().runtime().global_object_ref();
                self.update_object_property(global_obj_ref, key, f, e)
            }
        }
    }

    pub fn update_object_property(
        &mut self,
        base_ref: Reference,
        key: &PropertyKey,
        f: impl FnOnce(&mut Self, Value) -> Result<(Value, Value)>,
        e: impl FnOnce(ErrorKind) -> Error,
    ) -> Result {
        let value = {
            let base_obj = self.vm().heap().resolve(base_ref);
            let value = match base_obj.as_ref().get(self, key, base_ref) {
                Ok(value) => value.unwrap_or_default(),
                Err(err) => return Err(e(err)),
            };
            value
        };
        let (result_value, updated_value) = f(self, value)?;
        self.vm_mut()
            .heap_mut()
            .resolve_mut(base_ref)
            .as_ref_mut()
            .set(self, key, base_ref, updated_value)
            .map_err(e)?;
        Ok(result_value)
    }

    /// # Panics
    ///
    /// Panics if the a [`OutOfStackSpaceError`] occurs while trying to call the
    /// function. TODO: Propagate the error to the caller instead.
    pub fn call_user_fn(
        &mut self,
        f: &UserFunction,
        fn_obj_ref: Reference,
        receiver: Option<Reference>,
        args: &[Value],
    ) -> Result {
        let declared_params = f.declared_parameters();
        let mut supplied_args = args.iter().copied();
        let mut variables = Vec::with_capacity(declared_params.len());
        for declared_param_name in declared_params.iter() {
            let arg_value = supplied_args.next().unwrap_or_default();
            variables.push(Variable::new(
                VariableKind::Let,
                declared_param_name.clone(),
                arg_value,
            ));
        }

        let declared_scope = f.declared_scope();

        self.vm_mut()
            .stack_mut()
            .push_frame_with_existing_scope(declared_scope, receiver)
            .map_err(|_| todo!())?;
        if let Some(fn_name) = f.name() {
            // Create an outer scope with nothing but the function's name, which points to
            // itself, so that named function literals may recurse using their name without
            // making the name visible outside of the function body. It has its own outer
            // scope so it can still be shadowed by parameters with the same name.
            let outer_variables = vec![Variable::new(
                VariableKind::Var,
                fn_name.clone(),
                Value::Object(fn_obj_ref),
            )];
            self.vm_mut()
                .stack_mut()
                .push_scope(false, outer_variables)
                .map_err(|_| todo!())?;
        }
        self.vm_mut()
            .stack_mut()
            .push_scope(true, variables)
            .map_err(|_| todo!())?;
        f.body().eval(self)?;
        self.vm_mut().stack_mut().pop_scope();
        if f.name().is_some() {
            self.vm_mut().stack_mut().pop_scope();
        }
        self.vm_mut().stack_mut().pop_frame();

        Ok(match self.vm().execution_state() {
            ExecutionState::Advance | ExecutionState::Exception(_) | ExecutionState::Exit => {
                Value::Undefined
            }
            ExecutionState::Return(_) => {
                if let ExecutionState::Return(value) = self.vm_mut().reset_execution_state() {
                    value
                } else {
                    unreachable!()
                }
            }
            state @ (ExecutionState::Break | ExecutionState::Continue) => {
                unreachable!("unexpected execution state: {state:?}")
            }
        })
    }

    pub fn call_native_fn(
        &mut self,
        f: &NativeCall,
        receiver: Option<Reference>,
        args: &[Value],
    ) -> std::result::Result<Value, ErrorKind> {
        let receiver = receiver.unwrap_or_else(|| self.vm().runtime().global_object_ref());
        f.call(self, receiver, args)
    }

    fn concat(
        &mut self,
        lhs: &str,
        rhs: &str,
    ) -> std::result::Result<Reference, OutOfHeapSpaceError> {
        let out = format!("{lhs}{rhs}");
        self.vm_mut().alloc_string(out.into_boxed_str())
    }

    pub fn equal(&self, lhs: Value, rhs: Value) -> bool {
        match lhs {
            Value::Boolean(lhs) => lhs == self.coerce_to_bool(rhs),
            Value::Number(lhs) => lhs == self.coerce_to_number(rhs),
            Value::Object(lhs) => {
                if let Value::Object(rhs) = rhs {
                    lhs == rhs || {
                        let lhs_obj = self.vm().heap().resolve(lhs);
                        let rhs_obj = self.vm().heap().resolve(rhs);
                        let value = if let Some(lhs_str) = lhs_obj.as_ref().string_data()
                            && let Some(rhs_str) = rhs_obj.as_ref().string_data()
                        {
                            lhs_str == rhs_str
                        } else {
                            false
                        };
                        value
                    }
                } else {
                    false
                }
            }
            Value::Null | Value::Undefined => matches!(rhs, Value::Null | Value::Undefined),
        }
    }

    pub fn strictly_equal(&self, lhs: Value, rhs: Value) -> bool {
        match (lhs, rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => lhs == rhs,
            (Value::Number(lhs), Value::Number(rhs)) => lhs == rhs,
            (Value::Object(lhs), Value::Object(rhs)) => {
                lhs == rhs || {
                    let lhs_obj = self.vm().heap().resolve(lhs);
                    let rhs_obj = self.vm().heap().resolve(rhs);
                    if lhs_obj.as_ref().string_data().is_some()
                        || rhs_obj.as_ref().string_data().is_some()
                    {
                        lhs_obj.as_ref().js_to_string() == rhs_obj.as_ref().js_to_string()
                    } else {
                        false
                    }
                }
            }
            (Value::Null, Value::Null) | (Value::Undefined, Value::Undefined) => true,
            (_, _) => false,
        }
    }

    pub fn compare(&self, lhs: Value, rhs: Value, op: impl FnOnce(cmp::Ordering) -> bool) -> bool {
        let ord = if matches!(lhs, Value::Object(_)) || matches!(rhs, Value::Object(_)) {
            Some(self.coerce_to_string(lhs).cmp(&self.coerce_to_string(rhs)))
        } else {
            self.coerce_to_number(lhs)
                .partial_cmp(&self.coerce_to_number(rhs))
        };
        ord.map_or(false, op)
    }

    pub fn is_truthy(&self, v: Value) -> bool {
        self.coerce_to_bool(v)
    }

    pub fn coerce_to_bool(&self, v: Value) -> bool {
        match v {
            Value::Boolean(v) => v,
            Value::Number(v) => !v.is_zero() && !v.is_nan(),
            Value::Object(obj_ref) => {
                let obj = self.vm().heap().resolve(obj_ref);
                let value = obj
                    .as_ref()
                    .string_data()
                    .map_or(true, |string_data| !string_data.is_empty());
                value
            }
            Value::Null | Value::Undefined => false,
        }
    }

    pub fn coerce_to_number(&self, v: Value) -> Number {
        match v {
            Value::Boolean(v) => Number::Int(i64::from(v)),
            Value::Number(v) => v,
            Value::Object(obj_ref) => {
                let obj = self.vm().heap().resolve(obj_ref);
                let value = obj
                    .as_ref()
                    .string_data()
                    .and_then(|string_data| Number::from_str(string_data).ok())
                    .unwrap_or(Number::NAN);
                value
            }
            Value::Null => Number::Int(0),
            Value::Undefined => Number::NAN,
        }
    }

    pub fn coerce_to_string(&self, v: Value) -> Cow<'static, str> {
        match v {
            Value::Boolean(v) => if v { "true" } else { "false" }.into(),
            Value::Number(v) => v.to_string().into(),
            Value::Object(obj_ref) => {
                let obj = self.vm().heap().resolve(obj_ref);
                let value = obj.as_ref().js_to_string();
                value
            }
            Value::Null => "null".into(),
            Value::Undefined => "undefined".into(),
        }
    }
}
