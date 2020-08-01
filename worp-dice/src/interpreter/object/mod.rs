use super::error::RuntimeError;
use std::{any::Any, fmt::Debug};

mod key;
mod reference;
mod types;

pub use key::ObjectKey;
pub use reference::*;

pub trait Object: Any + Debug {
    fn as_any(&self) -> &dyn Any;

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

impl dyn Object
where
    Self: Any,
{
    #[inline]
    fn value<T: Object>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}
