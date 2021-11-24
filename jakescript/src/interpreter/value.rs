use crate::interpreter::error::NumericOverflowError;
use crate::interpreter::heap::Reference;
use crate::interpreter::Interpreter;
use std::str::FromStr;
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
        fn is_str_or_ref(v: &Value) -> bool {
            matches!(v, Value::String(..) | Value::Reference(..))
        }
        if is_str_or_ref(lhs) || is_str_or_ref(rhs) {
            let mut out = lhs.coerce_to_string(it);
            out.push_str(&rhs.coerce_to_string(it));
            Ok(Self::String(out))
        } else {
            checked_numeric_binary_op(it, lhs, rhs, Number::checked_add)
        }
    }

    pub fn sub(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        checked_numeric_binary_op(it, lhs, rhs, Number::checked_sub)
    }

    pub fn mul(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        checked_numeric_binary_op(it, lhs, rhs, Number::checked_mul)
    }

    pub fn div(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        checked_numeric_binary_op(it, lhs, rhs, Number::checked_div)
    }

    pub fn rem(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        checked_numeric_binary_op(it, lhs, rhs, Number::checked_rem)
    }

    pub fn pow(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        checked_numeric_binary_op(it, lhs, rhs, Number::checked_pow)
    }

    pub fn identical(_it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        let result = match (lhs, rhs) {
            (Self::Boolean(lhs), Self::Boolean(rhs)) => lhs == rhs,
            (Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::Reference(lhs), Self::Reference(rhs)) => lhs == rhs,
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
        if let Self::String(rhs) = rhs {
            return Self::Boolean(lhs.coerce_to_string(it).as_str() == rhs);
        }
        Self::Boolean(match lhs {
            Self::Boolean(lhs) => *lhs == rhs.coerce_to_bool(it),
            Self::Number(lhs) => *lhs == rhs.coerce_to_number(it),
            Self::String(lhs) => lhs == rhs.coerce_to_string(it).as_str(),
            Self::Reference(lhs) => {
                if let Self::Reference(rhs) = rhs {
                    lhs == rhs
                } else {
                    false
                }
            }
            Self::Null | Self::Undefined => matches!(rhs, Self::Null | Self::Undefined),
        })
    }

    pub fn ne(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        let eq = Self::eq(it, lhs, rhs);
        Self::not(it, &eq)
    }

    pub fn lt(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::partial_cmp_op(it, lhs, rhs, cmp::Ordering::is_lt)
    }

    pub fn le(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::partial_cmp_op(it, lhs, rhs, cmp::Ordering::is_le)
    }

    pub fn gt(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::partial_cmp_op(it, lhs, rhs, cmp::Ordering::is_gt)
    }

    pub fn ge(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::partial_cmp_op(it, lhs, rhs, cmp::Ordering::is_ge)
    }

    #[inline]
    fn partial_cmp_op(
        it: &mut Interpreter,
        lhs: &Self,
        rhs: &Self,
        f: fn(cmp::Ordering) -> bool,
    ) -> Self {
        fn is_str_or_ref(v: &Value) -> bool {
            matches!(v, Value::String(..) | Value::Reference(..))
        }
        Self::Boolean(if is_str_or_ref(lhs) || is_str_or_ref(rhs) {
            f(lhs.coerce_to_string(it).cmp(&rhs.coerce_to_string(it)))
        } else {
            lhs.coerce_to_number(it)
                .partial_cmp(&rhs.coerce_to_number(it))
                .map_or(false, f)
        })
    }

    pub fn bitwise_not(it: &mut Interpreter, operand: &Self) -> Self {
        Self::Number(!operand.coerce_to_number(it))
    }

    pub fn bitwise_and(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(lhs.coerce_to_number(it) & rhs.coerce_to_number(it))
    }

    pub fn bitwise_or(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(lhs.coerce_to_number(it) | rhs.coerce_to_number(it))
    }

    pub fn bitwise_xor(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(lhs.coerce_to_number(it) ^ rhs.coerce_to_number(it))
    }

    pub fn bitwise_shl(it: &mut Interpreter, lhs: &Self, rhs: &Self) -> Self {
        Self::Number(lhs.coerce_to_number(it) << rhs.coerce_to_number(it))
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
        Self::Number(operand.coerce_to_number(it))
    }

    pub fn neg(it: &mut Interpreter, operand: &Self) -> Result<Self, NumericOverflowError> {
        operand
            .coerce_to_number(it)
            .checked_neg()
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    pub fn not(it: &mut Interpreter, operand: &Self) -> Self {
        Self::Boolean(!operand.coerce_to_bool(it))
    }

    pub fn is_truthy(&self, it: &mut Interpreter) -> bool {
        self.coerce_to_bool(it)
    }

    pub fn is_falsy(&self, it: &mut Interpreter) -> bool {
        !self.coerce_to_bool(it)
    }

    pub fn coerce_to_bool(&self, _: &Interpreter) -> bool {
        match self {
            Self::Boolean(value) => *value,
            Self::Number(value) => *value != 0,
            Self::String(value) => !value.is_empty(),
            Self::Reference(..) => true,
            Self::Null | Self::Undefined => false,
        }
    }

    pub fn coerce_to_number(&self, _: &Interpreter) -> Number {
        match self {
            Self::Boolean(value) => Number::Int(if *value { 1 } else { 0 }),
            Self::Number(value) => *value,
            Self::String(value) => Number::from_str(value).unwrap_or(Number::NaN),
            Self::Null => Number::Int(0),
            Self::Reference(..) | Self::Undefined => Number::NaN,
        }
    }

    pub fn coerce_to_string(&self, it: &Interpreter) -> String {
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

impl FromStr for Number {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i64::from_str(s).map(Self::Int)
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

impl From<&str> for Number {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap_or(Self::NaN)
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
fn checked_numeric_binary_op(
    it: &mut Interpreter,
    lhs: &Value,
    rhs: &Value,
    op: fn(Number, Number) -> Option<Number>,
) -> Result<Value, NumericOverflowError> {
    op(lhs.coerce_to_number(it), rhs.coerce_to_number(it))
        .map(Value::Number)
        .ok_or(NumericOverflowError)
}
