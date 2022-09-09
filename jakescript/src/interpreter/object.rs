use super::error::{ErrorKind, NotCallableError};
use super::heap::Reference;
use super::stack::Scope;
use super::value::Value;
use super::Interpreter;
use crate::ast::{Block, Identifier};
use crate::runtime::NativeCall;
use common_macros::hash_map;
use std::collections::{hash_map, HashMap};

#[macro_export]
macro_rules! prop_key {
    ($lit:literal) => {{
        use $crate::ident;
        ident!($lit)
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

impl Object {
    pub fn new_string(proto: Reference, data: Box<str>, extensible: Extensible) -> Self {
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
    pub fn set_prototype(&mut self, proto: Option<Reference>) -> bool {
        match self.extensible() {
            Extensible::Yes => {
                self.proto = proto;
                true
            }
            Extensible::No => self.proto == proto,
        }
    }

    pub fn own_property_keys(&self) -> impl Iterator<Item = &PropertyKey> {
        self.props.keys()
    }

    pub fn own_property(&self, key: &PropertyKey) -> Option<&Property> {
        self.props.get(key)
    }
    pub fn own_property_mut(&mut self, key: &PropertyKey) -> Option<&mut Property> {
        self.props.get_mut(key)
    }

    pub fn define_own_property(&mut self, key: PropertyKey, value: Property) -> bool {
        match (self.extensible(), self.props.entry(key)) {
            (Extensible::Yes, hash_map::Entry::Occupied(mut entry)) => {
                entry.insert(value);
                true
            }
            (Extensible::Yes, hash_map::Entry::Vacant(entry)) => {
                entry.insert(value);
                true
            }
            (Extensible::No, hash_map::Entry::Occupied(entry)) => entry.get() == &value,
            (Extensible::No, hash_map::Entry::Vacant(_)) => false,
        }
    }

    pub fn get(
        &self,
        it: &mut Interpreter,
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

    pub fn delete(&mut self, key: &PropertyKey) -> Result<bool, ErrorKind> {
        Ok(match self.props.entry(key.clone()) {
            hash_map::Entry::Occupied(entry) if entry.get().deletable() => {
                entry.remove();
                true
            }
            hash_map::Entry::Occupied(_) => false,
            hash_map::Entry::Vacant(_) => true,
        })
    }

    pub fn call(
        &self,
        it: &mut Interpreter,
        self_ref: &Reference,
        receiver: Option<Reference>,
        args: &[Value],
    ) -> Result<Value, ErrorKind> {
        match self.call_data() {
            Some(Call::User(ref user_fn)) => it
                .call_user_fn(user_fn, self_ref, receiver, args)
                .map_err(|err| ErrorKind::Boxed(Box::new(err))),
            Some(Call::Native(ref native_fn)) => it.call_native_fn(native_fn, receiver, args),
            None => Err(ErrorKind::from(NotCallableError)),
        }
    }

    pub fn call_data(&self) -> Option<&Call> {
        match self.data {
            ObjectData::Call(ref data) => Some(data),
            ObjectData::None | ObjectData::String(_) => None,
        }
    }

    pub fn string_data(&self) -> Option<&str> {
        match self.data {
            ObjectData::String(ref data) => Some(data),
            ObjectData::None | ObjectData::Call(_) => None,
        }
    }

    pub fn extensible(&self) -> Extensible {
        self.extensible
    }

    pub fn js_to_string(&self) -> Box<str> {
        match self.data {
            ObjectData::String(ref data) => data.clone(),
            ObjectData::None | ObjectData::Call(_) => Box::from("[object Object]"),
        }
    }
}

#[derive(Debug, Default)]
pub enum ObjectData {
    #[default]
    None,
    Call(Call),
    String(Box<str>),
}

pub type PropertyKey = Identifier;

/// [Table 4 — Default Attribute Values](https://262.ecma-international.org/6.0/#table-4)
#[derive(PartialEq)]
pub struct Property(PropertyInner);

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
        Self(PropertyInner::Data(DataProperty {
            value,
            writable,
            enumerable,
            configurable,
        }))
    }

    pub fn new_const_accessor(get: Reference) -> Self {
        Self::new_accessor(Some(get), None, Enumerable::No, Configurable::No)
    }

    /// [Table 3 — Attributes of an Accessor Property](
    /// https://262.ecma-international.org/6.0/#table-3)
    pub fn new_accessor(
        get: Option<Reference>,
        set: Option<Reference>,
        enumerable: Enumerable,
        configurable: Configurable,
    ) -> Self {
        Self(PropertyInner::Accessor(AccessorProperty {
            get,
            set,
            enumerable,
            configurable,
        }))
    }

    pub fn get(&self, it: &mut Interpreter, receiver: Reference) -> Result<Value, ErrorKind> {
        match self.0 {
            PropertyInner::Data(ref inner) => Ok(inner.value.clone()),
            PropertyInner::Accessor(ref inner) => match inner.get {
                Some(ref get) => {
                    let get_obj = it.vm().heap().resolve(get);
                    get_obj.call(it, get, Some(receiver), &[])
                }
                None => Ok(Value::Undefined),
            },
        }
    }

    pub fn set(
        &mut self,
        it: &mut Interpreter,
        receiver: Reference,
        value: Value,
    ) -> Result<bool, ErrorKind> {
        Ok(match self.0 {
            PropertyInner::Data(ref mut inner) => match inner.writable {
                Writable::Yes => {
                    inner.value = value;
                    true
                }
                Writable::No => inner.value == value,
            },
            PropertyInner::Accessor(ref inner) => match inner.set {
                Some(ref set) => {
                    let set_obj = it.vm().heap().resolve(set);
                    set_obj.call(it, set, Some(receiver), &[value])?;
                    true
                }
                None => false,
            },
        })
    }

    pub fn deletable(&self) -> bool {
        match self.configurable() {
            Configurable::Yes => true,
            Configurable::No => false,
        }
    }

    pub fn writable(&self) -> Writable {
        match self.0 {
            PropertyInner::Data(ref inner) => inner.writable,
            PropertyInner::Accessor(ref inner) => Writable::from(inner.set.is_some()),
        }
    }

    pub fn enumerable(&self) -> Enumerable {
        match self.0 {
            PropertyInner::Data(ref inner) => inner.enumerable,
            PropertyInner::Accessor(ref inner) => inner.enumerable,
        }
    }

    pub fn configurable(&self) -> Configurable {
        match self.0 {
            PropertyInner::Data(ref inner) => inner.configurable,
            PropertyInner::Accessor(ref inner) => inner.configurable,
        }
    }
}

#[derive(PartialEq)]
enum PropertyInner {
    Data(DataProperty),
    Accessor(AccessorProperty),
}

/// [Table 2 — Attributes of a Data Property](https://262.ecma-international.org/6.0/#table-2)
#[derive(PartialEq)]
struct DataProperty {
    value: Value,
    writable: Writable,
    enumerable: Enumerable,
    configurable: Configurable,
}

/// [Table 3 — Attributes of an Accessor Property](https://262.ecma-international.org/6.0/#table-3)
#[derive(Eq, PartialEq)]
struct AccessorProperty {
    get: Option<Reference>,
    set: Option<Reference>,
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
