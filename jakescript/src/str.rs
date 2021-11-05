use std::fmt;
use std::str::FromStr;

// TODO: Turn this into a proc macro which fails at compile-time when passed empty string literals,
//  avoiding the runtime check and possible panic.
/// Create a `NonEmptyString` from a `"string literal"`.
///
/// # Panics
///
/// Panics _at runtime_ when used with an empty string literal.
///
/// ```should_panic
/// # use jakescript::non_empty_str;
/// # use jakescript::str::NonEmptyString;
/// non_empty_str!("");
/// ```
#[macro_export]
macro_rules! non_empty_str {
    ($s:literal) => {
        NonEmptyString::try_from($s).unwrap()
    };
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// # Safety
    ///
    /// The provided string must not be empty.
    pub unsafe fn from_unchecked(s: String) -> Self {
        Self(s)
    }

    /// # Safety
    ///
    /// The provided str must not be empty.
    pub unsafe fn from_str_unchecked(s: &str) -> Self {
        Self(String::from(s))
    }

    // len_without_is_empty: There's no point as it would always return `false`.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, ch: char) {
        self.0.push(ch);
    }

    pub fn push_str(&mut self, s: &str) {
        self.0.push_str(s);
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

impl From<char> for NonEmptyString {
    fn from(ch: char) -> Self {
        Self(String::from(ch))
    }
}

impl FromStr for NonEmptyString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if !s.is_empty() {
            // Safety: The str can't be empty because of the surrounding if.
            Ok(unsafe { Self::from_str_unchecked(s) })
        } else {
            Err(())
        }
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if !s.is_empty() {
            // Safety: The string can't be empty because of the surrounding if.
            Ok(unsafe { Self::from_unchecked(s) })
        } else {
            Err(())
        }
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

impl From<NonEmptyString> for String {
    fn from(s: NonEmptyString) -> Self {
        s.into_inner()
    }
}
