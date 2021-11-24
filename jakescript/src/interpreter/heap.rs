use crate::ast::{Block, Identifier};
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
    pub fn allocate_array(&mut self, values: Vec<Value>) -> Result<Reference, OutOfMemoryError> {
        let props = values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| (Identifier::from(idx as i64), value))
            .collect();
        self.allocate_object(|| Object::new(props, None))
    }

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

    // unused_self: Will be used in the future, see comment about storing objects inside the heap.
    #[allow(clippy::unused_self)]
    pub fn resolve<'a>(&self, refr: &'a Reference) -> Ref<'a, Object> {
        refr.deref()
    }
    // unused_self: Will be used in the future, see comment about storing objects inside the heap.
    #[allow(clippy::unused_self)]
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
    // TODO: `Identifier` as the key isn't sufficient here because all sorts of things which aren't
    //  valid identifiers can be used to lookup and set properties (including an empty string, whole
    //  other objects, etc.).
    properties: HashMap<Identifier, Value>,
    callable: Option<Callable>,
}

impl Object {
    fn new(properties: HashMap<Identifier, Value>, callable: Option<Callable>) -> Self {
        Self {
            properties,
            callable,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    pub fn property(&self, name: &Identifier) -> Option<&Value> {
        self.properties.get(name)
    }

    pub fn set_property(&mut self, name: Identifier, value: Value) {
        self.properties.insert(name, value);
    }

    pub fn callable(&self) -> Option<&Callable> {
        self.callable.as_ref()
    }

    /// # Panics
    ///
    /// Always panics.
    pub fn js_to_string(&self) -> String {
        todo!("Object::js_to_string: {:?}", self)
    }
}

#[derive(Debug)]
pub struct Callable {
    declared_parameters: Vec<Identifier>,
    declared_scope: Scope,
    body: Block,
}

impl Callable {
    pub fn new(declared_parameters: Vec<Identifier>, declared_scope: Scope, body: Block) -> Self {
        Self {
            declared_parameters,
            declared_scope,
            body,
        }
    }

    pub fn declared_parameters(&self) -> &[Identifier] {
        &self.declared_parameters
    }

    pub fn declared_scope(&self) -> &Scope {
        &self.declared_scope
    }

    pub fn body(&self) -> &Block {
        &self.body
    }
}
