use crate::str::NonEmptyString;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Identifier(NonEmptyString);

impl Identifier {
    pub fn into_inner(self) -> NonEmptyString {
        self.0
    }

    pub fn inner(&self) -> &NonEmptyString {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        self.inner().as_ref()
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
        write!(f, "{}", self.inner())
    }
}
