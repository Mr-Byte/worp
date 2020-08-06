use super::{key::ObjectKey, ObjectInstance};
use crate::runtime::{error::RuntimeError, symbol::Symbol};
use std::collections::HashMap;

pub trait Type {
    fn construct(&self) -> Result<ObjectInstance, RuntimeError> {
        Err(RuntimeError::NoConstructor(self.name().clone()))
    }

    fn name(&self) -> &Symbol;
    fn impl_names(&self) -> &[&Symbol];
    fn members(&self) -> &HashMap<ObjectKey, ObjectInstance>;

    // TODO: Add methods to apply a trait and new instance members to a type.
}
