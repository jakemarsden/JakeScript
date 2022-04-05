use super::error::{Error, Result};
use super::heap::{Object, UserFunction};
use super::value::{Number, Value};
use super::{Eval, Interpreter};
use crate::ast::*;
use std::collections::HashMap;

impl Eval for LiteralExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        Ok(match self.value {
            Literal::Boolean(value) => Value::Boolean(value),
            Literal::Numeric(value) => Value::Number(match value {
                NumericLiteral::Int(value) => Number::Int(i64::try_from(value).unwrap()),
                NumericLiteral::Float(value) => Number::Float(value),
            }),
            Literal::String(ref value) => Value::Object(
                it.vm_mut()
                    .heap_mut()
                    .allocate(Object::new_string(value.value.clone()))
                    .map_err(|err| Error::new(err, self.source_location()))?,
            ),
            Literal::Null => Value::Null,
        })
    }
}

impl Eval for ArrayExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let mut elems = Vec::with_capacity(self.declared_elements.len());
        for elem_expr in &self.declared_elements {
            elems.push(elem_expr.eval(it)?);
        }
        let obj_ref = it
            .vm_mut()
            .heap_mut()
            .allocate(Object::new_array(elems))
            .map_err(|err| Error::new(err, self.source_location()))?;
        Ok(Value::Object(obj_ref))
    }
}

impl Eval for ObjectExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let mut resolved_props = HashMap::with_capacity(self.declared_properties.len());
        for prop in &self.declared_properties {
            let name = match prop.name {
                DeclaredPropertyName::Identifier(ref value) => value.inner().clone(),
                DeclaredPropertyName::NumericLiteral(..)
                | DeclaredPropertyName::StringLiteral(..)
                | DeclaredPropertyName::Computed(..) => todo!(
                    "ObjectExpression::eval: Non-identifier property name: {:?}",
                    prop.name,
                ),
            };
            let value = prop.initialiser.eval(it)?;
            resolved_props.insert(name, value);
        }
        let obj_ref = it
            .vm_mut()
            .heap_mut()
            .allocate(Object::new_object(resolved_props))
            .map_err(|err| Error::new(err, self.source_location()))?;
        Ok(Value::Object(obj_ref))
    }
}

impl Eval for FunctionExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let declared_scope = it.vm().stack().frame().scope().clone();
        let fn_obj = Object::new_function(UserFunction::new(
            self.binding.clone(),
            self.formal_parameters.clone(),
            declared_scope,
            self.body.clone(),
        ));
        let fn_obj_ref = it
            .vm_mut()
            .heap_mut()
            .allocate(fn_obj)
            .map_err(|err| Error::new(err, self.source_location()))?;
        Ok(Value::Object(fn_obj_ref))
    }
}
