use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// # Safety
    ///
    /// The provided string must not be empty, i.e. it must contain at least one character.

    pub unsafe fn from_unchecked(str: String) -> Self {
        Self(str)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if !s.is_empty() {
            // Safety: Have checked that the string is not empty.
            Ok(unsafe { Self::from_unchecked(s) })
        } else {
            Err(())
        }
    }
}

impl From<NonEmptyString> for String {
    fn from(it: NonEmptyString) -> Self {
        it.into_inner()
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for NonEmptyString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
