use self::instance::ObjectInstance;
use super::{
    error::RuntimeError,
    symbol::{common::methods::FN_TO_STRING, Symbol},
    types::{func::Func, string::RcString},
};
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

thread_local! {
    static TO_STRING: ObjectInstance = ObjectInstance::new(Func::new_func1(to_string));
}

fn to_string(object: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let string: RcString = object.to_string().into();
    Ok(ObjectInstance::new(string))
}

/// Trait implemented by types wishing to expose functionality to Dice.
/// Provides several methods, with default implementations, for interacting with the Dice interpreter.
pub trait ObjectBase: Any + Debug + Display {
    /// Get a property by key from the object.
    fn get(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        if let Some(member) = self.reflect_type().members().get(key) {
            Ok(member.clone())
        } else if key == &ObjectKey::Symbol(FN_TO_STRING) {
            Ok(TO_STRING.with(Clone::clone))
        } else {
            self.get_instance_member(key)
        }
    }

    fn get_instance_member(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::MissingField(key.clone()))
    }

    /// Set the property by key on the object.
    fn set(&self, _key: &ObjectKey, _value: ObjectInstance) -> Result<(), RuntimeError> {
        Err(RuntimeError::NotAnObject(self.reflect_type().name().clone()))
    }

    /// Reflection facilities.
    fn reflect_type(&self) -> Rc<dyn Type>;

    /// Attempt to xall the object as a function.
    fn call(&self, _args: &[ObjectInstance]) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::NotAFunction(self.reflect_type().name().clone()))
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

    pub fn try_value<V: ObjectBase + 'static>(&self, _type: &Symbol) -> Result<&V, RuntimeError> {
        self.value::<V>()
            .ok_or_else(|| RuntimeError::InvalidType(_type.clone(), self.reflect_type().name().clone()))
    }
}
