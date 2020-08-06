use super::error::RuntimeError;
use instance::ObjectInstance;
use key::ObjectKey;
use reflection::Type;
use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};

pub mod instance;
pub mod key;
pub mod reflection;

/// Trait implemented by types wishing to expose functionality to Dice.
/// Provides several methods, with default implementations, for interacting with the Dice interpreter.
pub trait ObjectBase: Any + Debug + Display {
    /// Get a property by key from the object.
    fn get(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        if let Some(member) = self.reflect_type().instance_members().get(key) {
            Ok(member.clone())
        } else {
            self.get_instance_member(key)
        }
    }

    fn get_instance_member(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::MissingField(key.clone()))
    }

    /// Set the property by key on the object.
    fn set(&self, _key: &ObjectKey, _value: ObjectInstance) -> Result<(), RuntimeError> {
        Err(RuntimeError::NotAnObject(self.reflect_type().type_name().clone()))
    }

    /// Reflection facilities.
    fn reflect_type(&self) -> Rc<dyn Type>;

    /// Attempt to xall the object as a function.
    fn call(&self, _args: &[ObjectInstance]) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::NotAFunction(self.reflect_type().type_name().clone()))
    }

    /// Get a string representation of the type's value.
    fn format(&self) -> String {
        format!("[{}]", self.reflect_type().type_name())
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
    pub fn value<V: ObjectBase + 'static>(&self) -> Option<&V> {
        self.as_any().downcast_ref::<V>()
    }
}
