use super::{error::RuntimeError, symbol::Symbol};
use std::{any::Any, fmt::Debug};

mod key;
mod operator;
mod reference;
pub mod reflection;
mod types;

pub use key::ObjectKey;
pub use reference::*;
use reflection::TypeData;

/// Trait implemented by types wishing to expose functionality to Dice.
/// Provides several methods, with default implementations, for interacting with the Dice interpreter.
pub trait ObjectBase: Any + Debug {
    /// Get a property by key from the object.
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        Err(RuntimeError::MissingField(key.clone()))
    }

    /// Set the property by key on the object.
    fn set(&self, _key: &ObjectKey, _value: ObjectRef) -> Result<(), RuntimeError> {
        Err(RuntimeError::NotAnObject(self.type_data().tag().clone()))
    }

    /// Retrieve a list of properties on this object and their type data.
    fn properties(&self) -> Vec<(ObjectKey, TypeData)>;
    fn type_data(&self) -> TypeData;

    /// Attempt to xall the object as a function.
    fn call(&self, _args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        Err(RuntimeError::NotAFunction(self.type_data().tag().clone()))
    }

    /// Get a string representation of the type's value.
    fn to_string(&self) -> String {
        format!("[{}]", *self.type_data().tag())
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

    fn tag(&self) -> Symbol {
        self.type_data().tag().clone()
    }
}
