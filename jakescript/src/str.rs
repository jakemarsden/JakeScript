use serde::{de, ser};
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
/// use jakescript::non_empty_str;
/// non_empty_str!("");
/// ```
#[macro_export]
macro_rules! non_empty_str {
    ($s:literal) => {
        $crate::str::NonEmptyString::try_from($s).unwrap()
    };
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

    pub fn as_str(&self) -> &str {
        &self.0
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

impl From<usize> for NonEmptyString {
    fn from(n: usize) -> Self {
        let s = n.to_string();
        // Safety: The string can't be empty because it was created from a number.
        unsafe { Self::from_unchecked(s) }
    }
}

impl FromStr for NonEmptyString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            // Safety: The str can't be empty because of the surrounding if.
            Ok(unsafe { Self::from_str_unchecked(s) })
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
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
        self.as_str()
    }
}

impl ser::Serialize for NonEmptyString {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> de::Deserialize<'de> for NonEmptyString {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = NonEmptyString;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a non-empty string")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                NonEmptyString::try_from(value)
                    .map_err(|()| E::invalid_value(de::Unexpected::Str(value), &self))
            }

            fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
                NonEmptyString::try_from(value)
                    .map_err(|()| E::invalid_value(de::Unexpected::Str(""), &self))
            }
        }
        d.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod test {
    use super::NonEmptyString;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens, Token};

    #[test]
    fn ser_de_happy_path() {
        assert_tokens(
            &non_empty_str!("Hello, world!"),
            &[Token::Str("Hello, world!")],
        );
        assert_tokens(
            &non_empty_str!("Hello, world!"),
            &[Token::String("Hello, world!")],
        );
    }

    #[test]
    fn de_from_char() {
        assert_de_tokens(&non_empty_str!('Q'), &[Token::Char('Q')]);
    }

    #[test]
    fn de_empty_is_error() {
        assert_de_tokens_error::<NonEmptyString>(
            &[Token::Str("")],
            r#"invalid value: string "", expected a non-empty string"#,
        );
        assert_de_tokens_error::<NonEmptyString>(
            &[Token::String("")],
            r#"invalid value: string "", expected a non-empty string"#,
        );
    }

    #[test]
    fn de_invalid_type_is_error() {
        let params = [
            (Token::Bool(true), "boolean `true`"),
            (Token::Bool(false), "boolean `false`"),
            (Token::I8(42), "integer `42`"),
            (Token::I16(42), "integer `42`"),
            (Token::I32(42), "integer `42`"),
            (Token::I64(42), "integer `42`"),
            (Token::U8(42), "integer `42`"),
            (Token::U16(42), "integer `42`"),
            (Token::U32(42), "integer `42`"),
            (Token::U64(42), "integer `42`"),
            (Token::F32(0.25), "floating point `0.25`"),
            (Token::F64(0.25), "floating point `0.25`"),
        ];
        for (token, err_str) in params {
            assert_de_tokens_error::<NonEmptyString>(
                &[token],
                &format!(r#"invalid type: {}, expected a non-empty string"#, err_str),
            );
        }
    }
}
