use crate::runtime::{
    core::{Type, TypeInstanceBase, Value},
    error::RuntimeError,
};
use std::{collections::HashMap, fmt::Display, rc::Rc};

decl_type! {
    type TypeNone = "None";

    fn op_eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        lhs.try_value::<None>(&TypeNone::NAME)?;
        let rhs = rhs.value::<None>();

        Ok(Value::new(rhs.is_some()))
    }

    fn op_ne(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        lhs.try_value::<None>(&TypeNone::NAME)?;
        let rhs = rhs.value::<None>();

        Ok(Value::new(rhs.is_none()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct None;

impl TypeInstanceBase for None {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TypeNone::instance()
    }
}

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}
