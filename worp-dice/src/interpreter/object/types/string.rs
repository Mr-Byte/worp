use crate::interpreter::{
    object::{self, ObjectBase},
    symbol::common::types::TY_STRING,
};
use object::{reflection::TypeData, ObjectKey};
use std::rc::Rc;

impl ObjectBase for Rc<str> {
    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data() -> TypeData {
        TypeData::new(TY_STRING, Vec::new())
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data().clone()
    }
}
