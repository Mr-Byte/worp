use super::{
    func::{Func1, Func2},
    instance::ObjectInstance,
    Object, ObjectType,
};
use crate::{expression::ObjectKey, interpreter::error::RuntimeError};
use lazy_static::lazy_static;
use std::{any::Any, collections::HashMap};

lazy_static! {
    static ref INTEGER_OPERATIONS: HashMap<ObjectKey, ObjectInstance> = {
        let mut ops: HashMap<ObjectKey, ObjectInstance> = HashMap::new();

        ops.insert("#op_negate".into(), ObjectInstance::new(Func1(negate)));
        ops.insert("#op_mul".into(), ObjectInstance::new(Func2(mul)));
        ops.insert("#op_div".into(), ObjectInstance::new(Func2(div)));
        ops.insert("#op_rem".into(), ObjectInstance::new(Func2(rem)));
        ops.insert("#op_add".into(), ObjectInstance::new(Func2(add)));
        ops.insert("#op_sub".into(), ObjectInstance::new(Func2(sub)));
        ops
    };
}

fn negate(arg: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    if let Some(value) = arg.value::<i32>() {
        Ok(ObjectInstance::new(!value))
    } else {
        Err(RuntimeError::InvalidType(ObjectType::Integer, arg.instance_type()))
    }
}

fn mul(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let args = (lhs.value::<i32>(), rhs.value::<i32>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectInstance::new(lhs * rhs)),
        _ => Err(RuntimeError::Aborted),
    }
}

fn div(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let args = (lhs.value::<i32>(), rhs.value::<i32>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectInstance::new(lhs * rhs)),
        _ => Err(RuntimeError::Aborted),
    }
}

fn rem(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let args = (lhs.value::<i32>(), rhs.value::<i32>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectInstance::new(lhs % rhs)),
        _ => Err(RuntimeError::Aborted),
    }
}

fn add(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let args = (lhs.value::<i32>(), rhs.value::<i32>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectInstance::new(lhs + rhs)),
        _ => Err(RuntimeError::Aborted),
    }
}

fn sub(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let args = (lhs.value::<i32>(), rhs.value::<i32>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectInstance::new(lhs - rhs)),
        _ => Err(RuntimeError::Aborted),
    }
}

impl Object for i32 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        INTEGER_OPERATIONS
            .get(key)
            .cloned()
            .ok_or_else(|| RuntimeError::MissingField(key.clone()))
    }

    fn instance_type(&self) -> ObjectType {
        ObjectType::Integer
    }
}
