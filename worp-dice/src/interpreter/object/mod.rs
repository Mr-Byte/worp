use super::error::RuntimeError;
use std::{any::Any, fmt::Debug};

mod key;
mod reference;
mod types;

pub use key::ObjectKey;
pub use reference::*;

/// Trait implemented by types wishing to expose functionality to Dice.
/// Provides several methods, with default implementations, for interacting with the Dice interpreter.
pub trait ObjectBase: Any + Debug {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        Err(RuntimeError::MissingField(key.clone()))
    }

    fn set(&self, _key: &ObjectKey, _value: ObjectRef) -> Result<(), RuntimeError> {
        Err(RuntimeError::NotAnObject)
    }

    fn call(&self, _args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        Err(RuntimeError::NotAFunction)
    }

    fn to_string(&self) -> String {
        format!("[Object]")
    }
}

/// Trait that's automatically impelemented over all ObjectBase types that provides common functionality to the interpreter.
pub trait Object: ObjectBase {
    fn as_any(&self) -> &dyn Any;
}

impl<T> Object for T
where
    T: ObjectBase,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl dyn Object {
    fn value<V: ObjectBase + 'static>(&self) -> Option<&V> {
        self.as_any().downcast_ref::<V>()
    }
}
