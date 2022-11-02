use super::error::OutOfHeapSpaceError;
use super::object::Object;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

// TODO: Get rid of `Rc<RefCell<_>>` here.
#[derive(Default)]
pub struct Heap {
    slots: Vec<Rc<RefCell<Object>>>,
}

impl Heap {
    pub fn allocate(&mut self, obj: Object) -> Result<Reference, OutOfHeapSpaceError> {
        let slot_idx = self.slots.len();
        if slot_idx != usize::MAX {
            self.slots.push(Rc::new(RefCell::new(obj)));
            Ok(Reference(slot_idx))
        } else {
            Err(OutOfHeapSpaceError::new())
        }
    }

    pub fn resolve(&self, obj_ref: Reference) -> ObjectRef {
        let obj = &self.slots[obj_ref.0];
        ObjectRef { obj: obj.clone() }
    }

    pub fn resolve_mut(&mut self, obj_ref: Reference) -> ObjectRef {
        let obj = &self.slots[obj_ref.0];
        ObjectRef { obj: obj.clone() }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Reference(usize);

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

#[derive(Clone)]
pub struct ObjectRef {
    obj: Rc<RefCell<Object>>,
}

impl ObjectRef {
    pub fn as_ref(&self) -> Ref<Object> {
        RefCell::borrow(&self.obj)
    }

    pub fn as_ref_mut(&mut self) -> RefMut<Object> {
        RefCell::borrow_mut(&self.obj)
    }
}
