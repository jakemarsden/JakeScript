use crate::ast::{ConstantId, ConstantValue, ConstantValueRef};
use crate::interpreter::heap::*;
use crate::interpreter::stack::*;
use crate::interpreter::value::Value;
use std::mem;

#[derive(Default)]
pub struct Vm {
    execution_state: ExecutionState,
    constant_pool: ConstantPool,
    heap: Heap,
    stack: CallStack,
}

impl Vm {
    pub fn execution_state(&self) -> &ExecutionState {
        &self.execution_state
    }

    pub fn set_execution_state(&mut self, execution_state: ExecutionState) {
        assert!(
            self.execution_state.is_advance(),
            "Unexpected execution state (expected {:?} but was {:?}): Cannot set to {:?}",
            ExecutionState::Advance,
            self.execution_state,
            execution_state
        );
        self.execution_state = execution_state;
    }

    pub fn reset_execution_state(&mut self) -> ExecutionState {
        mem::take(&mut self.execution_state)
    }

    pub fn constant_pool(&mut self) -> &mut ConstantPool {
        &mut self.constant_pool
    }

    pub fn heap(&mut self) -> &mut Heap {
        &mut self.heap
    }

    pub fn stack(&mut self) -> &mut CallStack {
        &mut self.stack
    }
}

#[derive(Clone, Default, Debug)]
pub enum ExecutionState {
    #[default]
    Advance,
    Break,
    BreakContinue,
    Return(Value),
}

impl ExecutionState {
    pub fn is_advance(&self) -> bool {
        matches!(self, Self::Advance)
    }

    pub fn is_break_or_return(&self) -> bool {
        matches!(self, Self::Break | Self::BreakContinue | Self::Return(_))
    }
}

#[derive(Default)]
pub struct ConstantPool {
    constants: Vec<ConstantValue>,
}

impl ConstantPool {
    pub fn allocate(&mut self, value: ConstantValue) -> ConstantId {
        if !self.constants.contains(&value) {
            let idx = self.constants.len();
            self.constants.push(value);
            ConstantId::new(idx)
        } else {
            panic!(r#"Already present in the constant pool: "{}""#, value)
        }
    }

    pub fn lookup(&self, id: ConstantId) -> &ConstantValueRef {
        match self.constants.get(id.idx()) {
            Some(value) => value,
            None => panic!("Invalid constant ID: {}", id),
        }
    }
}
