use crate::ast::{Block, ConstantId, IdentifierName, IdentifierNameRef};
use crate::interpreter::error::OutOfMemoryError;
use crate::interpreter::stack::Scope;
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
        self.allocate_object(|| Object::new(HashMap::default(), None))
    }

    pub fn allocate_callable_object(
        &mut self,
        callable: Callable,
    ) -> Result<Reference, OutOfMemoryError> {
        self.allocate_object(|| Object::new(HashMap::default(), Some(callable)))
    }

    fn allocate_object(
        &mut self,
        constructor: impl FnOnce() -> Object,
    ) -> Result<Reference, OutOfMemoryError> {
        let obj_idx = self.next_obj_idx;
        self.next_obj_idx = self.next_obj_idx.checked_add(1).ok_or(OutOfMemoryError)?;
        let new_obj = constructor();
        Ok(Reference::new(obj_idx, new_obj))
    }

    pub fn resolve<'a>(&self, refr: &'a Reference) -> Ref<'a, Object> {
        refr.deref()
    }

    pub fn resolve_mut<'a>(&mut self, refr: &'a Reference) -> RefMut<'a, Object> {
        refr.deref_mut()
    }
}

// TODO: Store objects inside the actual `Heap` struct, rather than ref-counting them in the
//  `Reference` type because currently, the heap stores nothing. This was done for simplicity, and
//  to avoid needing to worry about garbage collection. Also simplify the `Reference` type to
//  `#[derive(Copy, Clone)] pub struct Reference(usize)` when possible.
#[derive(Clone)]
pub struct Reference(usize, Rc<RefCell<Object>>);

impl Eq for Reference {}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

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
        // Note: 6 includes the 2 chars for the "0x" prefix, so only 4 actual digits are displayed.
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
    callable: Option<Callable>,
}

impl Object {
    fn new(properties: HashMap<String, Value>, callable: Option<Callable>) -> Self {
        Self {
            properties,
            callable,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    pub fn property(&self, property_name: &IdentifierNameRef) -> Option<&Value> {
        self.properties.get(property_name)
    }

    pub fn set_property(&mut self, property_name: IdentifierName, value: Value) {
        self.properties.insert(property_name, value);
    }

    pub fn callable(&self) -> Option<&Callable> {
        self.callable.as_ref()
    }

    pub fn js_equals(&self, other: &Object) -> bool {
        todo!("Object::js_equals: {:?} == {:?}", self, other)
    }

    pub fn js_to_string(&self) -> String {
        todo!("Object::js_to_string: {:?}", self)
    }
}

#[derive(Debug)]
pub struct Callable {
    declared_parameters: Vec<ConstantId>,
    declared_scope: Scope,
    body: Block,
}

impl Callable {
    pub fn new(declared_parameters: Vec<ConstantId>, declared_scope: Scope, body: Block) -> Self {
        Self {
            declared_parameters,
            declared_scope,
            body,
        }
    }

    pub fn declared_parameters(&self) -> &[ConstantId] {
        &self.declared_parameters
    }

    pub fn declared_scope(&self) -> &Scope {
        &self.declared_scope
    }

    pub fn body(&self) -> &Block {
        &self.body
    }
}
