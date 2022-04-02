use super::{Builtin, NativeHeap, NativeRef};
use crate::ast::Identifier;
use crate::interpreter::{ErrorKind, InitialisationError, Number, Value, Vm};

pub struct Math {
    sqrt: Value,
}

pub struct MathSqrt;

impl Builtin for Math {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        let math = Self {
            sqrt: Value::NativeObject(MathSqrt::register(run)?),
        };
        Ok(run.register_builtin(math)?)
    }

    fn property(&self, name: &Identifier) -> Result<Option<Value>, ErrorKind> {
        Ok(match name.as_str() {
            "sqrt" => Some(self.sqrt.clone()),
            _ => None,
        })
    }

    fn set_property(&mut self, name: &Identifier, value: Value) -> Result<Option<()>, ErrorKind> {
        Ok(match name.as_str() {
            "sqrt" => {
                self.sqrt = value;
                Some(())
            }
            _ => None,
        })
    }
}

impl Builtin for MathSqrt {
    fn register(run: &mut NativeHeap) -> Result<NativeRef, InitialisationError> {
        Ok(run.register_builtin(Self)?)
    }

    fn invoke(&self, _: &mut Vm, args: &[Value]) -> Result<Value, ErrorKind> {
        let arg = args.first();
        Ok(Value::Number(match arg {
            Some(arg) => arg.coerce_to_number().sqrt(),
            None => Number::NAN,
        }))
    }
}
