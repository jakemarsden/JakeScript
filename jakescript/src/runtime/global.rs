use super::array::Array;
use super::boolean::Boolean;
use super::console::Console;
use super::math::Math;
use super::number::Number;
use super::string::String;
use super::Builtin;
use crate::interpreter::{
    self, ExecutionState, Extensible, Heap, InitialisationError, Object, ObjectData, Property,
    Reference, Value, Writable,
};
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;

pub struct GlobalObject {
    array: Array,
    boolean: Boolean,
    math: Math,
    number: Number,
    string: String,
    obj_ref: Reference,
}

impl GlobalObject {
    pub fn array(&self) -> &Array {
        &self.array
    }

    pub fn boolean(&self) -> &Boolean {
        &self.boolean
    }

    pub fn math(&self) -> &Math {
        &self.math
    }

    pub fn number(&self) -> &Number {
        &self.number
    }

    pub fn string(&self) -> &String {
        &self.string
    }
}

impl Builtin for GlobalObject {
    fn init(heap: &mut Heap) -> Result<Self, InitialisationError> {
        let array = Array::init(heap)?;
        let boolean = Boolean::init(heap)?;
        let math = Math::init(heap)?;
        let number = Number::init(heap)?;
        let string = String::init(heap)?;

        let console = Console::init(heap)?;
        let exit = Exit::init(heap)?;
        let is_nan = IsNaN::init(heap)?;

        let props = hash_map![
            prop_key!("Infinity") => Property::new(
                Value::Number(interpreter::Number::POS_INF),
                Writable::No
            ),
            prop_key!("NaN") => Property::new(
                Value::Number(interpreter::Number::NAN),
                Writable::No
            ),
            prop_key!("undefined") => Property::new(Value::Undefined, Writable::No),

            prop_key!("Array") => Property::new(array.as_value(), Writable::Yes),
            prop_key!("Boolean") => Property::new(boolean.as_value(), Writable::Yes),
            prop_key!("Math") => Property::new(math.as_value(), Writable::Yes),
            prop_key!("Number") => Property::new(number.as_value(), Writable::Yes),
            prop_key!("String") => Property::new(string.as_value(), Writable::Yes),

            prop_key!("console") => Property::new(console.as_value(), Writable::Yes),
            prop_key!("exit") => Property::new(exit.as_value(), Writable::Yes),
            prop_key!("isNaN") => Property::new(is_nan.as_value(), Writable::Yes),
        ];

        let obj_ref = heap.allocate(Object::new(None, props, ObjectData::None, Extensible::Yes))?;
        Ok(Self {
            array,
            boolean,
            math,
            number,
            string,
            obj_ref,
        })
    }

    fn obj_ref(&self) -> &Reference {
        &self.obj_ref
    }
}

builtin_fn!(Exit, Extensible::Yes, (it, _receiver, _args) => {
    it.vm_mut().set_execution_state(ExecutionState::Exit);
    Ok(Value::Undefined)
});

builtin_fn!(IsNaN, Extensible::Yes, (_it, _receiver, args) => {
    let arg = args.first().unwrap_or(&Value::Undefined);
    Ok(Value::Boolean(match arg {
        Value::Boolean(_) | Value::Object(_) | Value::Null | Value::Undefined => true,
        Value::Number(arg) => arg.is_nan(),
    }))
});
