use super::error::RuntimeError;
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
        Err(RuntimeError::NotAnObject(self.instance_type_data().type_tag().clone()))
    }

    /// Reflection facilities.

    fn type_data() -> TypeData
    where
        Self: Sized;
    fn instance_type_data(&self) -> TypeData;
    fn properties(&self) -> Vec<(ObjectKey, TypeData)>;

    /// Attempt to xall the object as a function.
    fn call(&self, _args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        Err(RuntimeError::NotAFunction(self.instance_type_data().type_tag().clone()))
    }

    /// Get a string representation of the type's value.
    fn to_string(&self) -> String {
        format!("[{}]", self.instance_type_data().type_tag())
    }
}

trait ObjectStatic: ObjectBase {
    fn type_data() -> TypeData
    where
        Self: Sized;
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
