use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use gc::{Finalize, Trace};
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Trace, Finalize)]
pub struct Unit;

impl TypeInstance for Unit {}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}

decl_type! {
    impl TypeUnit for Unit as "Unit";

    fn op_eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        lhs.try_value::<Unit>()?;
        let rhs = rhs.value::<Unit>();

        Ok(Value::Bool(rhs.is_some()))
    }

    fn op_neq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        lhs.try_value::<Unit>()?;
        let rhs = rhs.value::<Unit>();

        Ok(Value::Bool(rhs.is_none()))
    }
}
