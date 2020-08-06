use super::func::Func;
use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, key::ObjectKey, reflection::Type, ObjectBase},
    symbol::{common::operators::*, common::types::TY_NONE, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, fmt::Display, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeNone> = Default::default()
}

#[derive(Debug)]
pub struct TypeNone {
    name: Symbol,
    instance_members: HashMap<ObjectKey, ObjectInstance>,
}

impl Default for TypeNone {
    fn default() -> Self {
        Self {
            name: TY_NONE,
            instance_members: hashmap! [
                ObjectKey::Symbol(OP_EQ) => ObjectInstance::new(Func::new_func2(eq)),
                ObjectKey::Symbol(OP_NE) => ObjectInstance::new(Func::new_func2(ne)),
            ],
        }
    }
}

impl Type for TypeNone {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct None;

impl ObjectBase for None {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}

fn eq(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    lhs.try_value::<None>(&TY_NONE)?;
    let rhs = rhs.value::<None>();

    Ok(ObjectInstance::new(rhs.is_some()))
}

fn ne(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    lhs.try_value::<None>(&TY_NONE)?;
    let rhs = rhs.value::<None>();

    Ok(ObjectInstance::new(rhs.is_none()))
}
