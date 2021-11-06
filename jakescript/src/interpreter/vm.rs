use crate::interpreter::heap::Heap;
use crate::interpreter::stack::CallStack;
use crate::interpreter::value::Value;
use std::assert_matches::assert_matches;
use std::mem;

#[derive(Default)]
pub struct Vm {
    execution_state: ExecutionState,
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
        assert_matches!(self.execution_state, ExecutionState::Advance);
        self.execution_state = execution_state;
    }

    pub fn reset_execution_state(&mut self) -> ExecutionState {
        mem::take(&mut self.execution_state)
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
    Exit,
}
