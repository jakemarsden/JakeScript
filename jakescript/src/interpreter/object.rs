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

pub struct Property {
    value: Value,
    get: Option<NativeGet>,
    set: Option<NativeSet>,
    writable: Writable,
    enumerable: Enumerable,
}

macro_rules! bool_enum {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum $name {
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

bool_enum!(Writable);
bool_enum!(Enumerable);
bool_enum!(Extensible);

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
            .map(|(idx, value)| (PropertyKey::from(idx), Property::new(value, Writable::Yes)))
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
            .map(|(key, value)| (key, Property::new(value, Writable::Yes)))
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
    ) -> Result<Value, ErrorKind> {
        self.get_impl(it, key, receiver)
            .map(Option::unwrap_or_default)
    }

    pub fn set(
        &mut self,
        it: &mut Interpreter,
        key: &PropertyKey,
        receiver: Reference,
        value: Value,
    ) -> Result<(), ErrorKind> {
        let set = self.set_impl(it, key, receiver, value.clone())?.is_some();
        if !set {
            self.define_own_property(key.clone(), Property::new(value, Writable::Yes));
        }
        Ok(())
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

    fn get_impl(
        &self,
        it: &Interpreter,
        key: &PropertyKey,
        receiver: Reference,
    ) -> Result<Option<Value>, ErrorKind> {
        if let Some(prop) = self.own_property(key) {
            prop.get(it, receiver).map(Some)
        } else if let Some(proto_ref) = self.prototype() {
            let proto_obj = it.vm().heap().resolve(proto_ref);
            proto_obj.get_impl(it, key, receiver)
        } else {
            Ok(None)
        }
    }

    fn set_impl(
        &mut self,
        it: &mut Interpreter,
        key: &PropertyKey,
        receiver: Reference,
        value: Value,
    ) -> Result<Option<()>, ErrorKind> {
        if let Some(prop) = self.own_property_mut(key) {
            prop.set(it, receiver, value).map(Some)
        } else if let Some(proto_ref) = self.prototype() {
            let mut proto_obj = it.vm_mut().heap_mut().resolve_mut(proto_ref);
            proto_obj.set_impl(it, key, receiver, value)
        } else {
            Ok(None)
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
    pub fn new(value: Value, writable: Writable) -> Self {
        Self {
            value,
            set: None,
            get: None,
            writable,
            enumerable: Enumerable::Yes,
        }
    }

    pub fn new_native(
        get: impl Into<NativeGet>,
        set: impl Into<NativeSet>,
        writable: Writable,
        enumerable: Enumerable,
    ) -> Self {
        Self {
            value: Value::default(),
            get: Some(get.into()),
            set: Some(set.into()),
            writable,
            enumerable,
        }
    }

    pub fn get(&self, it: &Interpreter, receiver: Reference) -> Result<Value, ErrorKind> {
        if let Some(ref native) = self.get {
            native.get(it, receiver)
        } else {
            Ok(self.value.clone())
        }
    }

    pub fn set(
        &mut self,
        it: &mut Interpreter,
        receiver: Reference,
        value: Value,
    ) -> Result<(), ErrorKind> {
        if let Some(ref native) = self.set {
            native.set(it, receiver, value)
        } else if matches!(self.writable(), Writable::Yes) {
            self.value = value;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn writable(&self) -> Writable {
        self.writable
    }

    pub fn enumerable(&self) -> Enumerable {
        self.enumerable
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
