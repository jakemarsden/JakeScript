use crate::str::NonEmptyString;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{cmp, fmt};

#[macro_export]
macro_rules! ident {
    // TODO: Turn into a proc macro and check validity at compile-time.
    ($s:literal) => {
        $crate::ast::Identifier::try_from($s)
            .unwrap_or_else(|_| panic!(r#"Invalid identifier: "{}""#, $s))
    };
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(into = "Box<str>", try_from = "Box<str>")]
pub enum Identifier {
    Custom(Box<str>),
    WellKnown(WellKnownIdentifier),
}

impl Identifier {
    fn new_from_str(s: &str) -> Self {
        WellKnownIdentifier::from_str(s)
            .map_or_else(|_| Self::Custom(Box::from(s)), Self::WellKnown)
    }

    fn new_from_boxed_str(s: Box<str>) -> Self {
        WellKnownIdentifier::from_str(&s).map_or_else(|_| Self::Custom(s), Self::WellKnown)
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Custom(v) => v,
            Self::WellKnown(v) => v.as_str(),
        }
    }

    pub fn into_boxed_str(self) -> Box<str> {
        match self {
            Self::Custom(v) => v,
            Self::WellKnown(v) => Box::from(v.as_str()),
        }
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Identifier> for Box<str> {
    fn from(v: Identifier) -> Self {
        v.into_boxed_str()
    }
}

impl From<i64> for Identifier {
    fn from(n: i64) -> Self {
        Self::new_from_boxed_str(n.to_string().into_boxed_str())
    }
}

impl From<usize> for Identifier {
    fn from(n: usize) -> Self {
        Self::new_from_boxed_str(n.to_string().into_boxed_str())
    }
}

impl From<NonEmptyString> for Identifier {
    fn from(s: NonEmptyString) -> Self {
        Self::new_from_boxed_str(s.into_inner().into_boxed_str())
    }
}

impl From<char> for Identifier {
    fn from(c: char) -> Self {
        Self::new_from_boxed_str(c.to_string().into_boxed_str())
    }
}

impl FromStr for Identifier {
    type Err = ParseIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            Ok(Self::new_from_str(s))
        } else {
            Err(ParseIdentifierError(Box::from(s)))
        }
    }
}

impl TryFrom<&str> for Identifier {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl TryFrom<Box<str>> for Identifier {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: Box<str>) -> Result<Self, Self::Error> {
        if !s.is_empty() {
            Ok(Self::new_from_boxed_str(s))
        } else {
            Err(ParseIdentifierError(s))
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum WellKnownIdentifier {
    E,
    LN2,
    LN10,
    LOG2E,
    LOG10E,
    PI,
    SQRT1_2,
    SQRT2,

    Array,
    Boolean,
    Function,
    Infinity,
    Math,
    NaN,
    Number,
    Object,
    String,

    abs,
    assert,
    assertEqual,
    assertNotReached,
    charAt,
    console,
    exit,
    floor,
    isNaN,
    length,
    log,
    max,
    min,
    sqrt,
    substring,
    trunc,
    undefined,
}

impl WellKnownIdentifier {
    const ALL: &'static [Self] = &[
        Self::E,
        Self::LN2,
        Self::LN10,
        Self::LOG2E,
        Self::LOG10E,
        Self::PI,
        Self::SQRT1_2,
        Self::SQRT2,
        Self::Array,
        Self::Boolean,
        Self::Function,
        Self::Infinity,
        Self::Math,
        Self::NaN,
        Self::Number,
        Self::Object,
        Self::String,
        Self::abs,
        Self::assert,
        Self::assertEqual,
        Self::assertNotReached,
        Self::charAt,
        Self::console,
        Self::exit,
        Self::floor,
        Self::isNaN,
        Self::length,
        Self::log,
        Self::max,
        Self::min,
        Self::sqrt,
        Self::substring,
        Self::trunc,
        Self::undefined,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::E => "E",
            Self::LN2 => "LN2",
            Self::LN10 => "LN10",
            Self::LOG2E => "LOG2E",
            Self::LOG10E => "LOG10E",
            Self::PI => "PI",
            Self::SQRT1_2 => "SQRT1_2",
            Self::SQRT2 => "SQRT2",

            Self::Array => "Array",
            Self::Boolean => "Boolean",
            Self::Function => "Function",
            Self::Infinity => "Infinity",
            Self::Math => "Math",
            Self::NaN => "NaN",
            Self::Number => "Number",
            Self::Object => "Object",
            Self::String => "String",

            Self::abs => "abs",
            Self::assert => "assert",
            Self::assertEqual => "assertEqual",
            Self::assertNotReached => "assertNotReached",
            Self::charAt => "charAt",
            Self::console => "console",
            Self::exit => "exit",
            Self::floor => "floor",
            Self::isNaN => "isNaN",
            Self::length => "length",
            Self::log => "log",
            Self::max => "max",
            Self::min => "min",
            Self::sqrt => "sqrt",
            Self::substring => "substring",
            Self::trunc => "trunc",
            Self::undefined => "undefined",
        }
    }
}

impl FromStr for WellKnownIdentifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .copied()
            .find(|v| v.as_str() == s)
            .ok_or(())
    }
}

#[derive(Debug)]
pub struct ParseIdentifierError(Box<str>);

impl fmt::Display for ParseIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Failed to parse identifier: "{}""#, self.0)
    }
}
