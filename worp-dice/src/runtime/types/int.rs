use super::func::Func;
use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, key::ObjectKey, reflection::Type, ObjectBase},
    symbol::{common::operators::*, common::types::TY_INT, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeInt> = Rc::new(TypeInt::new());
}

struct TypeInt {
    _type: Symbol,
    instance_members: HashMap<ObjectKey, ObjectInstance>,
}

impl TypeInt {
    fn new() -> Self {
        Self {
            _type: TY_INT,
            instance_members: hashmap! [
                ObjectKey::Symbol(OP_NEG) => ObjectInstance::new(Func::new_func1(negate)),
                ObjectKey::Symbol(OP_MUL) => ObjectInstance::new(Func::new_func2(mul)),
                ObjectKey::Symbol(OP_DIV) => ObjectInstance::new(Func::new_func2(div)),
                ObjectKey::Symbol(OP_REM) => ObjectInstance::new(Func::new_func2(rem)),
                ObjectKey::Symbol(OP_ADD) => ObjectInstance::new(Func::new_func2(add)),
                ObjectKey::Symbol(OP_SUB) => ObjectInstance::new(Func::new_func2(sub)),
                ObjectKey::Symbol(OP_EQ) => ObjectInstance::new(Func::new_func2(eq)),
                ObjectKey::Symbol(OP_NE) => ObjectInstance::new(Func::new_func2(ne)),
                ObjectKey::Symbol(OP_GT) => ObjectInstance::new(Func::new_func2(gt)),
                ObjectKey::Symbol(OP_GTE) => ObjectInstance::new(Func::new_func2(gte)),
                ObjectKey::Symbol(OP_LT) => ObjectInstance::new(Func::new_func2(lt)),
                ObjectKey::Symbol(OP_LTE) => ObjectInstance::new(Func::new_func2(lte)),
            ],
        }
    }
}

impl Type for TypeInt {
    fn construct(&self) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::NoConstructor(self._type.clone()))
    }

    fn type_name(&self) -> &Symbol {
        &self._type
    }

    fn impl_names(&self) -> &[&Symbol] {
        &[]
    }

    fn instance_members(&self) -> &HashMap<ObjectKey, ObjectInstance> {
        &self.instance_members
    }
}

impl ObjectBase for i64 {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

fn negate(value: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let value = value.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(-value))
}

fn mul(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs * rhs))
}

fn div(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs / rhs))
}

fn rem(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs % rhs))
}

fn add(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs + rhs))
}

fn sub(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs - rhs))
}

fn eq(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.value::<i64>();
    let result = match rhs {
        Some(rhs) => lhs == rhs,
        None => false,
    };

    Ok(ObjectInstance::new(result))
}

fn ne(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.value::<i64>();
    let result = match rhs {
        Some(rhs) => lhs != rhs,
        None => true,
    };

    Ok(ObjectInstance::new(result))
}

fn gt(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs > rhs))
}

fn gte(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs >= rhs))
}

fn lt(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs < rhs))
}

fn lte(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<i64>(&TY_INT)?;
    let rhs = rhs.try_value::<i64>(&TY_INT)?;

    Ok(ObjectInstance::new(lhs <= rhs))
}
