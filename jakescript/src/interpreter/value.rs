use super::error::NumericOverflowError;
use super::heap::Reference;
use super::vm::Vm;
use crate::runtime::NativeRef;
use std::ops::{self, BitAnd, BitOr, BitXor};
use std::str::FromStr;
use std::{cmp, fmt, num};

#[derive(Clone, Default, Debug)]
pub enum Value {
    Boolean(bool),
    Number(Number),
    String(String),
    Reference(Reference),
    NativeObject(NativeRef),
    Null,
    #[default]
    Undefined,
}

impl Value {
    pub fn add_or_append(vm: &Vm, lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        if lhs.is_str_or_ref() || rhs.is_str_or_ref() {
            let mut out = lhs.coerce_to_string(vm);
            out.push_str(&rhs.coerce_to_string(vm));
            Ok(Self::String(out))
        } else {
            Self::checked_numeric_binary_op(lhs, rhs, Number::checked_add)
        }
    }

    pub fn sub(lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        Self::checked_numeric_binary_op(lhs, rhs, Number::checked_sub)
    }

    pub fn mul(lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        Self::checked_numeric_binary_op(lhs, rhs, Number::checked_mul)
    }

    pub fn div(lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        Self::checked_numeric_binary_op(lhs, rhs, Number::checked_div)
    }

    pub fn rem(lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        Self::checked_numeric_binary_op(lhs, rhs, Number::checked_rem)
    }

    pub fn pow(lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        Ok(Self::numeric_binary_op(lhs, rhs, Number::pow))
    }

    pub fn identical(_vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        let result = match (lhs, rhs) {
            (Self::Boolean(lhs), Self::Boolean(rhs)) => lhs == rhs,
            (Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::Reference(lhs), Self::Reference(rhs)) => lhs == rhs,
            (Self::NativeObject(lhs), Self::NativeObject(rhs)) => lhs == rhs,
            (Self::Null, Self::Null) | (Self::Undefined, Self::Undefined) => true,
            (_, _) => false,
        };
        Self::Boolean(result)
    }

    pub fn not_identical(vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        let identical = Self::identical(vm, lhs, rhs);
        Self::not(&identical)
    }

    pub fn eq(vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        if let Self::String(rhs) = rhs {
            return Self::Boolean(lhs.coerce_to_string(vm).as_str() == rhs);
        }
        Self::Boolean(match lhs {
            Self::Boolean(lhs) => *lhs == rhs.coerce_to_bool(),
            Self::Number(lhs) => *lhs == rhs.coerce_to_number(),
            Self::String(lhs) => lhs == rhs.coerce_to_string(vm).as_str(),
            Self::Reference(lhs) => {
                if let Self::Reference(rhs) = rhs {
                    lhs == rhs
                } else {
                    false
                }
            }
            Self::NativeObject(lhs) => {
                if let Self::NativeObject(rhs) = rhs {
                    lhs == rhs
                } else {
                    false
                }
            }
            Self::Null | Self::Undefined => matches!(rhs, Self::Null | Self::Undefined),
        })
    }

    pub fn ne(vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        let eq = Self::eq(vm, lhs, rhs);
        Self::not(&eq)
    }

    pub fn lt(vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        Self::partial_cmp_op(vm, lhs, rhs, cmp::Ordering::is_lt)
    }

    pub fn le(vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        Self::partial_cmp_op(vm, lhs, rhs, cmp::Ordering::is_le)
    }

    pub fn gt(vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        Self::partial_cmp_op(vm, lhs, rhs, cmp::Ordering::is_gt)
    }

    pub fn ge(vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        Self::partial_cmp_op(vm, lhs, rhs, cmp::Ordering::is_ge)
    }

    pub fn bitnot(operand: &Self) -> Self {
        Self::Number(!operand.coerce_to_number())
    }

    pub fn bitand(lhs: &Self, rhs: &Self) -> Self {
        Self::numeric_binary_op(lhs, rhs, Number::bitand)
    }

    pub fn bitor(lhs: &Self, rhs: &Self) -> Self {
        Self::numeric_binary_op(lhs, rhs, Number::bitor)
    }

    pub fn bitxor(lhs: &Self, rhs: &Self) -> Self {
        Self::numeric_binary_op(lhs, rhs, Number::bitxor)
    }

    pub fn shl(lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        Self::checked_numeric_binary_op(lhs, rhs, Number::checked_shl)
    }

    pub fn shr_signed(lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        Self::checked_numeric_binary_op(lhs, rhs, Number::checked_shr_signed)
    }

    pub fn shr_unsigned(lhs: &Self, rhs: &Self) -> Result<Self, NumericOverflowError> {
        Self::checked_numeric_binary_op(lhs, rhs, Number::checked_shr_unsigned)
    }

    pub fn plus(operand: &Self) -> Self {
        Self::Number(operand.coerce_to_number())
    }

    pub fn neg(operand: &Self) -> Result<Self, NumericOverflowError> {
        operand
            .coerce_to_number()
            .checked_neg()
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    pub fn not(operand: &Self) -> Self {
        Self::Boolean(!operand.coerce_to_bool())
    }

    pub fn is_truthy(&self) -> bool {
        self.coerce_to_bool()
    }

    pub fn is_falsy(&self) -> bool {
        !self.coerce_to_bool()
    }

    fn is_str_or_ref(&self) -> bool {
        matches!(self, Self::String(..) | Self::Reference(..))
    }

    pub fn coerce_to_bool(&self) -> bool {
        match self {
            Self::Boolean(value) => *value,
            Self::Number(value) => !value.is_zero() && !value.is_nan(),
            Self::String(value) => !value.is_empty(),
            Self::Reference(..) | Self::NativeObject(_) => true,
            Self::Null | Self::Undefined => false,
        }
    }

    pub fn coerce_to_number(&self) -> Number {
        match self {
            Self::Boolean(value) => Number::Int(if *value { 1 } else { 0 }),
            Self::Number(value) => *value,
            Self::String(value) => Number::from_str(value).unwrap_or(Number::NAN),
            Self::Null => Number::Int(0),
            Self::Reference(..) | Self::NativeObject(_) | Self::Undefined => Number::NAN,
        }
    }

    pub fn coerce_to_string(&self, vm: &Vm) -> String {
        match self {
            Self::Boolean(value) => value.to_string(),
            Self::Number(value) => value.to_string(),
            Self::String(value) => value.clone(),
            Self::Reference(obj_ref) => {
                let obj = vm.heap().resolve(obj_ref);
                obj.js_to_string()
            }
            Self::NativeObject(obj_ref) => {
                let obj = vm.runtime().resolve(obj_ref);
                obj.to_js_string()
            }
            Self::Null => "null".to_owned(),
            Self::Undefined => "undefined".to_owned(),
        }
    }

    #[inline]
    fn numeric_binary_op(lhs: &Self, rhs: &Self, op: fn(Number, Number) -> Number) -> Self {
        let result = op(lhs.coerce_to_number(), rhs.coerce_to_number());
        Self::Number(result)
    }

    #[inline]
    fn checked_numeric_binary_op(
        lhs: &Self,
        rhs: &Self,
        op: fn(Number, Number) -> Option<Number>,
    ) -> Result<Self, NumericOverflowError> {
        op(lhs.coerce_to_number(), rhs.coerce_to_number())
            .map(Self::Number)
            .ok_or(NumericOverflowError)
    }

    #[inline]
    fn partial_cmp_op(vm: &Vm, lhs: &Self, rhs: &Self, f: fn(cmp::Ordering) -> bool) -> Self {
        let result = if lhs.is_str_or_ref() || rhs.is_str_or_ref() {
            f(lhs.coerce_to_string(vm).cmp(&rhs.coerce_to_string(vm)))
        } else {
            lhs.coerce_to_number()
                .partial_cmp(&rhs.coerce_to_number())
                .map_or(false, f)
        };
        Self::Boolean(result)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Number(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::Reference(value) => write!(f, "{}", value),
            Self::NativeObject(value) => write!(f, "NativeObject<{}>", value),
            Self::Null => f.write_str("null"),
            Self::Undefined => f.write_str("undefined"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Number {
    Float(f64),
    Int(i64),
}

impl Number {
    pub const NAN: Self = Self::Float(f64::NAN);
    pub const POS_INF: Self = Self::Float(f64::INFINITY);
    pub const NEG_INF: Self = Self::Float(f64::NEG_INFINITY);

    pub fn infinity(sign: i64) -> Self {
        if sign >= 0 {
            Self::POS_INF
        } else {
            Self::NEG_INF
        }
    }

    pub fn is_zero(self) -> bool {
        match self {
            Self::Float(value) => value == 0.0,
            Self::Int(value) => value == 0,
        }
    }

    pub fn is_finite(self) -> bool {
        match self {
            Self::Float(value) => value.is_finite(),
            Self::Int(_) => true,
        }
    }

    pub fn is_infinite(self) -> bool {
        match self {
            Self::Float(value) => value.is_infinite(),
            Self::Int(_) => false,
        }
    }

    pub fn is_nan(self) -> bool {
        match self {
            Self::Float(value) => value.is_nan(),
            Self::Int(_) => false,
        }
    }

    pub fn as_i64(self) -> i64 {
        match self {
            #[allow(clippy::cast_possible_truncation)]
            Self::Float(value) => value as i64,
            Self::Int(value) => value,
        }
    }

    pub fn as_f64(self) -> f64 {
        match self {
            Self::Float(value) => value,
            #[allow(clippy::cast_precision_loss)]
            Self::Int(value) => value as f64,
        }
    }

    pub fn checked_neg(self) -> Option<Self> {
        Some(match self {
            Self::Float(value) => Self::Float(-value),
            Self::Int(value) => Self::Int(value.checked_neg()?),
        })
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_add(lhs, rhs)?),
            (lhs, rhs) => Self::Float(lhs.as_f64() + rhs.as_f64()),
        })
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_sub(lhs, rhs)?),
            (lhs, rhs) => Self::Float(lhs.as_f64() - rhs.as_f64()),
        })
    }

    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_mul(lhs, rhs)?),
            (lhs, rhs) => Self::Float(lhs.as_f64() * rhs.as_f64()),
        })
    }

    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Self::Int(0), Self::Int(0)) => Self::NAN,
            (Self::Int(lhs), Self::Int(0)) => Self::infinity(lhs),
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_div(lhs, rhs)?),
            (lhs, rhs) => Self::Float(lhs.as_f64() / rhs.as_f64()),
        })
    }

    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Self::Int(_), Self::Int(0)) => Self::NAN,
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_rem(lhs, rhs)?),
            (lhs, rhs) => Self::Float(lhs.as_f64() % rhs.as_f64()),
        })
    }

    pub fn pow(self, exp: Self) -> Self {
        Self::Float(self.as_f64().powf(exp.as_f64()))
    }

    pub fn sqrt(self) -> Self {
        Self::Float(self.as_f64().sqrt())
    }

    /// # Panics
    ///
    /// Always panics.
    pub fn checked_shl(self, rhs: Self) -> Option<Self> {
        todo!("Number::checked_shl: lhs={}, rhs={}", self, rhs)
    }

    /// # Panics
    ///
    /// Always panics.
    pub fn checked_shr_signed(self, rhs: Self) -> Option<Self> {
        todo!("Number::checked_shr_signed: lhs={}, rhs={}", self, rhs)
    }

    /// # Panics
    ///
    /// Always panics.
    pub fn checked_shr_unsigned(self, rhs: Self) -> Option<Self> {
        todo!("Number::checked_shr_unsigned: lhs={}, rhs={}", self, rhs)
    }
}

impl ops::Not for Number {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is_nan() {
            Self::NAN
        } else if self.is_infinite() {
            Self::Int(-1)
        } else {
            Self::Int(!self.as_i64())
        }
    }
}

impl ops::BitAnd for Number {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            Self::NAN
        } else if self.is_infinite() || rhs.is_infinite() {
            Self::Int(0)
        } else {
            Self::Int(self.as_i64() & self.as_i64())
        }
    }
}

impl ops::BitOr for Number {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            Self::NAN
        } else if self.is_infinite() && rhs.is_infinite() {
            Self::Int(0)
        } else if self.is_infinite() {
            Self::Int(rhs.as_i64())
        } else if rhs.is_infinite() {
            Self::Int(self.as_i64())
        } else {
            Self::Int(self.as_i64() | rhs.as_i64())
        }
    }
}

impl ops::BitXor for Number {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            Self::NAN
        } else if self.is_infinite() && rhs.is_infinite() {
            Self::Int(0)
        } else if self.is_infinite() {
            Self::Int(rhs.as_i64())
        } else if rhs.is_infinite() {
            Self::Int(self.as_i64())
        } else {
            Self::Int(self.as_i64() ^ rhs.as_i64())
        }
    }
}

impl cmp::PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (lhs, rhs) => lhs.as_f64() == rhs.as_f64(),
        }
    }
}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => Some(lhs.cmp(rhs)),
            (lhs, rhs) => lhs.as_f64().partial_cmp(&rhs.as_f64()),
        }
    }
}

impl TryFrom<u64> for Number {
    type Error = num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        i64::try_from(value).map(Self::Int)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Float(value) => {
                if value.is_nan() {
                    f.write_str("NaN")
                } else if value.is_infinite() {
                    f.write_str(if value.is_sign_positive() {
                        "Infinity"
                    } else {
                        "-Infinity"
                    })
                } else {
                    write!(f, "{}", value)
                }
            }
            Self::Int(value) => write!(f, "{}", value),
        }
    }
}

impl FromStr for Number {
    type Err = num::ParseIntError;

    // TODO: Parsing numbers at runtime should use the same logic as the lexer?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            i64::from_str(s).map(Self::Int)
        } else {
            Ok(Self::Int(0))
        }
    }
}

impl From<&str> for Number {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap_or(Self::NAN)
    }
}
