use super::func::Func;
use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, key::ObjectKey, reflection::Type, ObjectBase},
    symbol::{common::operators::*, common::types::TY_FLOAT, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeFloat> = Default::default();
}

#[derive(Debug)]
struct TypeFloat {
    name: Symbol,
    instance_members: HashMap<ObjectKey, ObjectInstance>,
}

impl Default for TypeFloat {
    fn default() -> Self {
        Self {
            name: TY_FLOAT,
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

impl Type for TypeFloat {
    fn construct(&self) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::NoConstructor(self.name.clone()))
    }

    fn name(&self) -> &Symbol {
        &self.name
    }

    fn impl_names(&self) -> &[&Symbol] {
        &[]
    }

    fn members(&self) -> &HashMap<ObjectKey, ObjectInstance> {
        &self.instance_members
    }
}

impl ObjectBase for f64 {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

fn negate(value: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let value = value.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(-value))
}

fn mul(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs * rhs))
}

fn div(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs / rhs))
}

fn rem(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs % rhs))
}

fn add(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs + rhs))
}

fn sub(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs - rhs))
}

fn eq(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.value::<f64>();
    let result = match rhs {
        Some(rhs) => lhs == rhs,
        None => false,
    };

    Ok(ObjectInstance::new(result))
}

fn ne(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.value::<f64>();
    let result = match rhs {
        Some(rhs) => lhs != rhs,
        None => true,
    };

    Ok(ObjectInstance::new(result))
}

fn gt(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs > rhs))
}

fn gte(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs >= rhs))
}

fn lt(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs < rhs))
}

fn lte(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(ObjectInstance::new(lhs <= rhs))
}
