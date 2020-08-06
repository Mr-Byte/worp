use super::func::Func;
use crate::runtime::{
    core::{key::ValueKey, reflection::Type, value::Value, TypeInstanceBase},
    error::RuntimeError,
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
    instance_members: HashMap<ValueKey, Value>,
}

impl Default for TypeNone {
    fn default() -> Self {
        Self {
            name: TY_NONE,
            instance_members: hashmap! [
                ValueKey::Symbol(OP_EQ) => Value::new(Func::new_func2(eq)),
                ValueKey::Symbol(OP_NE) => Value::new(Func::new_func2(ne)),
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

    fn members(&self) -> &HashMap<ValueKey, Value> {
        &self.instance_members
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct None;

impl TypeInstanceBase for None {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}

fn eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    lhs.try_value::<None>(&TY_NONE)?;
    let rhs = rhs.value::<None>();

    Ok(Value::new(rhs.is_some()))
}

fn ne(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    lhs.try_value::<None>(&TY_NONE)?;
    let rhs = rhs.value::<None>();

    Ok(Value::new(rhs.is_none()))
}
