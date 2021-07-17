#[derive(Default)]
pub struct Vm {}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Null,
    Numeric(u64),
    String(String),
    Undefined,
}

impl Default for Value {
    fn default() -> Self {
        Self::Undefined
    }
}
