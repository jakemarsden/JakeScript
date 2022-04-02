use std::fmt;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct SourceLocation {
    location: Rc<PathBuf>,
    position: SourcePosition,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct SourcePosition(usize, usize);

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

    pub fn at(&self, position: SourcePosition) -> Self {
        Self {
            location: Rc::clone(&self.location),
            position,
        }
    }

    pub fn location(&self) -> &Path {
        self.location.as_ref()
    }

    pub fn position(&self) -> &SourcePosition {
        &self.position
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.location().display(), self.position())
    }
}

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
        write!(
            f,
            "{}:{}",
            self.line().saturating_add(1),
            self.column().saturating_add(1)
        )
    }
}
