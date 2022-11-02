use super::error::InitialisationError;
use super::heap::Heap;
use super::stack::CallStack;
use super::value::Value;
use crate::interpreter::ObjectRef;
use crate::runtime::Runtime;
use std::assert_matches::assert_matches;
use std::mem;

pub struct Vm {
    execution_state: ExecutionState,
    hidden_exception: Option<Value>,
    heap: Heap,
    runtime: Runtime,
    stack: CallStack,
}

impl Vm {
    pub fn new() -> Result<Self, InitialisationError> {
        let mut heap = Heap::default();
        let runtime = Runtime::with_default_global_object(&mut heap)?;
        Ok(Self {
            execution_state: ExecutionState::default(),
            hidden_exception: Option::default(),
            heap,
            runtime,
            stack: CallStack::default(),
        })
    }

    pub fn execution_state(&self) -> &ExecutionState {
        &self.execution_state
    }

    /// # Panics
    ///
    /// Panics if the current execution state is not
    /// [`ExecutionState::Advance`].
    pub fn set_execution_state(&mut self, execution_state: ExecutionState) {
        assert_matches!(self.execution_state, ExecutionState::Advance);
        self.execution_state = execution_state;
    }

    pub fn reset_execution_state(&mut self) -> ExecutionState {
        mem::take(&mut self.execution_state)
    }

    pub fn handle_loop_execution_state(&mut self) -> IterationDecision {
        match self.execution_state() {
            ExecutionState::Advance => IterationDecision::Advance,
            ExecutionState::Break => {
                self.reset_execution_state();
                IterationDecision::Break
            }
            ExecutionState::Continue => {
                self.reset_execution_state();
                IterationDecision::Continue
            }
            ExecutionState::Return(_) | ExecutionState::Exception(_) | ExecutionState::Exit => {
                // Exit the loop, but don't reset the execution state just yet so that it can be
                // handled/cleared by some calling AST node.
                IterationDecision::Break
            }
        }
    }

    /// If the current execution state is an exception, reset it to
    /// [`ExecutionState::Advance`] and stash the exception value away so it
    /// may be [restored][Self::restore_hidden_exception()] later. If an
    /// exception has already been hidden, discard the exception value taken
    /// from the execution state.
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

    /// If an exception was previously [hidden][Self::hide_current_exception()],
    /// restore it by putting it back into the execution state. If the
    /// execution state already contains an exception, discard the hidden
    /// exception.
    pub fn restore_hidden_exception(&mut self) {
        if let Some(exception) = self.hidden_exception.take() {
            if !matches!(self.execution_state(), ExecutionState::Exception(..)) {
                self.set_execution_state(ExecutionState::Exception(exception));
            }
        }
    }

    /// Reset the execution state to [`ExecutionState::Advance`] if it contains
    /// an exception, and discard any hidden exception.
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

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    pub fn stack(&self) -> &CallStack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut CallStack {
        &mut self.stack
    }

    pub fn global_object(&self) -> ObjectRef {
        let obj_ref = self.runtime().global_object_ref();
        self.heap().resolve(obj_ref)
    }

    #[allow(clippy::unused_self)]
    pub fn write_message(&mut self, message: &str) {
        // Note: Print to stderr as stdout is swallowed when running in the REPL.
        eprintln!("{message}");
    }
}

#[derive(Clone, Debug, Default)]
pub enum ExecutionState {
    #[default]
    Advance,
    Break,
    Continue,
    Exception(Value),
    Exit,
    Return(Value),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IterationDecision {
    Advance,
    Break,
    Continue,
}
