use serde::{de, ser};
use std::fmt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct SourceLocation {
    location: Rc<PathBuf>,
    position: SourcePosition,
}

impl SourceLocation {
    pub fn at_start_of(location: impl Into<PathBuf>) -> Self {
        Self::new(location, SourcePosition::default())
    }

    pub fn new(location: impl Into<PathBuf>, position: SourcePosition) -> Self {
        Self {
            location: Rc::new(location.into()),
            position,
        }
    }

    pub fn location(&self) -> &Path {
        self.location.as_ref()
    }

    pub fn position(&self) -> &SourcePosition {
        &self.position
    }

    pub fn at(&self, position: SourcePosition) -> Self {
        Self {
            location: Rc::clone(&self.location),
            position,
        }
    }

    pub fn advance_line(&mut self) {
        self.position = self.position.plus_lines(1);
    }

    pub fn advance_column(&mut self) {
        self.position = self.position.plus_columns(1);
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self.location().display();
        let position = self.position();
        if f.alternate() {
            write!(f, "{position:#} of {location:#}",)
        } else {
            write!(f, "<{location}@{position}>")
        }
    }
}

impl FromStr for SourceLocation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (loc_str, pos_str) = s.split_once('@').ok_or(())?;
        let pos = SourcePosition::from_str(pos_str)?;
        Ok(Self::new(loc_str, pos))
    }
}

impl<'de> de::Deserialize<'de> for SourceLocation {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = SourceLocation;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(r#"a string in the format "path@line:col""#)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                SourceLocation::from_str(v)
                    .map_err(|()| E::invalid_value(de::Unexpected::Str(v), &self))
            }
        }

        d.deserialize_str(Visitor)
    }
}

impl ser::Serialize for SourceLocation {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct SourcePosition(usize, usize);

impl SourcePosition {
    /// - `line` - Zero-based.
    /// - `column` - Zero-based.
    pub fn at(line: usize, column: usize) -> Self {
        Self(line, column)
    }

    /// Zero-based.
    pub fn line(&self) -> usize {
        self.0
    }

    /// Zero-based.
    pub fn column(&self) -> usize {
        self.1
    }

    pub fn plus_lines(&self, n: usize) -> Self {
        Self::at(
            self.line().saturating_add(n),
            if n != 0 { 0 } else { self.column() },
        )
    }

    pub fn plus_columns(&self, n: usize) -> Self {
        Self::at(self.line(), self.column().saturating_add(n))
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = self.line().saturating_add(1);
        let column = self.column().saturating_add(1);
        if f.alternate() {
            write!(f, "line {line}, column {column}",)
        } else {
            write!(f, "{line}:{column}",)
        }
    }
}

impl FromStr for SourcePosition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line_str, col_str) = s.split_once(':').ok_or(())?;
        let line = usize::from_str(line_str).map_err(|_| ())?;
        let col = usize::from_str(col_str).map_err(|_| ())?;
        Ok(Self::at(line.saturating_sub(1), col.saturating_sub(1)))
    }
}

impl<'de> de::Deserialize<'de> for SourcePosition {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = SourcePosition;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(r#"a string in the format "line:col""#)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                SourcePosition::from_str(v)
                    .map_err(|()| E::invalid_value(de::Unexpected::Str(v), &self))
            }
        }

        d.deserialize_str(Visitor)
    }
}

impl ser::Serialize for SourcePosition {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}
