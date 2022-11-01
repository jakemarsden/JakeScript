use crate::simple_enumeration;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{cmp, fmt};

#[macro_export]
macro_rules! ident {
    // TODO: Turn into a proc macro to do the work at compile-time.
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
        Self::from(n.to_string())
    }
}

impl From<usize> for Identifier {
    fn from(n: usize) -> Self {
        Self::from(n.to_string())
    }
}

impl From<char> for Identifier {
    fn from(c: char) -> Self {
        Self::from(c.to_string())
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        WellKnownIdentifier::from_str(s)
            .map_or_else(|()| Self::Custom(Box::from(s)), Self::WellKnown)
    }
}

impl From<Box<str>> for Identifier {
    fn from(s: Box<str>) -> Self {
        WellKnownIdentifier::from_str(&s).map_or_else(|()| Self::Custom(s), Self::WellKnown)
    }
}

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Self::from(s.into_boxed_str())
    }
}

simple_enumeration!(
    #[allow(non_camel_case_types)]
    pub WellKnownIdentifier {
        E => "E",
        LN2 => "LN2",
        LN10 => "LN10",
        LOG2E => "LOG2E",
        LOG10E => "LOG10E",
        PI => "PI",
        SQRT1_2 => "SQRT1_2",
        SQRT2 => "SQRT2",

        Array => "Array",
        Boolean => "Boolean",
        Function => "Function",
        Infinity => "Infinity",
        Math => "Math",
        NaN => "NaN",
        Number => "Number",
        Object => "Object",
        String => "String",

        abs => "abs",
        assert => "assert",
        assertEqual => "assertEqual",
        assertNotReached => "assertNotReached",
        charAt => "charAt",
        console => "console",
        exit => "exit",
        floor => "floor",
        isNaN => "isNaN",
        length => "length",
        log => "log",
        max => "max",
        min => "min",
        sqrt => "sqrt",
        substring => "substring",
        trunc => "trunc",
        undefined => "undefined",
    }
);

#[derive(Debug)]
pub struct ParseIdentifierError(Box<str>);

impl fmt::Display for ParseIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Failed to parse identifier: "{}""#, self.0)
    }
}
