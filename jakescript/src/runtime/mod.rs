pub use global::DefaultGlobalObject;

use crate::ast::Identifier;
use crate::interpreter::{self, ExecutionState, InitialisationError, OutOfMemoryError, Value, Vm};
use common_macros::hash_map;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

mod boolean;
mod console;
mod global;
mod math;
mod number;
mod string;

pub struct Runtime {
    global_object: NativeRef,
    native_heap: NativeHeap,
}

impl Runtime {
    pub fn new<T: Builtin>() -> Result<Self, InitialisationError> {
        let mut native_heap = NativeHeap::default();
        let global_object = T::register(&mut native_heap)?;
        Ok(Self {
            global_object,
            native_heap,
        })
    }

    pub fn global_object_ref(&self) -> NativeRef {
        self.global_object.clone()
    }

    pub fn global_object(&self) -> Ref<NativeObject> {
        self.native_heap.resolve(&self.global_object)
    }
    pub fn global_object_mut(&mut self) -> RefMut<NativeObject> {
        self.native_heap.resolve_mut(&self.global_object)
    }

    // unused_self: May be needed in the future.
    #[allow(clippy::unused_self)]
    pub fn resolve<'a>(&self, refr: &'a NativeRef) -> Ref<'a, NativeObject> {
        self.native_heap.resolve(refr)
    }
    // unused_self: May be needed in the future.
    #[allow(clippy::unused_self)]
    pub fn resolve_mut<'a>(&mut self, refr: &'a NativeRef) -> RefMut<'a, NativeObject> {
        self.native_heap.resolve_mut(refr)
    }
}

#[derive(Default)]
pub struct NativeHeap {
    next_obj_idx: usize,
}

impl NativeHeap {
    fn register_builtin(
        &mut self,
        builtin: impl Builtin + 'static,
    ) -> Result<NativeRef, OutOfMemoryError> {
        let native_obj = NativeObject::new(builtin);
        self.register(native_obj)
    }

    // unused_self: May be needed in the future.
    #[allow(clippy::unused_self)]
    fn register(&mut self, obj: NativeObject) -> Result<NativeRef, OutOfMemoryError> {
        let idx = self.next_obj_idx;
        self.next_obj_idx = self.next_obj_idx.checked_add(1).ok_or(OutOfMemoryError)?;
        Ok(NativeRef::new(idx, obj))
    }

    // unused_self: May be needed in the future.
    #[allow(clippy::unused_self)]
    pub fn resolve<'a>(&self, refr: &'a NativeRef) -> Ref<'a, NativeObject> {
        refr.deref()
    }
    // unused_self: May be needed in the future.
    #[allow(clippy::unused_self)]
    pub fn resolve_mut<'a>(&mut self, refr: &'a NativeRef) -> RefMut<'a, NativeObject> {
        refr.deref_mut()
    }
}

pub struct NativeObject {
    builtin: Box<dyn Builtin>,
    user_properties: HashMap<Identifier, Value>,
}

impl NativeObject {
    fn new(builtin: impl Builtin + 'static) -> Self {
        Self {
            builtin: Box::new(builtin),
            user_properties: hash_map! {},
        }
    }

    pub fn to_js_string(&self) -> String {
        self.builtin.to_js_string()
    }

    pub fn invoke(&self, vm: &mut Vm, args: &[Value]) -> interpreter::Result {
        self.builtin.invoke(vm, args)
    }

    pub fn property(&self, name: &Identifier) -> interpreter::Result {
        Ok(if let Some(user_value) = self.user_properties.get(name) {
            user_value.clone()
        } else if let Some(builtin_value) = self.builtin.property(name)? {
            builtin_value
        } else {
            Value::Undefined
        })
    }

    pub fn set_property(&mut self, name: &Identifier, value: Value) -> interpreter::Result<()> {
        match self.user_properties.entry(name.clone()) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
            }
            Entry::Vacant(entry) => {
                if self.builtin.set_property(name, value.clone())?.is_none() {
                    entry.insert(value);
                }
            }
        }
        Ok(())
    }
}

pub trait Builtin {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError>
    where
        Self: Sized;

    fn to_js_string(&self) -> String {
        "function assert() {\n    [native code]\n}".to_owned()
    }

    fn invoke(&self, vm: &mut Vm, args: &[Value]) -> interpreter::Result {
        let arg = args.first().unwrap_or(&Value::Undefined);
        let exception = Value::String(format!("Type error: {} is not a function", arg));
        vm.set_execution_state(ExecutionState::Exception(exception));
        Ok(Value::Undefined)
    }

    fn property(&self, _: &Identifier) -> interpreter::Result<Option<Value>> {
        Ok(None)
    }

    fn set_property(&mut self, _: &Identifier, _: Value) -> interpreter::Result<Option<()>> {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct NativeRef(usize, Rc<RefCell<NativeObject>>);

impl NativeRef {
    fn new(idx: usize, builtin: NativeObject) -> Self {
        Self(idx, Rc::new(RefCell::new(builtin)))
    }

    fn deref(&self) -> Ref<NativeObject> {
        RefCell::borrow(&self.1)
    }
    fn deref_mut(&self) -> RefMut<NativeObject> {
        RefCell::borrow_mut(&self.1)
    }
}

impl Eq for NativeRef {}

impl PartialEq for NativeRef {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for NativeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Note: 6 includes the 2 chars for the "0x" prefix, so only 4 actual digits are displayed.
        write!(f, "{:#06x}", self.0)
    }
}

impl fmt::Debug for NativeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
