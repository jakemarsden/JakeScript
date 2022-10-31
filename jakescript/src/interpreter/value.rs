use super::heap::Reference;
use std::str::FromStr;
use std::{cmp, fmt, num, ops};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(Number),
    Object(Reference),
    Null,
    #[default]
    Undefined,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::Object(value) => write!(f, "{value}"),
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
    pub const NEG_INF: Self = Self::Float(f64::NEG_INFINITY);
    pub const POS_INF: Self = Self::Float(f64::INFINITY);

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

    pub fn is_negative(self) -> bool {
        match self {
            Self::Float(value) => value < 0.0,
            Self::Int(value) => value < 0,
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

    pub fn checked_abs(self) -> Option<Self> {
        Some(match self {
            Self::Float(value) => Self::Float(value.abs()),
            Self::Int(value) => Self::Int(value.checked_abs()?),
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
                    write!(f, "{value}")
                }
            }
            Self::Int(value) => write!(f, "{value}"),
        }
    }
}

impl FromStr for Number {
    type Err = ();

    // TODO: Parsing numbers at runtime should use the same logic as the lexer?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            i64::from_str(s)
                .map(Self::Int)
                .map_err(|_| ())
                .or_else(|_| f64::from_str(s).map(Self::Float).map_err(|_| ()))
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

impl From<i64> for Number {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}

impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl TryFrom<usize> for Number {
    type Error = num::TryFromIntError;

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        i64::try_from(v).map(Self::from)
    }
}
