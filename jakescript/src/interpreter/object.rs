use super::error::ErrorKind;
use super::heap::Reference;
use super::stack::Scope;
use super::value::Value;
use super::Interpreter;
use crate::ast::{Block, Identifier};
use crate::runtime::{NativeCall, NativeGet, NativeSet};
use crate::str::NonEmptyString;
use common_macros::hash_map;
use std::collections::HashMap;

#[macro_export]
macro_rules! prop_key {
    ($lit:literal) => {{
        use $crate::non_empty_str;
        $crate::interpreter::PropertyKey::from(non_empty_str!($lit))
    }};
}

macro_rules! bool_enum {
    ($vis:vis $name:ident) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        $vis enum $name {
            No,
            Yes,
        }

        impl From<bool> for $name {
            fn from(b: bool) -> Self {
                match b {
                    false => Self::No,
                    true => Self::Yes,
                }
            }
        }
    };
}

pub struct Object {
    proto: Option<Reference>,
    props: HashMap<PropertyKey, Property>,
    data: ObjectData,
    extensible: Extensible,
}

#[derive(Debug, Default)]
pub enum ObjectData {
    #[default]
    None,
    Call(Call),
    String(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PropertyKey(NonEmptyString);

/// [Table 4 — Default Attribute Values](https://262.ecma-international.org/6.0/#table-4)
pub struct Property {
    value: Value,
    get: Option<NativeGet>,
    set: Option<NativeSet>,
    writable: Writable,
    enumerable: Enumerable,
    configurable: Configurable,
}

bool_enum!(pub Configurable);
bool_enum!(pub Enumerable);
bool_enum!(pub Extensible);
bool_enum!(pub Writable);

#[derive(Clone, Debug)]
pub enum Call {
    User(UserFunction),
    Native(NativeCall),
}

#[derive(Clone, Debug)]
pub struct UserFunction {
    name: Option<Identifier>,
    declared_parameters: Vec<Identifier>,
    declared_scope: Scope,
    body: Block,
}

impl Object {
    pub fn new_string(proto: Reference, data: String, extensible: Extensible) -> Self {
        Self::new(
            Some(proto),
            hash_map![],
            ObjectData::String(data),
            extensible,
        )
    }

    pub fn new_array(proto: Reference, elems: Vec<Value>, extensible: Extensible) -> Self {
        let props = elems
            .into_iter()
            .enumerate()
            .map(|(idx, value)| (PropertyKey::from(idx), Property::new_enumerable(value)))
            .collect();
        Self::new(Some(proto), props, ObjectData::None, extensible)
    }

    pub fn new_object(
        proto: Option<Reference>,
        props: HashMap<PropertyKey, Value>,
        extensible: Extensible,
    ) -> Self {
        let props = props
            .into_iter()
            .map(|(key, value)| (key, Property::new_user(value)))
            .collect();
        Self::new(proto, props, ObjectData::None, extensible)
    }

    pub fn new_function(call: UserFunction, extensible: Extensible) -> Self {
        Self::new(
            None,
            hash_map![],
            ObjectData::Call(Call::User(call)),
            extensible,
        )
    }

    pub fn new_native(
        proto: Option<Reference>,
        props: HashMap<PropertyKey, Property>,
        call: impl Into<NativeCall>,
        extensible: Extensible,
    ) -> Self {
        Self::new(
            proto,
            props,
            ObjectData::Call(Call::Native(call.into())),
            extensible,
        )
    }

    pub fn new(
        proto: Option<Reference>,
        props: HashMap<PropertyKey, Property>,
        data: ObjectData,
        extensible: Extensible,
    ) -> Self {
        Self {
            proto,
            props,
            data,
            extensible,
        }
    }

    pub fn prototype(&self) -> Option<&Reference> {
        self.proto.as_ref()
    }

    pub fn own_property(&self, key: &PropertyKey) -> Option<&Property> {
        self.props.get(key)
    }
    pub fn own_property_mut(&mut self, key: &PropertyKey) -> Option<&mut Property> {
        self.props.get_mut(key)
    }

    pub fn define_own_property(&mut self, key: PropertyKey, value: Property) {
        self.props.insert(key, value);
    }

    pub fn own_property_count(&self) -> usize {
        self.props
            .iter()
            .filter(|(_, prop)| matches!(prop.enumerable(), Enumerable::Yes))
            .count()
    }

    pub fn get(
        &self,
        it: &Interpreter,
        key: &PropertyKey,
        receiver: Reference,
    ) -> Result<Option<Value>, ErrorKind> {
        if let Some(prop) = self.own_property(key) {
            prop.get(it, receiver).map(Some)
        } else if let Some(proto_ref) = self.prototype() {
            let proto_obj = it.vm().heap().resolve(proto_ref);
            proto_obj.get(it, key, receiver)
        } else {
            Ok(None)
        }
    }

    pub fn set(
        &mut self,
        it: &mut Interpreter,
        key: &PropertyKey,
        receiver: Reference,
        value: Value,
    ) -> Result<bool, ErrorKind> {
        if let Some(prop) = self.own_property_mut(key) {
            prop.set(it, receiver, value)
        } else if let Some(proto_ref) = self.prototype() {
            let mut proto_obj = it.vm_mut().heap_mut().resolve_mut(proto_ref);
            proto_obj.set(it, key, receiver, value)
        } else if matches!(self.extensible(), Extensible::Yes) {
            self.define_own_property(key.clone(), Property::new_user(value));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn call_data(&self) -> Option<&Call> {
        match self.data() {
            ObjectData::Call(data) => Some(data),
            ObjectData::None | ObjectData::String(_) => None,
        }
    }

    pub fn string_data(&self) -> Option<&str> {
        match self.data() {
            ObjectData::String(data) => Some(data),
            ObjectData::None | ObjectData::Call(_) => None,
        }
    }

    pub fn data(&self) -> &ObjectData {
        &self.data
    }

    pub fn extensible(&self) -> Extensible {
        self.extensible
    }

    pub fn js_to_string(&self) -> String {
        if let Some(data) = self.string_data() {
            data.to_owned()
        } else {
            "[object Object]".to_owned()
        }
    }
}

impl PropertyKey {
    pub fn into_inner(self) -> NonEmptyString {
        self.0
    }
    pub fn inner(&self) -> &NonEmptyString {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        self.inner().as_str()
    }
}

impl From<usize> for PropertyKey {
    fn from(idx: usize) -> Self {
        Self::from(NonEmptyString::from(idx))
    }
}

impl From<&Identifier> for PropertyKey {
    fn from(id: &Identifier) -> Self {
        Self::from(id.inner().clone())
    }
}

impl From<Identifier> for PropertyKey {
    fn from(id: Identifier) -> Self {
        Self::from(id.into_inner())
    }
}

impl From<NonEmptyString> for PropertyKey {
    fn from(s: NonEmptyString) -> Self {
        Self(s)
    }
}

impl Property {
    pub fn new_const(value: Value) -> Self {
        Self::new_data(value, Writable::No, Enumerable::No, Configurable::No)
    }

    pub fn new_user(value: Value) -> Self {
        Self::new_data(value, Writable::Yes, Enumerable::No, Configurable::Yes)
    }

    pub fn new_enumerable(value: Value) -> Self {
        Self::new_data(value, Writable::Yes, Enumerable::Yes, Configurable::Yes)
    }

    /// [Table 2 — Attributes of a Data Property](https://262.ecma-international.org/6.0/#table-2)
    pub fn new_data(
        value: Value,
        writable: Writable,
        enumerable: Enumerable,
        configurable: Configurable,
    ) -> Self {
        Self {
            value,
            get: None,
            set: None,
            writable,
            enumerable,
            configurable,
        }
    }

    pub fn new_const_accessor(get: impl Into<NativeGet>) -> Self {
        Self::new_accessor(Some(get.into()), None, Enumerable::No, Configurable::No)
    }

    /// [Table 3 — Attributes of an Accessor Property](
    /// https://262.ecma-international.org/6.0/#table-3)
    pub fn new_accessor(
        get: Option<NativeGet>,
        set: Option<NativeSet>,
        enumerable: Enumerable,
        configurable: Configurable,
    ) -> Self {
        Self {
            value: Value::Undefined,
            get,
            set,
            writable: Writable::No,
            enumerable,
            configurable,
        }
    }

    pub fn get(&self, it: &Interpreter, receiver: Reference) -> Result<Value, ErrorKind> {
        if let Some(ref get) = self.get {
            get.get(it, receiver)
        } else {
            Ok(self.value.clone())
        }
    }

    pub fn set(
        &mut self,
        it: &mut Interpreter,
        receiver: Reference,
        value: Value,
    ) -> Result<bool, ErrorKind> {
        if let Some(ref native) = self.set {
            native.set(it, receiver, value)
        } else if matches!(self.writable(), Writable::Yes) {
            self.value = value;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn writable(&self) -> Writable {
        self.writable
    }

    pub fn enumerable(&self) -> Enumerable {
        self.enumerable
    }

    pub fn configurable(&self) -> Configurable {
        self.configurable
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
