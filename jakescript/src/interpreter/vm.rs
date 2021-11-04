use crate::ast::ConstantPool;
use crate::interpreter::heap::Heap;
use crate::interpreter::stack::CallStack;
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

    /// # Panics
    ///
    /// Panics if the current execution state is not [`ExecutionState::Advance`].
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

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    pub fn set_constant_pool(&mut self, constant_pool: ConstantPool) {
        self.constant_pool = constant_pool;
    }

    pub fn heap(&self) -> &Heap {
        &self.heap
    }
    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }

    pub fn stack(&self) -> &CallStack {
        &self.stack
    }
    pub fn stack_mut(&mut self) -> &mut CallStack {
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
