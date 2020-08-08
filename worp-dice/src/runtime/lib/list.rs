use super::func::Func;
use crate::runtime::{
    core::{key::ValueKey, reflection::Type, value::Value, TypeInstanceBase},
    error::RuntimeError,
    symbol::{common::lib::TY_LIST, common::operators::OP_ADD, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, fmt::Display, iter, ops::Deref, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeList> = Default::default();
}

#[derive(Debug)]
struct TypeList {
    name: Symbol,
    members: HashMap<ValueKey, Value>,
}

impl Default for TypeList {
    fn default() -> Self {
        Self {
            name: TY_LIST,
            members: hashmap! [
                ValueKey::Symbol(OP_ADD) => Value::new(Func::new_func2(concat)),
                "length".into() => Value::new(Func::new_func1(length)),
                "is_empty".into() => Value::new(Func::new_func1(is_empty)),
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

    fn members(&self) -> &HashMap<ValueKey, Value> {
        &self.members
    }
}

#[derive(Debug, Clone)]
pub struct List(Rc<[Value]>);

impl TypeInstanceBase for List {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }

    fn get_instance_member(&self, key: &ValueKey) -> Result<Value, RuntimeError> {
        if let ValueKey::Index(index) = key {
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
    type Target = [Value];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl From<Vec<Value>> for List {
    fn from(value: Vec<Value>) -> Self {
        Self(value.into())
    }
}

fn concat(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs
        .value::<List>()
        .ok_or_else(|| RuntimeError::InvalidType(TY_LIST, lhs.reflect_type().name().clone()))?;
    let output: List = if let Some(list) = rhs.value::<List>() {
        lhs.iter().chain(list.iter()).cloned().collect::<Vec<_>>().into()
    } else {
        lhs.iter().chain(iter::once(&rhs)).cloned().collect::<Vec<_>>().into()
    };

    Ok(Value::new(output))
}

fn length(this: Value) -> Result<Value, RuntimeError> {
    let this = this.try_value::<List>(&TY_LIST)?;

    Ok(Value::new(this.len() as i64))
}

fn is_empty(this: Value) -> Result<Value, RuntimeError> {
    let this = this.try_value::<List>(&TY_LIST)?;

    Ok(Value::new(this.is_empty() as bool))
}
