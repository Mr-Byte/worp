use super::func::Func;
use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, key::ObjectKey, reflection::Type, ObjectBase},
    symbol::{common::operators::OP_ADD, common::types::TY_LIST, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, fmt::Display, iter, ops::Deref, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeList> = Default::default();
}

struct TypeList {
    name: Symbol,
    members: HashMap<ObjectKey, ObjectInstance>,
}

impl Default for TypeList {
    fn default() -> Self {
        Self {
            name: TY_LIST,
            members: hashmap! [
                ObjectKey::Symbol(OP_ADD) => ObjectInstance::new(Func::new_func2(concat)),
                "length".into() => ObjectInstance::new(Func::new_func1(length)),
                "is_empty".into() => ObjectInstance::new(Func::new_func1(is_empty)),
            ],
        }
    }
}

impl Type for TypeList {
    fn name(&self) -> &Symbol {
        &self.name
    }

    fn impl_names(&self) -> &[&Symbol] {
        &[]
    }

    fn members(&self) -> &HashMap<ObjectKey, ObjectInstance> {
        &self.members
    }
}

#[derive(Debug, Clone)]
pub struct List(Rc<[ObjectInstance]>);

impl ObjectBase for List {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }

    fn get_instance_member(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        if let ObjectKey::Index(index) = key {
            let index = if *index >= 0 { *index } else { (self.len() as i64) + *index };

            if (index as usize) >= self.len() || index < 0 {
                Err(RuntimeError::IndexOutOfBounds(self.len(), index))
            } else {
                Ok(self[index as usize].clone())
            }
        } else {
            Err(RuntimeError::MissingField(key.clone()))
        }
    }
}

impl Display for List {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Deref for List {
    type Target = [ObjectInstance];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl From<Vec<ObjectInstance>> for List {
    fn from(value: Vec<ObjectInstance>) -> Self {
        Self(value.into())
    }
}

fn concat(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs
        .value::<List>()
        .ok_or_else(|| RuntimeError::InvalidType(TY_LIST, lhs.reflect_type().name().clone()))?;
    let output: List = if let Some(list) = rhs.value::<List>() {
        lhs.iter().chain(list.iter()).cloned().collect::<Vec<_>>().into()
    } else {
        lhs.iter().chain(iter::once(&rhs)).cloned().collect::<Vec<_>>().into()
    };

    Ok(ObjectInstance::new(output))
}

fn length(this: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let this = this.try_value::<List>(&TY_LIST)?;

    Ok(ObjectInstance::new(this.len() as i64))
}

fn is_empty(this: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let this = this.try_value::<List>(&TY_LIST)?;

    Ok(ObjectInstance::new(this.is_empty() as bool))
}
