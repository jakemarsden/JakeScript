use super::array::ArrayBuiltin;
use super::boolean::BooleanBuiltin;
use super::console::ConsoleBuiltin;
use super::math::MathBuiltin;
use super::number::NumberBuiltin;
use super::string::StringBuiltin;
use super::Builtin;
use crate::interpreter::{
    ExecutionState, Extensible, Heap, InitialisationError, Number, Object, ObjectData, Property,
    Reference, Value,
};
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;

pub struct GlobalObject {
    array: ArrayBuiltin,
    boolean: BooleanBuiltin,
    math: MathBuiltin,
    number: NumberBuiltin,
    string: StringBuiltin,
    obj_ref: Reference,
}

impl GlobalObject {
    pub fn array(&self) -> &ArrayBuiltin {
        &self.array
    }

    pub fn boolean(&self) -> &BooleanBuiltin {
        &self.boolean
    }

    pub fn math(&self) -> &MathBuiltin {
        &self.math
    }

    pub fn number(&self) -> &NumberBuiltin {
        &self.number
    }

    pub fn string(&self) -> &StringBuiltin {
        &self.string
    }
}

impl Builtin for GlobalObject {
    fn init(heap: &mut Heap) -> Result<Self, InitialisationError> {
        let array = ArrayBuiltin::init(heap)?;
        let boolean = BooleanBuiltin::init(heap)?;
        let math = MathBuiltin::init(heap)?;
        let number = NumberBuiltin::init(heap)?;
        let string = StringBuiltin::init(heap)?;

        let console = ConsoleBuiltin::init(heap)?;
        let exit = ExitBuiltin::init(heap)?;
        let is_nan = IsNanBuiltin::init(heap)?;

        let props = hash_map![
            prop_key!("Infinity") => Property::new_const(Value::Number(Number::POS_INF)),
            prop_key!("NaN") => Property::new_const(Value::Number(Number::NAN)),
            prop_key!("undefined") => Property::new_const(Value::Undefined),

            prop_key!("Array") => Property::new_user(array.as_value()),
            prop_key!("Boolean") => Property::new_user(boolean.as_value()),
            prop_key!("Math") => Property::new_user(math.as_value()),
            prop_key!("Number") => Property::new_user(number.as_value()),
            prop_key!("String") => Property::new_user(string.as_value()),

            prop_key!("console") => Property::new_user(console.as_value()),
            prop_key!("exit") => Property::new_user(exit.as_value()),
            prop_key!("isNaN") => Property::new_user(is_nan.as_value()),
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

builtin_fn!(ExitBuiltin, Extensible::Yes, (it, _receiver, _args) => {
    it.vm_mut().set_execution_state(ExecutionState::Exit);
    Ok(Value::Undefined)
});

builtin_fn!(IsNanBuiltin, Extensible::Yes, (_it, _receiver, args) => {
    let arg = args.first().unwrap_or(&Value::Undefined);
    Ok(Value::Boolean(match arg {
        Value::Boolean(_) | Value::Object(_) | Value::Null | Value::Undefined => true,
        Value::Number(arg) => arg.is_nan(),
    }))
});
