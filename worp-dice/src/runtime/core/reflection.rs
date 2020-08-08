use super::{Value, ValueKey};
use crate::runtime::{core::Symbol, error::RuntimeError};
use std::{collections::HashMap, fmt::Debug};

pub trait Type: Debug {
    fn construct(&self, _args: &[Value]) -> Result<Value, RuntimeError> {
        Err(RuntimeError::NoConstructor(self.name().clone()))
    }

    fn name(&self) -> &Symbol;
    fn impl_names(&self) -> &[&Symbol];
    fn members(&self) -> &HashMap<ValueKey, Value>;

    // TODO: Add methods to apply a trait and new instance members to a type.
}
