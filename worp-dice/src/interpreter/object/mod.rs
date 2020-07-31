use super::error::RuntimeError;
use crate::expression::{ObjectKey, Symbol};
use std::{any::Any, fmt::Debug};

mod func;
mod instance;
mod integer;

pub use instance::*;

#[derive(Debug, Clone)]
pub enum ObjectType {
    None,
    Integer,
    Float,
    String,
    List,
    Object,
    Function,
    Custom(Symbol),
}

pub trait Object: Send + Sync + Any {
    fn as_any(&self) -> &dyn Any;
    fn instance_type(&self) -> ObjectType;

    fn get(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::MissingField(key.clone()))
    }

    fn set(&self, _key: &ObjectKey, _value: ObjectInstance) -> Result<(), RuntimeError> {
        Err(RuntimeError::NotAnObject(self.instance_type()))
    }

    fn call(&self, _args: &[ObjectInstance]) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::NotAFunction(self.instance_type()))
    }

    fn to_string(&self) -> String {
        format!("[{:?}]", self.instance_type())
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

#[derive(Debug)]
pub struct None;

impl Object for None {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn instance_type(&self) -> ObjectType {
        ObjectType::None
    }
}

impl Object for f32 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn instance_type(&self) -> ObjectType {
        ObjectType::Float
    }
}

impl Object for String {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn instance_type(&self) -> ObjectType {
        ObjectType::String
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use func::Func1;

    #[test]
    fn test() -> Result<(), RuntimeError> {
        let arg = ObjectInstance::new(42i32);
        let test_func = ObjectInstance::new(Func1(|arg: ObjectInstance| {
            let arg1 = arg.value::<i32>();

            assert_eq!(42, *arg1.unwrap());

            Ok(ObjectInstance::new(None))
        }));

        let result = test_func.call(&[arg.clone()])?;
        println!("{:?}", result);

        Ok(())
    }
}
