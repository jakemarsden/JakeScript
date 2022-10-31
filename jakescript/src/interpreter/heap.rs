use super::error::OutOfMemoryError;
use super::object::Object;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Heap {
    next_obj_idx: usize,
}

impl Heap {
    pub fn allocate(&mut self, obj: Object) -> Result<Reference, OutOfMemoryError> {
        let obj_idx = self.next_obj_idx;
        self.next_obj_idx = self.next_obj_idx.checked_add(1).ok_or(OutOfMemoryError)?;
        Ok(Reference::new(obj_idx, obj))
    }

    // unused_self: Will be used in the future, see comment about storing objects
    // inside the heap.
    #[allow(clippy::unused_self)]
    pub fn resolve<'a>(&self, refr: &'a Reference) -> Ref<'a, Object> {
        refr.deref()
    }

    // unused_self: Will be used in the future, see comment about storing objects
    // inside the heap.
    #[allow(clippy::unused_self)]
    pub fn resolve_mut<'a>(&mut self, refr: &'a Reference) -> RefMut<'a, Object> {
        refr.deref_mut()
    }
}

// TODO: Store objects inside the actual `Heap` struct, rather than ref-counting
// them in the  `Reference` type because currently, the heap stores nothing.
// This was done for simplicity, and  to avoid needing to worry about garbage
// collection. Also simplify the `Reference` type to  `#[derive(Copy, Clone)]
// pub struct Reference(usize)` when possible.
#[derive(Clone)]
pub struct Reference(usize, Rc<RefCell<Object>>);

impl Reference {
    fn new(idx: usize, obj: Object) -> Self {
        Self(idx, Rc::new(RefCell::new(obj)))
    }

    fn deref(&self) -> Ref<Object> {
        RefCell::borrow(&self.1)
    }

    fn deref_mut(&self) -> RefMut<Object> {
        RefCell::borrow_mut(&self.1)
    }
}

impl Eq for Reference {}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Note: 6 includes the 2 chars for the "0x" prefix, so only 4 actual digits are
        // displayed.
        write!(f, "{:#06x}", self.0)
    }
}

impl fmt::Debug for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
