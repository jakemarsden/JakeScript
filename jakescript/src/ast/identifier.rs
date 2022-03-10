use crate::str::NonEmptyString;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Identifier(NonEmptyString);

impl Identifier {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<NonEmptyString> for Identifier {
    fn from(s: NonEmptyString) -> Self {
        Self(s)
    }
}

impl From<i64> for Identifier {
    fn from(value: i64) -> Self {
        let s = value.to_string();
        // Safety: The string can't be empty because it was created from a number.
        Self(unsafe { NonEmptyString::from_unchecked(s) })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
