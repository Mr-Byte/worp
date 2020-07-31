use super::{instance::ObjectInstance, Object, ObjectType};
use crate::interpreter::error::RuntimeError;
use std::any::Any;

pub struct Func0<F>(pub F)
where
    F: Fn() -> Result<ObjectInstance, RuntimeError>;

impl<F> Object for Func0<F>
where
    F: Fn() -> Result<ObjectInstance, RuntimeError> + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn instance_type(&self) -> ObjectType {
        ObjectType::Function
    }

    fn call(&self, args: &[ObjectInstance]) -> Result<ObjectInstance, RuntimeError> {
        if let [] = args {
            Ok((self.0)()?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(0, args.len()))
        }
    }
}

pub struct Func1<F>(pub F)
where
    F: Fn(ObjectInstance) -> Result<ObjectInstance, RuntimeError>;

impl<F> Object for Func1<F>
where
    F: Fn(ObjectInstance) -> Result<ObjectInstance, RuntimeError> + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn instance_type(&self) -> ObjectType {
        ObjectType::Function
    }

    fn call(&self, args: &[ObjectInstance]) -> Result<ObjectInstance, RuntimeError> {
        if let [arg1] = args {
            Ok((self.0)(arg1.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }
}

pub struct Func2<F>(pub F)
where
    F: Fn(ObjectInstance, ObjectInstance) -> Result<ObjectInstance, RuntimeError>;

impl<F> Object for Func2<F>
where
    F: Fn(ObjectInstance, ObjectInstance) -> Result<ObjectInstance, RuntimeError> + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn instance_type(&self) -> ObjectType {
        ObjectType::Function
    }

    fn call(&self, args: &[ObjectInstance]) -> Result<ObjectInstance, RuntimeError> {
        if let [arg1, arg2] = args {
            Ok((self.0)(arg1.clone(), arg2.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }
}

pub struct Func3<F>(pub F)
where
    F: Fn(ObjectInstance, ObjectInstance, ObjectInstance) -> Result<ObjectInstance, RuntimeError>;

impl<F> Object for Func3<F>
where
    F: Fn(ObjectInstance, ObjectInstance, ObjectInstance) -> Result<ObjectInstance, RuntimeError> + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn instance_type(&self) -> ObjectType {
        ObjectType::Function
    }

    fn call(&self, args: &[ObjectInstance]) -> Result<ObjectInstance, RuntimeError> {
        if let [arg1, arg2, arg3] = args {
            Ok((self.0)(arg1.clone(), arg2.clone(), arg3.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(3, args.len()))
        }
    }
}
