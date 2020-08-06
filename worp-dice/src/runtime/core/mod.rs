use self::value::Value;
use super::{
    error::RuntimeError,
    symbol::{common::methods::FN_TO_STRING, Symbol},
    types::{func::Func, string::RcString},
};
use key::ValueKey;
use reflection::Type;
use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};

pub mod key;
pub mod reflection;
pub mod value;

thread_local! {
    static TO_STRING: Value = Value::new(Func::new_func1(to_string));
}

fn to_string(object: Value) -> Result<Value, RuntimeError> {
    let string: RcString = object.to_string().into();
    Ok(Value::new(string))
}

/// Trait implemented by types wishing to expose functionality to Dice.
/// Provides several methods, with default implementations, for interacting with the Dice interpreter.
pub trait TypeInstanceBase: Any + Debug + Display {
    /// Get a property by key from the object.
    fn get(&self, key: &ValueKey) -> Result<Value, RuntimeError> {
        if let Some(member) = self.reflect_type().members().get(key) {
            Ok(member.clone())
        } else if key == &ValueKey::Symbol(FN_TO_STRING) {
            Ok(TO_STRING.with(Clone::clone))
        } else {
            self.get_instance_member(key)
        }
    }

    fn get_instance_member(&self, key: &ValueKey) -> Result<Value, RuntimeError> {
        Err(RuntimeError::MissingField(key.clone()))
    }

    /// Set the property by key on the object.
    fn set(&self, _key: &ValueKey, _value: Value) -> Result<(), RuntimeError> {
        Err(RuntimeError::NotAnObject(self.reflect_type().name().clone()))
    }

    /// Reflection facilities.
    fn reflect_type(&self) -> Rc<dyn Type>;

    /// Attempt to xall the object as a function.
    fn call(&self, _args: &[Value]) -> Result<Value, RuntimeError> {
        Err(RuntimeError::NotAFunction(self.reflect_type().name().clone()))
    }
}

/// Trait that's automatically impelemented over all TypeInstanceBase types that provides common functionality to the interpreter.
pub trait TypeInstance: TypeInstanceBase {
    fn as_any(&self) -> &dyn Any;
}

impl<T> TypeInstance for T
where
    T: TypeInstanceBase,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl dyn TypeInstance {
    pub fn value<V: TypeInstanceBase + 'static>(&self) -> Option<&V> {
        self.as_any().downcast_ref::<V>()
    }

    pub fn try_value<V: TypeInstanceBase + 'static>(&self, _type: &Symbol) -> Result<&V, RuntimeError> {
        self.value::<V>()
            .ok_or_else(|| RuntimeError::InvalidType(_type.clone(), self.reflect_type().name().clone()))
    }
}
