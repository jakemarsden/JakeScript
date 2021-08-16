use crate::interpreter::error::OutOfMemoryError;
use crate::interpreter::value::Value;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Heap {
    next_obj_idx: usize,
}

impl Heap {
    pub fn allocate_empty_object(&mut self) -> Result<Reference, OutOfMemoryError> {
        let obj_idx = self.next_obj_idx;
        self.next_obj_idx = self.next_obj_idx.checked_add(1).ok_or(OutOfMemoryError)?;
        let new_obj = Object::new();
        Ok(Reference::new(obj_idx, new_obj))
    }

    pub fn resolve<'a>(&self, refr: &'a Reference) -> Ref<'a, Object> {
        refr.deref()
    }

    pub fn resolve_mut<'a>(&mut self, refr: &'a Reference) -> RefMut<'a, Object> {
        refr.deref_mut()
    }
}

// TODO: At the moment, JS references are implemented as reference-couted Rust references to the
//  actual object; the heap stores nothing. Eventually, the _heap_ should hold the objects and this
//  type should be simplified `#[derive(Copy, Clone)] pub struct Reference(usize)`. However this is
//  simpler overall for now; garbage collection can be tomorrow's problem.
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

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Note: 6 includes the 2 chars for the "0x" prefix, so only 4 actual digits are displayed
        write!(f, "{:#06x}", self.0)
    }
}

impl fmt::Debug for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct Object {
    properties: HashMap<String, Value>,
}

impl Object {
    fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    pub fn js_equals(&self, other: &Object) -> bool {
        todo!("Object::js_equals: {:?} == {:?}", self, other)
    }

    pub fn js_to_string(&self) -> String {
        todo!("Object::js_to_string: {:?}", self)
    }
}
