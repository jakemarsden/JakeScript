pub use error::*;
pub use heap::*;
pub use stack::*;
pub use value::*;
pub use vm::*;

use crate::ast::Node;

mod block;
mod declaration;
mod error;
mod expression;
mod heap;
mod stack;
mod statement;
mod value;
mod vm;

pub trait Eval: Node {
    type Output = ();

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output>;
}

pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn new(vm: Vm) -> Self {
        Self { vm }
    }

    pub fn vm(&self) -> &Vm {
        &self.vm
    }
    pub fn vm_mut(&mut self) -> &mut Vm {
        &mut self.vm
    }
}
