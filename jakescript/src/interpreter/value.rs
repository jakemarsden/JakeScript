use crate::interpreter::error::NumericOverflowError;
use crate::interpreter::heap::Reference;
use crate::interpreter::Interpreter;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl};
use std::{cmp, fmt, num, ops};

#[derive(Clone, Default, Debug)]
pub enum Value {
    Boolean(bool),
    Number(Number),
    String(String),
    Reference(Reference),
    Null,
    #[default]
    Undefined,
}

impl Value {
    /// # Panics
    ///
    /// Panics if certain coercions are needed.
    pub fn add(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        match lhs {
            Value::String(ref lhs) => {
                let rhs = rhs.coerce_to_string_impl(it);
                let mut result = String::with_capacity(lhs.len() + rhs.len());
                result.push_str(lhs);
                result.push_str(&rhs);
                Ok(Value::String(result))
            }
            Value::Reference(_) => todo!("Value::add: {:?} + {:?}", lhs, rhs),
            _ => numeric_bin_op(it, lhs, rhs, Number::checked_add)
                .map(Self::Number)
                .ok_or(NumericOverflowError),
        }
    }

    pub fn sub(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        numeric_bin_op(it, lhs, rhs, Number::checked_sub)
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    pub fn mul(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        numeric_bin_op(it, lhs, rhs, Number::checked_mul)
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    pub fn div(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        numeric_bin_op(it, lhs, rhs, Number::checked_div)
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    pub fn rem(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        numeric_bin_op(it, lhs, rhs, Number::checked_rem)
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    pub fn pow(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        numeric_bin_op(it, lhs, rhs, Number::checked_pow)
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    pub fn identical(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        let result = match (lhs, rhs) {
            (Self::Boolean(lhs), Self::Boolean(rhs)) => lhs == rhs,
            (Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::Reference(lhs_ref), Self::Reference(rhs_ref)) => {
                lhs_ref == rhs_ref || {
                    let lhs_obj = it.vm().heap().resolve(lhs_ref);
                    let rhs_obj = it.vm().heap().resolve(rhs_ref);
                    lhs_obj.js_equals(&*rhs_obj)
                }
            }
            (Self::Null, Self::Null) | (Self::Undefined, Self::Undefined) => true,
            (_, _) => false,
        };
        Self::Boolean(result)
    }

    pub fn not_identical(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        let identical = Self::identical(it, lhs, rhs);
        Self::not(it, &identical)
    }

    pub fn eq(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        match lhs {
            Self::Boolean(_) => {
                let rhs = rhs.coerce_to_boolean(it);
                Self::identical(it, lhs, &rhs)
            }
            Self::Number(_) => {
                let rhs = rhs.coerce_to_number(it);
                Self::identical(it, lhs, &rhs)
            }
            Self::String(_) => {
                let rhs = rhs.coerce_to_string(it);
                Self::identical(it, lhs, &rhs)
            }
            Self::Reference(_) => Self::identical(it, lhs, rhs),
            Self::Null | Self::Undefined => {
                Self::Boolean(matches!(rhs, Self::Null | Self::Undefined))
            }
        }
    }

    pub fn ne(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        let eq = Self::identical(it, lhs, rhs);
        Self::not(it, &eq)
    }

    pub fn lt(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Boolean(numeric_bin_op(it, lhs, rhs, |lhs, rhs| lhs < rhs))
    }

    pub fn le(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Boolean(numeric_bin_op(it, lhs, rhs, |lhs, rhs| lhs <= rhs))
    }

    pub fn gt(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Boolean(numeric_bin_op(it, lhs, rhs, |lhs, rhs| lhs > rhs))
    }

    pub fn ge(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Boolean(numeric_bin_op(it, lhs, rhs, |lhs, rhs| lhs >= rhs))
    }

    pub fn bitwise_not(it: &mut Interpreter, operand: &Self) -> Self {
        Self::Number(numeric_uni_op(it, operand, Number::not))
    }

    pub fn bitwise_and(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(numeric_bin_op(it, lhs, rhs, Number::bitand))
    }

    pub fn bitwise_or(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(numeric_bin_op(it, lhs, rhs, Number::bitor))
    }

    pub fn bitwise_xor(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(numeric_bin_op(it, lhs, rhs, Number::bitxor))
    }

    pub fn bitwise_shl(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(numeric_bin_op(it, lhs, rhs, Number::shl))
    }

    /// # Panics
    ///
    /// Always panics.
    pub fn bitwise_shr(_it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        todo!("Interpreter::bitwise_shr: lhs={:?}, rhs={:?}", lhs, rhs)
    }

    /// # Panics
    ///
    /// Always panics.
    pub fn bitwise_shrr(_it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        todo!("Interpreter::bitwise_shrr: lhs={:?}, rhs={:?}", lhs, rhs)
    }

    pub fn plus(it: &mut Interpreter, operand: &Self) -> Self {
        Self::Number(numeric_uni_op(it, operand, |operand| operand))
    }

    pub fn neg(it: &mut Interpreter, operand: &Self) -> Result<Self, NumericOverflowError> {
        numeric_uni_op(it, operand, Number::checked_neg)
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    pub fn not(it: &mut Interpreter, operand: &Self) -> Self {
        Self::Boolean(boolean_uni_op(it, operand, bool::not))
    }

    pub fn is_truthy(&self, it: &mut Interpreter) -> bool {
        self.coerce_to_boolean_impl(it)
    }

    pub fn is_falsy(&self, it: &mut Interpreter) -> bool {
        !self.coerce_to_boolean_impl(it)
    }

    pub fn coerce_to_boolean(&self, it: &mut Interpreter) -> Self {
        Self::Boolean(self.coerce_to_boolean_impl(it))
    }

    fn coerce_to_boolean_impl(&self, _it: &mut Interpreter) -> bool {
        match self {
            Self::Boolean(value) => *value,
            Self::Number(value) => *value > 0,
            Self::String(value) => !value.is_empty(),
            Self::Reference(_) => true,
            Self::Null | Self::Undefined => false,
        }
    }

    pub fn coerce_to_number(&self, it: &mut Interpreter) -> Self {
        Self::Number(self.coerce_to_number_impl(it))
    }

    fn coerce_to_number_impl(&self, _it: &mut Interpreter) -> Number {
        match self {
            Self::Boolean(value) => Number::Int(if *value { 1 } else { 0 }),
            Self::Number(value) => *value,
            Self::Null => Number::Int(0),
            Self::Reference(..) | Self::Undefined => Number::NaN,
            Self::String(..) => todo!("Value::coerce_to_number: {:?}", self),
        }
    }

    pub fn coerce_to_string(&self, it: &mut Interpreter) -> Self {
        Self::String(self.coerce_to_string_impl(it))
    }

    fn coerce_to_string_impl(&self, it: &mut Interpreter) -> String {
        match self {
            Self::Boolean(value) => value.to_string(),
            Self::Number(value) => value.to_string(),
            Self::String(value) => value.clone(),
            Self::Reference(obj_ref) => {
                let obj = it.vm().heap().resolve(obj_ref);
                obj.js_to_string()
            }
            Self::Null => "null".to_owned(),
            Self::Undefined => "undefined".to_owned(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Number(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::Reference(value) => write!(f, "{}", value),
            Self::Null => f.write_str("null"),
            Self::Undefined => f.write_str("undefined"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Number {
    Int(i64),
    NaN,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(value) => write!(f, "{}", value),
            Self::NaN => f.write_str("NaN"),
        }
    }
}

impl Number {
    pub fn checked_neg(self) -> Option<Self> {
        self.checked_unary_op(i64::checked_neg)
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.checked_binary_op(rhs, i64::checked_add)
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_binary_op(rhs, i64::checked_sub)
    }

    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        self.checked_binary_op(rhs, i64::checked_mul)
    }

    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        self.checked_binary_op(rhs, i64::checked_div)
    }

    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        self.checked_binary_op(rhs, i64::checked_rem)
    }

    pub fn checked_pow(self, rhs: Self) -> Option<Self> {
        self.checked_binary_op(rhs, checked_pow)
    }

    #[inline]
    fn unary_op(self, f: fn(i64) -> i64) -> Self {
        match self {
            Self::Int(operand) => Self::Int(f(operand)),
            Self::NaN => Self::NaN,
        }
    }

    #[inline]
    fn binary_op(self, rhs: Self, f: fn(i64, i64) -> i64) -> Self {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(f(lhs, rhs)),
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
        }
    }

    #[inline]
    fn checked_unary_op(self, f: fn(i64) -> Option<i64>) -> Option<Self> {
        match self {
            Self::Int(operand) => f(operand).map(Self::Int),
            Self::NaN => Some(Self::NaN),
        }
    }

    #[inline]
    fn checked_binary_op(self, rhs: Self, f: fn(i64, i64) -> Option<i64>) -> Option<Self> {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => f(lhs, rhs).map(Self::Int),
            (Self::NaN, _) | (_, Self::NaN) => Some(Self::NaN),
        }
    }
}

impl ops::Not for Number {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.unary_op(i64::not)
    }
}

impl ops::BitAnd for Number {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, i64::bitand)
    }
}

impl ops::BitOr for Number {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, i64::bitor)
    }
}

impl ops::BitXor for Number {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, i64::bitxor)
    }
}

impl cmp::PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::NaN, _) | (_, Self::NaN) => false,
        }
    }
}

impl ops::Shl for Number {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, i64::shl)
    }
}

impl cmp::PartialEq<i64> for Number {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Self::Int(lhs) => lhs == other,
            Self::NaN => false,
        }
    }
}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => Some(i64::cmp(lhs, rhs)),
            (Self::NaN, _) | (_, Self::NaN) => None,
        }
    }
}

impl cmp::PartialOrd<i64> for Number {
    fn partial_cmp(&self, other: &i64) -> Option<cmp::Ordering> {
        match self {
            Self::Int(lhs) => Some(i64::cmp(lhs, other)),
            Self::NaN => None,
        }
    }
}

impl TryFrom<u64> for Number {
    type Error = num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        i64::try_from(value).map(Self::Int)
    }
}

// cast_precision_loss, cast_possible_truncation: TODO: Handle floating-point properly
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn checked_pow(lhs: i64, rhs: i64) -> Option<i64> {
    if i32::try_from(rhs).is_ok() {
        Some((lhs as f64).powi(rhs as i32) as i64)
    } else {
        None
    }
}

#[inline]
fn boolean_uni_op<R>(it: &mut Interpreter, operand: &Value, op: fn(bool) -> R) -> R {
    let operand = operand.coerce_to_boolean_impl(it);
    op(operand)
}

#[inline]
fn numeric_uni_op<R>(it: &mut Interpreter, operand: &Value, op: fn(Number) -> R) -> R {
    let operand = operand.coerce_to_number_impl(it);
    op(operand)
}

#[inline]
fn numeric_bin_op<R>(
    it: &mut Interpreter,
    lhs: &Value,
    rhs: &Value,
    op: fn(Number, Number) -> R,
) -> R {
    let lhs = lhs.coerce_to_number_impl(it);
    let rhs = rhs.coerce_to_number_impl(it);
    op(lhs, rhs)
}
