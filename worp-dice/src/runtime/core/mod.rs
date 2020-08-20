use super::{
    error::RuntimeError,
    lib::{DiceString, Func},
};
use gc::{Finalize, Trace};
use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};
use symbol::common::methods::FN_TO_STRING;

mod key;
mod reflection;
pub mod span;
pub mod symbol;
mod value;

pub use key::ValueKey;
pub use reflection::Type;
pub use span::Span;
pub use symbol::Symbol;
pub use value::Value;

thread_local! {
    static TO_STRING: Value = Value::new(Func::new_func1(to_string));
}

fn to_string(object: Value) -> Result<Value, RuntimeError> {
    let string: DiceString = object.to_string().into();
    Ok(Value::new(string))
}

pub trait TypeInstanceBase: Any + Debug + Display + Trace + Finalize {
    fn as_any(&self) -> &dyn Any;

    /// Reflection facilities.
    fn instance_type(&self) -> Rc<dyn Type>;
    fn reflect_type() -> Rc<dyn Type>
    where
        Self: Sized;
}

/// Trait implemented by types wishing to expose functionality to Dice.
/// Provides several methods, with default implementations, for interacting with the Dice interpreter.
pub trait TypeInstance: TypeInstanceBase {
    /// Get a property by key from the object.
    fn get(&self, key: &ValueKey) -> Result<Value, RuntimeError> {
        if let Some(member) = self.instance_type().members().get(key) {
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
        Err(RuntimeError::NotAnObject(self.instance_type().name().clone()))
    }

    /// Attempt to xall the object as a function.
    fn call(&self, _args: &[Value]) -> Result<Value, RuntimeError> {
        Err(RuntimeError::NotAFunction(self.instance_type().name().clone()))
    }
}

impl dyn TypeInstance {
    pub fn value<V: TypeInstanceBase + 'static>(&self) -> Option<&V> {
        self.as_any().downcast_ref::<V>()
    }

    pub fn try_value<V: TypeInstanceBase + 'static>(&self) -> Result<&V, RuntimeError> {
        let expected = <V as TypeInstanceBase>::reflect_type().name().clone();

        self.value::<V>()
            .ok_or_else(move || RuntimeError::InvalidType(expected, self.instance_type().name().clone()))
    }

    pub fn is_instance_of(&self, expected_type: &dyn Type) -> bool {
        self.instance_type().name() == expected_type.name()
    }

    pub fn is_instance_of_any(&self, expected_types: &[&dyn Type]) -> bool {
        expected_types.iter().any(|typ| self.is_instance_of(*typ))
    }
}

impl PartialEq for dyn TypeInstance {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
