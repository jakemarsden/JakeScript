use std::fmt;

// TODO: Proper template support.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Template {
    pub value: Box<str>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"`{}`"#, self.value)
    }
}
