use crate::interpreter::error::NumericOverflowError;
use crate::interpreter::heap::Reference;
use crate::interpreter::vm::Vm;
use std::ops::{self, BitAnd, BitOr, BitXor};
use std::str::FromStr;
use std::{cmp, fmt, num};

#[derive(Clone, Default, Debug)]
pub enum Value {
    Boolean(bool),
    Number(Number),
    String(String),
    Reference(Reference),
    NativeFunction(NativeFunction),
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
        Self::checked_numeric_binary_op(lhs, rhs, Number::checked_pow)
    }

    pub fn identical(_vm: &Vm, lhs: &Self, rhs: &Self) -> Self {
        let result = match (lhs, rhs) {
            (Self::Boolean(lhs), Self::Boolean(rhs)) => lhs == rhs,
            (Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::Reference(lhs), Self::Reference(rhs)) => lhs == rhs,
            (Self::NativeFunction(lhs), Self::NativeFunction(rhs)) => lhs == rhs,
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
            Self::NativeFunction(lhs) => {
                if let Self::NativeFunction(rhs) = rhs {
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
            Self::Number(value) => match value {
                Number::Int(value) => *value != 0,
                Number::Inf(_) => true,
                Number::NaN => false,
            },
            Self::String(value) => !value.is_empty(),
            Self::Reference(..) | Self::NativeFunction(_) => true,
            Self::Null | Self::Undefined => false,
        }
    }

    pub fn coerce_to_number(&self) -> Number {
        match self {
            Self::Boolean(value) => Number::Int(if *value { 1 } else { 0 }),
            Self::Number(value) => *value,
            Self::String(value) => Number::from_str(value).unwrap_or(Number::NaN),
            Self::Null => Number::Int(0),
            Self::Reference(..) | Self::NativeFunction(_) | Self::Undefined => Number::NaN,
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
            Self::NativeFunction(value) => value.to_string(),
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
            Self::NativeFunction(value) => write!(f, "{}", value),
            Self::Null => f.write_str("null"),
            Self::Undefined => f.write_str("undefined"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Number {
    Int(i64),
    Inf(Sign),
    NaN,
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub enum Sign {
    #[default]
    Pos,
    Neg,
}

impl Sign {
    pub fn of(n: i64) -> Self {
        if n >= 0 {
            Self::Pos
        } else {
            Self::Neg
        }
    }
}

impl ops::Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Pos => Self::Neg,
            Self::Neg => Self::Pos,
        }
    }
}

impl ops::Mul for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Pos, Self::Pos) | (Self::Neg, Self::Neg) => Self::Pos,
            (Self::Pos, Self::Neg) | (Self::Neg, Self::Pos) => Self::Neg,
        }
    }
}

impl cmp::Ord for Sign {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Self::Pos, Self::Pos) | (Self::Neg, Self::Neg) => cmp::Ordering::Equal,
            (Self::Pos, Self::Neg) => cmp::Ordering::Greater,
            (Self::Neg, Self::Pos) => cmp::Ordering::Less,
        }
    }
}

impl PartialOrd<Self> for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Number {
    pub fn checked_neg(self) -> Option<Self> {
        Some(match self {
            Self::Int(value) => Self::Int(value.checked_neg()?),
            Self::Inf(sign) => Self::Inf(-sign),
            Self::NaN => Self::NaN,
        })
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_add(lhs, rhs)?),
            (Self::Int(_), Self::Inf(sign)) | (Self::Inf(sign), Self::Int(_)) => Self::Inf(sign),
            (Self::Inf(lhs_sign), Self::Inf(rhs_sign)) => {
                if lhs_sign == rhs_sign {
                    Self::Inf(lhs_sign)
                } else {
                    Self::NaN
                }
            }
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
        })
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_sub(lhs, rhs)?),
            (Self::Int(_), Self::Inf(sign)) => Self::Inf(-sign),
            (Self::Inf(sign), Self::Int(_)) => Self::Inf(sign),
            (Self::Inf(lhs_sign), Self::Inf(rhs_sign)) => {
                if lhs_sign != rhs_sign {
                    Self::Inf(lhs_sign)
                } else {
                    Self::NaN
                }
            }
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
        })
    }

    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        #[allow(clippy::match_same_arms)]
        Some(match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_mul(lhs, rhs)?),
            (Self::Int(0), Self::Inf(_)) | (Self::Inf(_), Self::Int(0)) => Self::NaN,
            (Self::Int(value), Self::Inf(sign)) | (Self::Inf(sign), Self::Int(value)) => {
                Self::Inf(Sign::of(value) * sign)
            }
            (Self::Inf(lhs_sign), Self::Inf(rhs_sign)) => Self::Inf(lhs_sign * rhs_sign),
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
        })
    }

    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        #[allow(clippy::match_same_arms)]
        Some(match (self, rhs) {
            (Self::Int(0), Self::Int(0)) | (Self::Inf(_), Self::Inf(_)) => Self::NaN,
            (Self::Int(lhs), Self::Int(0)) => Self::Inf(Sign::of(lhs)),
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_div(lhs, rhs)?),
            (Self::Int(_), Self::Inf(_)) => Self::Int(0),
            (Self::Inf(sign), Self::Int(0)) => Self::Inf(sign),
            (Self::Inf(sign), Self::Int(value)) => Self::Inf(Sign::of(value) * sign),
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
        })
    }

    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        #[allow(clippy::match_same_arms)]
        Some(match (self, rhs) {
            (Self::Int(_), Self::Int(0)) => Self::NaN,
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::checked_rem(lhs, rhs)?),
            (Self::Int(lhs), Self::Inf(_)) => Self::Int(lhs),
            (Self::Inf(_), Self::Int(_) | Self::Inf(_)) => Self::NaN,
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
        })
    }

    pub fn checked_pow(self, exp: Self) -> Option<Self> {
        #[allow(clippy::match_same_arms)]
        Some(match (exp, self) {
            (Self::Int(exp), Self::Int(0)) if exp.is_negative() => Self::Inf(Sign::Pos),
            (Self::Int(exp), Self::Int(base)) => Self::Int(checked_pow(base, exp)?),
            (Self::Int(0), Self::Inf(_)) => Self::Int(1),
            (Self::Int(exp), Self::Inf(_)) if exp.is_negative() => Self::Int(0),
            (Self::Int(_), Self::Inf(Sign::Pos)) => Self::Inf(Sign::Pos),
            (Self::Int(exp), Self::Inf(Sign::Neg)) => {
                Self::Inf(if exp % 2 == 0 { Sign::Pos } else { Sign::Neg })
            }
            (Self::Inf(Sign::Pos), Self::Int(0)) => Self::Int(0),
            (Self::Inf(Sign::Pos), Self::Int(_) | Self::Inf(_)) => Self::Inf(Sign::Pos),
            (Self::Inf(Sign::Neg), Self::Int(0)) => Self::Inf(Sign::Pos),
            (Self::Inf(Sign::Neg), Self::Int(_) | Self::Inf(_)) => Self::Int(0),
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
        })
    }

    // cast_precision_loss, cast_possible_truncation: TODO: Handle floating-point properly
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn checked_sqrt(self) -> Option<Self> {
        #[allow(clippy::match_same_arms)]
        Some(match self {
            Self::Int(n) => match n.cmp(&0) {
                cmp::Ordering::Greater => Self::Int((n as f64).sqrt().round() as i64),
                cmp::Ordering::Less => Self::NaN,
                cmp::Ordering::Equal => return None,
            },
            Self::Inf(Sign::Pos) => Self::Inf(Sign::Pos),
            Self::Inf(Sign::Neg) => Self::NaN,
            Self::NaN => Self::NaN,
        })
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
        match self {
            Self::Int(value) => Self::Int(value.not()),
            Self::Inf(_) => Self::Int(-1),
            Self::NaN => Self::NaN,
        }
    }
}

impl ops::BitAnd for Number {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::bitand(lhs, rhs)),
            (Self::Inf(_), _) | (_, Self::Inf(_)) => Self::Int(0),
        }
    }
}

impl ops::BitOr for Number {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::bitand(lhs, rhs)),
            (Self::Int(lhs), Self::Inf(_)) => Self::Int(lhs),
            (Self::Inf(_), Self::Int(rhs)) => Self::Int(rhs),
            (Self::Inf(_), Self::Inf(_)) => Self::Int(0),
        }
    }
}

impl ops::BitXor for Number {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::NaN, _) | (_, Self::NaN) => Self::NaN,
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(i64::bitxor(lhs, rhs)),
            (Self::Int(lhs), Self::Inf(_)) => Self::Int(lhs),
            (Self::Inf(_), Self::Int(rhs)) => Self::Int(rhs),
            (Self::Inf(_), Self::Inf(_)) => Self::Int(0),
        }
    }
}

impl cmp::PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::Inf(lhs_sign), Self::Inf(rhs_sign)) => lhs_sign == rhs_sign,
            (Self::Inf(_) | Self::NaN, _) | (_, Self::Inf(_) | Self::NaN) => false,
        }
    }
}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => i64::cmp(lhs, rhs),
            (Self::Int(_), Self::Inf(Sign::Neg)) | (Self::Inf(Sign::Pos), Self::Int(_)) => {
                cmp::Ordering::Greater
            }
            (Self::Int(_), Self::Inf(Sign::Pos)) | (Self::Inf(Sign::Neg), Self::Int(_)) => {
                cmp::Ordering::Less
            }
            (Self::Inf(lhs_sign), Self::Inf(rhs_sign)) => lhs_sign.cmp(rhs_sign),
            (Self::NaN, _) | (_, Self::NaN) => return None,
        })
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
            Self::Int(value) => write!(f, "{}", value),
            Self::Inf(Sign::Pos) => f.write_str("Infinity"),
            Self::Inf(Sign::Neg) => f.write_str("-Infinity"),
            Self::NaN => f.write_str("NaN"),
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

#[derive(Copy, Clone)]
pub struct NativeFunction {
    name: &'static str,
    implementation: &'static dyn Fn(&mut Vm, &[Value]) -> Value,
}

impl NativeFunction {
    pub fn new(
        name: &'static str,
        implementation: &'static dyn Fn(&mut Vm, &[Value]) -> Value,
    ) -> Self {
        Self {
            name,
            implementation,
        }
    }

    pub fn apply(&self, vm: &mut Vm, args: &[Value]) -> Value {
        (self.implementation)(vm, args)
    }
}

impl cmp::PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}() {{\n    [native code]\n}}", self.name)
    }
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NativeFunction")
            .field("name", &self.name)
            .field("implementation", &"[native code]")
            .finish()
    }
}
