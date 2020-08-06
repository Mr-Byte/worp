use super::func::Func;
use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, key::ObjectKey, reflection::Type, ObjectBase},
    symbol::{common::operators::*, common::types::TY_BOOL, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeBool> = Default::default();
}

#[derive(Debug)]
struct TypeBool {
    name: Symbol,
    instance_members: HashMap<ObjectKey, ObjectInstance>,
}

impl Default for TypeBool {
    fn default() -> Self {
        Self {
            name: TY_BOOL,
            instance_members: hashmap! [
                ObjectKey::Symbol(OP_NOT) => ObjectInstance::new(Func::new_func1(not)),
                ObjectKey::Symbol(OP_AND) => ObjectInstance::new(Func::new_func2(and)),
                ObjectKey::Symbol(OP_OR) => ObjectInstance::new(Func::new_func2(or)),
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

impl Type for TypeBool {
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

impl ObjectBase for bool {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

fn not(value: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let value = value.try_value::<bool>(&TY_BOOL)?;

    Ok(ObjectInstance::new(!value))
}

fn eq(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.value::<bool>();
    let result = match rhs {
        Some(rhs) => lhs == rhs,
        None => false,
    };

    Ok(ObjectInstance::new(result))
}

fn ne(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.value::<bool>();
    let result = match rhs {
        Some(rhs) => lhs != rhs,
        None => true,
    };

    Ok(ObjectInstance::new(result))
}

fn and(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(ObjectInstance::new(*lhs && *rhs))
}

fn or(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(ObjectInstance::new(*lhs || *rhs))
}

fn gt(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(ObjectInstance::new(lhs > rhs))
}

fn gte(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(ObjectInstance::new(lhs >= rhs))
}

fn lt(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(ObjectInstance::new(lhs < rhs))
}

fn lte(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(ObjectInstance::new(lhs <= rhs))
}
