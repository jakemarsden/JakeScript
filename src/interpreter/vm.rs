use crate::interpreter::stack::*;

#[derive(Default)]
pub struct Vm {
    stack: CallStack,
}

impl Vm {
    pub fn stack(&mut self) -> &mut CallStack {
        &mut self.stack
    }
}
