use super::error::{ErrorKind, OutOfMemoryError, VariableNotDefinedError};
use super::stack::Scope;
use super::value::Value;
use super::Interpreter;
use crate::ast::{Block, Identifier};
use crate::runtime::NativeFn;
use crate::str::NonEmptyString;
use common_macros::hash_map;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{hash_map, HashMap};
use std::fmt;
use std::rc::Rc;

// TODO: Introduce a newtype for property keys?
type PropertyKey = NonEmptyString;

#[derive(Debug, Default)]
pub struct Heap {
    next_obj_idx: usize,
}

// TODO: Store objects inside the actual `Heap` struct, rather than ref-counting them in the
//  `Reference` type because currently, the heap stores nothing. This was done for simplicity, and
//  to avoid needing to worry about garbage collection. Also simplify the `Reference` type to
//  `#[derive(Copy, Clone)] pub struct Reference(usize)` when possible.
#[derive(Clone)]
pub struct Reference(usize, Rc<RefCell<Object>>);

#[derive(Debug)]
pub struct Object {
    extensible: bool,
    // TODO: Introduce a newtype for property keys?
    properties: HashMap<PropertyKey, Property>,
    call: Option<Call>,
    string_data: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Property {
    modifiable: bool,
    value: Value,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetPropertyResult {
    /// The property was present and modifiable, so its value was updated.
    Updated,
    /// The property was present and unmodifiable, so its value was not updated.
    NotUpdated,
    /// The property was absent and the object was extensible, so a new property was added.
    Added,
    /// The property was absent and the object was inextensible, so a new property was not added.
    NotAdded,
}

#[derive(Debug)]
pub enum Call {
    User(UserFunction),
    Native(NativeFunction),
}

#[derive(Debug)]
pub struct UserFunction {
    name: Option<Identifier>,
    declared_parameters: Vec<Identifier>,
    declared_scope: Scope,
    body: Block,
}

pub struct NativeFunction(&'static NativeFn);

impl Heap {
    pub fn allocate(&mut self, obj: Object) -> Result<Reference, OutOfMemoryError> {
        let obj_idx = self.next_obj_idx;
        self.next_obj_idx = self.next_obj_idx.checked_add(1).ok_or(OutOfMemoryError)?;
        Ok(Reference::new(obj_idx, obj))
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

impl Object {
    pub fn new_array(elements: Vec<Value>) -> Self {
        let properties = elements
            .into_iter()
            .enumerate()
            .map(|(idx, value)| (PropertyKey::from(idx), Property::new(true, value)))
            .collect();
        Self::new(true, properties, None, None)
    }

    pub fn new_object(properties: HashMap<PropertyKey, Value>) -> Self {
        let properties = properties
            .into_iter()
            .map(|(key, value)| (key, Property::new(true, value)))
            .collect();
        Self::new(true, properties, None, None)
    }

    pub fn new_string(string_data: String) -> Self {
        Self::new(true, hash_map![], None, Some(string_data))
    }

    pub fn new_function(user_fn: UserFunction) -> Self {
        Self::new(true, hash_map![], Some(Call::User(user_fn)), None)
    }

    pub fn new_builtin(
        extensible: bool,
        properties: HashMap<PropertyKey, Property>,
        call: Option<&'static NativeFn>,
    ) -> Self {
        Self::new(
            extensible,
            properties,
            call.map(|f| Call::Native(NativeFunction::new(f))),
            None,
        )
    }

    fn new(
        extensible: bool,
        properties: HashMap<PropertyKey, Property>,
        call: Option<Call>,
        string_data: Option<String>,
    ) -> Self {
        Self {
            extensible,
            properties,
            call,
            string_data,
        }
    }

    pub fn get(&self, key: &PropertyKey) -> Option<&Value> {
        self.properties.get(key).map(Property::value)
    }

    pub fn set(&mut self, key: PropertyKey, value: Value) -> SetPropertyResult {
        match self.properties.entry(key) {
            hash_map::Entry::Occupied(mut entry) => {
                if entry.get_mut().set_value(value) {
                    SetPropertyResult::Updated
                } else {
                    SetPropertyResult::NotUpdated
                }
            }
            hash_map::Entry::Vacant(entry) => {
                if self.extensible {
                    entry.insert(Property::new(true, value));
                    SetPropertyResult::Added
                } else {
                    SetPropertyResult::NotAdded
                }
            }
        }
    }

    pub fn call(&self) -> Option<&Call> {
        self.call.as_ref()
    }

    pub fn string_data(&self) -> Option<&str> {
        self.string_data.as_deref()
    }

    // TODO: Call `.toString()` on the object if it exists.
    #[allow(clippy::unused_self)]
    pub fn js_to_string(&self) -> String {
        self.string_data
            .clone()
            .unwrap_or_else(|| "[object Object]".to_owned())
    }
}

impl Property {
    pub fn new(modifiable: bool, value: Value) -> Self {
        Self { modifiable, value }
    }

    pub fn modifiable(&self) -> bool {
        self.modifiable
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn set_value(&mut self, value: Value) -> bool {
        if self.modifiable {
            self.value = value;
        }
        self.modifiable
    }
}

impl SetPropertyResult {
    pub fn into_result(self) -> Result<bool, VariableNotDefinedError> {
        match self {
            Self::Updated | Self::Added => Ok(true),
            Self::NotUpdated => Ok(false),
            Self::NotAdded => Err(VariableNotDefinedError),
        }
    }
}

impl UserFunction {
    pub fn new(
        name: Option<Identifier>,
        declared_parameters: Vec<Identifier>,
        declared_scope: Scope,
        body: Block,
    ) -> Self {
        Self {
            name,
            declared_parameters,
            declared_scope,
            body,
        }
    }

    pub fn name(&self) -> Option<&Identifier> {
        self.name.as_ref()
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

impl NativeFunction {
    pub fn new(f: &'static NativeFn) -> Self {
        Self(f)
    }

    pub fn call(&self, it: &mut Interpreter, args: &[Value]) -> Result<Value, ErrorKind> {
        self.0(it, args)
    }
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NativeFunction({:p})", self.0)
    }
}
