use crate::ast::Value;
use crate::interpreter::stack::*;
use std::mem;

#[derive(Default)]
pub struct Vm {
    execution_state: ExecutionState,
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

    pub fn stack(&mut self) -> &mut CallStack {
        &mut self.stack
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
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
