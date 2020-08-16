use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use std::fmt::Display;

decl_type! {
    impl TypeNone for None as "None";

    fn op_eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        lhs.try_value::<None>()?;
        let rhs = rhs.value::<None>();

        Ok(Value::new(rhs.is_some()))
    }

    fn op_ne(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        lhs.try_value::<None>()?;
        let rhs = rhs.value::<None>();

        Ok(Value::new(rhs.is_none()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct None;

impl TypeInstance for None {}

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}
