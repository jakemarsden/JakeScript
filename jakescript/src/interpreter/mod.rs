pub use error::*;
pub use heap::*;
pub use object::*;
pub use stack::*;
pub use value::*;
pub use vm::*;

use crate::ast::*;
use crate::runtime::{Builtin, NativeCall};
use std::cmp;
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::str::FromStr;

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

pub trait Eval: Node {
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

    pub fn alloc_string(
        &mut self,
        s: Box<str>,
    ) -> std::result::Result<Reference, OutOfMemoryError> {
        let proto = self.vm().runtime().global_object().string().as_obj_ref();
        self.vm_mut()
            .heap_mut()
            .allocate(Object::new_string(proto, s, Extensible::Yes))
    }

    pub fn alloc_array(
        &mut self,
        elems: Vec<Value>,
    ) -> std::result::Result<Reference, OutOfMemoryError> {
        let proto = self.vm().runtime().global_object().array().as_obj_ref();
        self.vm_mut()
            .heap_mut()
            .allocate(Object::new_array(proto, elems, Extensible::Yes))
    }

    pub fn alloc_object(
        &mut self,
        props: HashMap<PropertyKey, Value>,
    ) -> std::result::Result<Reference, OutOfMemoryError> {
        self.vm_mut()
            .heap_mut()
            .allocate(Object::new_object(None, props, Extensible::Yes))
    }

    pub fn alloc_function(
        &mut self,
        f: UserFunction,
    ) -> std::result::Result<Reference, OutOfMemoryError> {
        self.vm_mut()
            .heap_mut()
            .allocate(Object::new_function(f, Extensible::Yes))
    }

    pub fn update_variable_or_global_object_property(
        &mut self,
        key: &Identifier,
        f: impl FnOnce(&mut Self, &Value) -> Result<(Value, Value)>,
        e: impl FnOnce(ErrorKind) -> Error,
    ) -> Result {
        if let Some(mut var) = self.vm().stack().frame().scope().lookup_variable(key) {
            let value = var.value().clone();
            let (result_value, updated_value) = f(self, &value)?;
            var.set_value(updated_value)
                .map_err(ErrorKind::from)
                .map_err(e)?;
            Ok(result_value)
        } else {
            let global_obj_ref = self.vm().runtime().global_object_ref().clone();
            self.update_object_property(&global_obj_ref, key, f, e)
        }
    }

    pub fn update_object_property(
        &mut self,
        base_ref: &Reference,
        key: &PropertyKey,
        f: impl FnOnce(&mut Self, &Value) -> Result<(Value, Value)>,
        e: impl FnOnce(ErrorKind) -> Error,
    ) -> Result {
        let value = {
            let base_obj = self.vm().heap().resolve(base_ref);
            match base_obj.get(self, key, base_ref.clone()) {
                Ok(value) => value.unwrap_or_default(),
                Err(err) => return Err(e(err)),
            }
        };
        let (result_value, updated_value) = f(self, &value)?;
        self.vm_mut()
            .heap_mut()
            .resolve_mut(base_ref)
            .set(self, key, base_ref.clone(), updated_value)
            .map_err(e)?;
        Ok(result_value)
    }

    pub fn call_user_fn(
        &mut self,
        f: &UserFunction,
        fn_obj_ref: &Reference,
        receiver: Option<Reference>,
        args: &[Value],
    ) -> Result {
        let declared_params = f.declared_parameters();
        let mut supplied_args = args.iter();
        let mut variables = Vec::with_capacity(declared_params.len());
        for declared_param_name in declared_params.iter() {
            let arg_value = supplied_args.next().cloned().unwrap_or_default();
            variables.push(Variable::new(
                VariableKind::Let,
                declared_param_name.clone(),
                arg_value,
            ));
        }

        let declared_scope = f.declared_scope().clone();
        let fn_scope_ctx = ScopeCtx::new(variables);

        self.vm_mut()
            .stack_mut()
            .push_frame(declared_scope, receiver);
        if let Some(fn_name) = f.name() {
            // Create an outer scope with nothing but the function's name, which points to itself,
            // so that named function literals may recurse using their name without making the name
            // visible outside of the function body. It has its own outer scope so it can still be
            // shadowed by parameters with the same name.
            let fn_scope_ctx_outer = ScopeCtx::new(vec![Variable::new(
                VariableKind::Var,
                fn_name.clone(),
                Value::Object(fn_obj_ref.clone()),
            )]);
            self.vm_mut()
                .stack_mut()
                .frame_mut()
                .push_scope(fn_scope_ctx_outer, false);
        }
        self.vm_mut()
            .stack_mut()
            .frame_mut()
            .push_scope(fn_scope_ctx, true);
        f.body().eval(self)?;
        self.vm_mut().stack_mut().frame_mut().pop_scope();
        if f.name().is_some() {
            self.vm_mut().stack_mut().frame_mut().pop_scope();
        }
        self.vm_mut().stack_mut().pop_frame();

        Ok(match self.vm_mut().reset_execution_state() {
            ExecutionState::Advance | ExecutionState::Exit => Value::Undefined,
            ExecutionState::Return(value) => value,
            execution_state => unreachable!("Unexpected execution state: {:?}", execution_state),
        })
    }

    pub fn call_native_fn(
        &mut self,
        f: &NativeCall,
        receiver: Option<Reference>,
        args: &[Value],
    ) -> std::result::Result<Value, ErrorKind> {
        let receiver = receiver.unwrap_or_else(|| self.vm().runtime().global_object_ref().clone());
        f.call(self, receiver, args)
    }

    pub fn eval_binary_op(
        &mut self,
        op: BinaryOperator,
        lhs: &Value,
        rhs: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        match op {
            BinaryOperator::Addition => self.add_or_concat(lhs, rhs),
            BinaryOperator::Subtraction => self.sub(lhs, rhs),
            BinaryOperator::Multiplication => self.mul(lhs, rhs),
            BinaryOperator::Division => self.div(lhs, rhs),
            BinaryOperator::Modulus => self.rem(lhs, rhs),
            BinaryOperator::Exponentiation => self.pow(lhs, rhs),
            BinaryOperator::BitwiseAnd => self.bitand(lhs, rhs),
            BinaryOperator::BitwiseOr => self.bitor(lhs, rhs),
            BinaryOperator::BitwiseXOr => self.bitxor(lhs, rhs),
            BinaryOperator::LogicalAnd => {
                Ok(Value::Boolean(self.is_truthy(lhs) && self.is_truthy(rhs)))
            }
            BinaryOperator::LogicalOr => {
                Ok(Value::Boolean(self.is_truthy(lhs) || self.is_truthy(rhs)))
            }
            BinaryOperator::BitwiseLeftShift => self.shl(lhs, rhs),
            BinaryOperator::BitwiseRightShift => self.shr_signed(lhs, rhs),
            BinaryOperator::BitwiseRightShiftUnsigned => self.shr_unsigned(lhs, rhs),
        }
    }

    pub fn eval_relational_op(
        &mut self,
        op: RelationalOperator,
        lhs: &Value,
        rhs: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        match op {
            RelationalOperator::Equality => self.equal(lhs, rhs),
            RelationalOperator::Inequality => self.not_equal(lhs, rhs),
            RelationalOperator::StrictEquality => self.strictly_equal(lhs, rhs),
            RelationalOperator::StrictInequality => self.not_strictly_equal(lhs, rhs),
            RelationalOperator::GreaterThan => self.gt(lhs, rhs),
            RelationalOperator::GreaterThanOrEqual => self.ge(lhs, rhs),
            RelationalOperator::LessThan => self.lt(lhs, rhs),
            RelationalOperator::LessThanOrEqual => self.le(lhs, rhs),
        }
    }

    pub fn eval_unary_op(
        &mut self,
        op: UnaryOperator,
        operand: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        match op {
            UnaryOperator::NumericPlus => self.plus(operand),
            UnaryOperator::NumericNegation => self.negate(operand),
            UnaryOperator::BitwiseNot => self.bitnot(operand),
            UnaryOperator::LogicalNot => self.not(operand),
        }
    }

    pub fn eval_update_op(
        &mut self,
        op: UpdateOperator,
        operand: &Value,
    ) -> std::result::Result<(Value, Value), ErrorKind> {
        let one = Value::Number(Number::Int(1));
        match op {
            UpdateOperator::GetAndIncrement => self
                .add(operand, &one)
                .map(|updated| (operand.clone(), updated)),
            UpdateOperator::IncrementAndGet => self
                .add(operand, &one)
                .map(|updated| (updated.clone(), updated)),
            UpdateOperator::GetAndDecrement => self
                .sub(operand, &one)
                .map(|updated| (operand.clone(), updated)),
            UpdateOperator::DecrementAndGet => self
                .sub(operand, &one)
                .map(|updated| (updated.clone(), updated)),
        }
    }

    pub fn add_or_concat(
        &mut self,
        lhs: &Value,
        rhs: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        if matches!(lhs, Value::Object(_)) || matches!(rhs, Value::Object(_)) {
            self.concat(lhs, rhs)
        } else {
            self.add(lhs, rhs)
        }
    }

    fn add(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_add)
    }

    // unnecessary_wraps: Future-proofing
    #[allow(clippy::unnecessary_wraps)]
    fn concat(&mut self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        let out = format!(
            "{}{}",
            self.coerce_to_string(lhs),
            self.coerce_to_string(rhs)
        );
        self.alloc_string(out.into_boxed_str())
            .map(Value::Object)
            .map_err(ErrorKind::from)
    }

    pub fn sub(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_sub)
    }

    pub fn mul(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_mul)
    }

    pub fn div(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_div)
    }

    pub fn rem(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_rem)
    }

    pub fn pow(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_binary_op(lhs, rhs, Number::pow)
    }

    pub fn bitand(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_binary_op(lhs, rhs, Number::bitand)
    }

    pub fn bitor(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_binary_op(lhs, rhs, Number::bitor)
    }

    pub fn bitxor(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_binary_op(lhs, rhs, Number::bitxor)
    }

    pub fn shl(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_shl)
    }

    pub fn shr_signed(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_shr_signed)
    }

    pub fn shr_unsigned(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_binary_op(lhs, rhs, Number::checked_shr_unsigned)
    }

    pub fn equal(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Boolean(match lhs {
            Value::Boolean(lhs) => *lhs == self.coerce_to_bool(rhs),
            Value::Number(lhs) => *lhs == self.coerce_to_number(rhs),
            Value::Object(lhs) => {
                if let Value::Object(rhs) = rhs {
                    lhs == rhs || {
                        let lhs_obj = self.vm().heap().resolve(lhs);
                        let rhs_obj = self.vm().heap().resolve(rhs);
                        if let Some(lhs_str) = lhs_obj.string_data()
                            && let Some(rhs_str) = rhs_obj.string_data()
                        {
                            lhs_str == rhs_str
                        } else {
                            false
                        }
                    }
                } else {
                    false
                }
            }
            Value::Null | Value::Undefined => matches!(rhs, Value::Null | Value::Undefined),
        }))
    }

    pub fn not_equal(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.equal(lhs, rhs).and_then(|ref result| self.not(result))
    }

    // unused_self: Will be used when string values are stored on the heap.
    #[allow(clippy::unused_self)]
    pub fn strictly_equal(
        &self,
        lhs: &Value,
        rhs: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Boolean(match (lhs, rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => lhs == rhs,
            (Value::Number(lhs), Value::Number(rhs)) => lhs == rhs,
            (Value::Object(lhs), Value::Object(rhs)) => {
                lhs == rhs || {
                    let lhs_obj = self.vm().heap().resolve(lhs);
                    let rhs_obj = self.vm().heap().resolve(rhs);
                    if lhs_obj.string_data().is_some() || rhs_obj.string_data().is_some() {
                        lhs_obj.js_to_string() == rhs_obj.js_to_string()
                    } else {
                        false
                    }
                }
            }
            (Value::Null, Value::Null) | (Value::Undefined, Value::Undefined) => true,
            (_, _) => false,
        }))
    }

    pub fn not_strictly_equal(
        &self,
        lhs: &Value,
        rhs: &Value,
    ) -> std::result::Result<Value, ErrorKind> {
        self.strictly_equal(lhs, rhs)
            .and_then(|ref result| self.not(result))
    }

    pub fn gt(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.comparison_op(lhs, rhs, cmp::Ordering::is_gt)
    }

    pub fn ge(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.comparison_op(lhs, rhs, cmp::Ordering::is_ge)
    }

    pub fn lt(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.comparison_op(lhs, rhs, cmp::Ordering::is_lt)
    }

    pub fn le(&self, lhs: &Value, rhs: &Value) -> std::result::Result<Value, ErrorKind> {
        self.comparison_op(lhs, rhs, cmp::Ordering::is_le)
    }

    pub fn plus(&self, operand: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_unary_op(operand, |operand| operand)
    }

    pub fn negate(&self, operand: &Value) -> std::result::Result<Value, ErrorKind> {
        self.checked_numeric_unary_op(operand, Number::checked_neg)
    }

    pub fn bitnot(&self, operand: &Value) -> std::result::Result<Value, ErrorKind> {
        self.numeric_unary_op(operand, Number::not)
    }

    pub fn not(&self, operand: &Value) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Boolean(!self.coerce_to_bool(operand)))
    }

    pub fn is_truthy(&self, v: &Value) -> bool {
        self.coerce_to_bool(v)
    }

    // unused_self: Will be used when string values are stored on the heap.
    #[allow(clippy::unused_self)]
    pub fn coerce_to_bool(&self, v: &Value) -> bool {
        match v {
            Value::Boolean(v) => *v,
            Value::Number(v) => !v.is_zero() && !v.is_nan(),
            Value::Object(obj_ref) => {
                let obj = self.vm().heap().resolve(obj_ref);
                obj.string_data()
                    .map_or(true, |string_data| !string_data.is_empty())
            }
            Value::Null | Value::Undefined => false,
        }
    }

    // unused_self: Will be used when string values are stored on the heap.
    #[allow(clippy::unused_self)]
    pub fn coerce_to_number(&self, v: &Value) -> Number {
        match v {
            Value::Boolean(v) => Number::Int(if *v { 1 } else { 0 }),
            Value::Number(v) => *v,
            Value::Object(obj_ref) => {
                let obj = self.vm().heap().resolve(obj_ref);
                obj.string_data()
                    .and_then(|string_data| Number::from_str(string_data).ok())
                    .unwrap_or(Number::NAN)
            }
            Value::Null => Number::Int(0),
            Value::Undefined => Number::NAN,
        }
    }

    pub fn coerce_to_string(&self, v: &Value) -> Box<str> {
        match v {
            Value::Boolean(v) => v.to_string().into_boxed_str(),
            Value::Number(v) => v.to_string().into_boxed_str(),
            Value::Object(obj_ref) => {
                let obj = self.vm().heap().resolve(obj_ref);
                obj.js_to_string()
            }
            Value::Null => Box::from("null"),
            Value::Undefined => Box::from("undefined"),
        }
    }

    #[inline]
    fn checked_numeric_unary_op(
        &self,
        operand: &Value,
        checked_op: impl FnOnce(Number) -> Option<Number>,
    ) -> std::result::Result<Value, ErrorKind> {
        checked_op(self.coerce_to_number(operand))
            .map(Value::Number)
            .ok_or(ErrorKind::NumericOverflow(NumericOverflowError))
    }

    // unnecessary_wraps: Future-proofing
    #[allow(clippy::unnecessary_wraps)]
    #[inline]
    fn numeric_unary_op(
        &self,
        operand: &Value,
        op: impl FnOnce(Number) -> Number,
    ) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Number(op(self.coerce_to_number(operand))))
    }

    #[inline]
    fn checked_numeric_binary_op(
        &self,
        lhs: &Value,
        rhs: &Value,
        checked_op: impl FnOnce(Number, Number) -> Option<Number>,
    ) -> std::result::Result<Value, ErrorKind> {
        checked_op(self.coerce_to_number(lhs), self.coerce_to_number(rhs))
            .map(Value::Number)
            .ok_or(ErrorKind::NumericOverflow(NumericOverflowError))
    }

    // unnecessary_wraps: Future-proofing
    #[allow(clippy::unnecessary_wraps)]
    #[inline]
    fn numeric_binary_op(
        &self,
        lhs: &Value,
        rhs: &Value,
        op: impl FnOnce(Number, Number) -> Number,
    ) -> std::result::Result<Value, ErrorKind> {
        Ok(Value::Number(op(
            self.coerce_to_number(lhs),
            self.coerce_to_number(rhs),
        )))
    }

    // unnecessary_wraps: Future-proofing
    #[allow(clippy::unnecessary_wraps)]
    fn comparison_op(
        &self,
        lhs: &Value,
        rhs: &Value,
        op: impl FnOnce(cmp::Ordering) -> bool,
    ) -> std::result::Result<Value, ErrorKind> {
        let ord = if matches!(lhs, Value::Object(_)) || matches!(rhs, Value::Object(_)) {
            Some(self.coerce_to_string(lhs).cmp(&self.coerce_to_string(rhs)))
        } else {
            self.coerce_to_number(lhs)
                .partial_cmp(&self.coerce_to_number(rhs))
        };
        Ok(Value::Boolean(ord.map_or(false, op)))
    }
}
