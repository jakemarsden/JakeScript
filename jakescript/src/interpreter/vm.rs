use crate::interpreter::heap::Heap;
use crate::interpreter::stack::CallStack;
use crate::interpreter::value::Value;
use std::assert_matches::assert_matches;
use std::mem;

#[derive(Default)]
pub struct Vm {
    execution_state: ExecutionState,
    hidden_exception: Option<Value>,
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

    /// If the current execution state is an exception, reset it to [`ExecutionState::Advance`] and
    /// stash the exception value away so it may be [restored][Self::restore_hidden_exception()]
    /// later. If an exception has already been hidden, discard the exception value taken from the
    /// execution state.
    ///
    /// This is useful for allowing `finally` blocks to function properly.
    ///
    /// # JavaScript examples
    ///
    /// ```javascript
    /// try {
    ///   // 1. The exception `1` is set in the execution state, and any further
    ///   // statements in the block are skipped.
    ///   throw 1;
    ///
    /// } finally {
    ///   // 2. The exception `1` in the execution state is hidden so that any
    ///   // statements in the block are _not_ skipped.
    ///
    ///   // 3. At the end of the block, the exception `1` is restored back to
    ///   // the execution state. The block ends with the VM in an exception
    ///   // condition (exception `1`).
    /// }
    /// ```
    ///
    /// ```javascript
    /// try {
    ///   // 1. The exception `1` is set in the execution state, and any further
    ///   // statements in the block are skipped.
    ///   throw 1;
    ///
    /// } finally {
    ///   // 2. The exception `1` in the execution state is hidden so that any
    ///   // statements in the block are _not_ skipped.
    ///
    ///   // 3. The exception `2` is set in the execution state, and any further
    ///   // statements in the block are skipped.
    ///   throw 2;
    ///
    ///   // 4. At the end of the block, the exception `1`, which is hidden, is
    ///   // discarded because the execution state already contains an exception.
    ///   // The block ends with the VM in an exception condition (exception
    ///   // `2`).
    /// }
    /// ```
    pub fn hide_current_exception(&mut self) {
        if let ExecutionState::Exception(..) = self.execution_state() {
            if let ExecutionState::Exception(ex) = self.reset_execution_state() {
                if self.hidden_exception.is_none() {
                    self.hidden_exception = Some(ex);
                }
            } else {
                unreachable!();
            }
        }
    }

    /// If an exception was previously [hidden][Self::hide_current_exception()], restore it by
    /// putting it back into the execution state. If the execution state already contains an
    /// exception, discard the hidden exception.
    pub fn restore_hidden_exception(&mut self) {
        if let Some(exception) = self.hidden_exception.take() {
            if !matches!(self.execution_state(), ExecutionState::Exception(..)) {
                self.set_execution_state(ExecutionState::Exception(exception));
            }
        }
    }

    /// Reset the execution state to [`ExecutionState::Advance`] if it contains an exception, and
    /// discard any hidden exception.
    pub fn clear_exception(&mut self) -> Option<Value> {
        self.hidden_exception.take();
        if matches!(self.execution_state(), ExecutionState::Exception(..)) {
            if let ExecutionState::Exception(ex) = self.reset_execution_state() {
                Some(ex)
            } else {
                unreachable!()
            }
        } else {
            None
        }
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
    Exception(Value),
    Exit,
}
