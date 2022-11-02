use super::array::{ArrayCtorBuiltin, ArrayProtoBuiltin};
use super::boolean::BooleanCtorBuiltin;
use super::console::ConsoleBuiltin;
use super::function::FunctionProtoBuiltin;
use super::math::MathBuiltin;
use super::number::NumberCtorBuiltin;
use super::string::{StringCtorBuiltin, StringProtoBuiltin};
use super::Builtin;
use crate::interpreter::{
    ExecutionState, Extensible, Heap, InitialisationError, Number, Object, ObjectData, Property,
    Reference, Value,
};
use crate::runtime::object::ObjectProtoBuiltin;
use crate::{builtin_fn, prop_key};
use common_macros::hash_map;

pub struct GlobalObjectProto {
    obj_ref: Reference,
}

pub struct GlobalObject {
    // TODO: Prototypes should probably be members of the `Runtime` rather than of the
    // `GlobalObject`.
    array_proto: ArrayProtoBuiltin,
    string_proto: StringProtoBuiltin,
    obj_ref: Reference,
}

impl Builtin for GlobalObjectProto {
    type InitArgs = Reference;

    fn init(heap: &mut Heap, obj_proto: Self::InitArgs) -> Result<Self, InitialisationError> {
        let obj_ref = heap.allocate(Object::new(
            Some(obj_proto),
            hash_map![],
            ObjectData::None,
            Extensible::Yes,
        ))?;
        Ok(Self { obj_ref })
    }

    fn obj_ref(&self) -> Reference {
        self.obj_ref
    }
}

impl GlobalObject {
    pub fn array_proto(&self) -> &ArrayProtoBuiltin {
        &self.array_proto
    }

    pub fn string_proto(&self) -> &StringProtoBuiltin {
        &self.string_proto
    }
}

impl Builtin for GlobalObject {
    fn init(heap: &mut Heap, (): Self::InitArgs) -> Result<Self, InitialisationError> {
        let obj_proto = ObjectProtoBuiltin::init(heap, ())?;
        let fn_proto = FunctionProtoBuiltin::init(heap, obj_proto.obj_ref())?;
        let global_obj_proto = GlobalObjectProto::init(heap, obj_proto.obj_ref())?;

        let array_proto = ArrayProtoBuiltin::init(heap, (obj_proto.obj_ref(), fn_proto.obj_ref()))?;
        let string_proto =
            StringProtoBuiltin::init(heap, (obj_proto.obj_ref(), fn_proto.obj_ref()))?;

        let array = ArrayCtorBuiltin::init(heap, fn_proto.obj_ref())?;
        let boolean = BooleanCtorBuiltin::init(heap, fn_proto.obj_ref())?;
        let math = MathBuiltin::init(heap, (obj_proto.obj_ref(), fn_proto.obj_ref()))?;
        let number = NumberCtorBuiltin::init(heap, fn_proto.obj_ref())?;
        let string = StringCtorBuiltin::init(heap, fn_proto.obj_ref())?;

        let console = ConsoleBuiltin::init(heap, (obj_proto.obj_ref(), fn_proto.obj_ref()))?;
        let exit = ExitBuiltin::init(heap, fn_proto.obj_ref())?;
        let is_nan = IsNanBuiltin::init(heap, fn_proto.obj_ref())?;

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

        let obj_ref = heap.allocate(Object::new(
            Some(global_obj_proto.obj_ref()),
            props,
            ObjectData::None,
            Extensible::Yes,
        ))?;
        Ok(Self {
            array_proto,
            string_proto,
            obj_ref,
        })
    }

    fn obj_ref(&self) -> Reference {
        self.obj_ref
    }
}

builtin_fn!(ExitBuiltin, Extensible::Yes, (it, _receiver, _args) => {
    it.vm_mut().set_execution_state(ExecutionState::Exit);
    Ok(Value::Undefined)
});

builtin_fn!(IsNanBuiltin, Extensible::Yes, (_it, _receiver, args) => {
    let arg = args.first().copied().unwrap_or(Value::Undefined);
    Ok(Value::Boolean(match arg {
        Value::Boolean(_) | Value::Object(_) | Value::Null | Value::Undefined => true,
        Value::Number(arg) => arg.is_nan(),
    }))
});
