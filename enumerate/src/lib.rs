//! # enumerate
//!
//! Traits and macros for working with `enum`s.

pub use enumerate_derive::*;

use std::fmt;

/// ## Usage
///
/// ```rust
/// use enumerate::Enumerate;
///
/// #[derive(Debug, PartialEq, Enumerate)]
/// enum MyEnum {
///     Item1,
///     Item2,
///     Item3,
/// }
///
/// assert_eq!(
///     MyEnum::enumerate(),
///     &[MyEnum::Item1, MyEnum::Item2, MyEnum::Item3]
/// );
/// ```
pub trait Enumerate {
    fn enumerate() -> &'static [Self]
    where
        Self: Sized;
}

/// ## Usage
///
/// ```rust
/// use enumerate::{EnumerateStr, NoSuchVariantError};
/// use std::str::FromStr;
///
/// #[derive(Clone, Debug, PartialEq, EnumerateStr)]
/// #[enumerate_str(rename_all = "UPPERCASE")]
/// enum MyEnum {
///     Item1,
///     #[enumerate_str(rename = "item2")]
///     Item2,
///     Item3,
/// }
///
/// assert_eq!(MyEnum::Item1.as_str(), "ITEM1");
/// assert_eq!(MyEnum::Item2.as_str(), "item2");
/// assert_eq!(MyEnum::Item3.as_str(), "ITEM3");
///
/// assert_eq!(format!("{}", MyEnum::Item1), "ITEM1");
/// assert_eq!(format!("{}", MyEnum::Item2), "item2");
/// assert_eq!(format!("{}", MyEnum::Item3), "ITEM3");
///
/// assert_eq!(MyEnum::from_str("ITEM1"), Ok(MyEnum::Item1));
/// assert_eq!(MyEnum::from_str("item2"), Ok(MyEnum::Item2));
/// assert_eq!(MyEnum::from_str("ITEM3"), Ok(MyEnum::Item3));
/// assert_eq!(MyEnum::from_str("invalid"), Err(NoSuchVariantError));
/// ```
pub trait EnumerateStr: Clone {
    fn as_str(&self) -> &'static str;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NoSuchVariantError;

impl fmt::Display for NoSuchVariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("No such enum variant")
    }
}

impl std::error::Error for NoSuchVariantError {}
