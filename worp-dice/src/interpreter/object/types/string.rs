use crate::interpreter::{
    object::{self, ObjectBase},
    symbol::common::types::TY_STRING,
};
use object::{reflection::TypeData, ObjectKey};

impl ObjectBase for String {
    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_STRING, Vec::new())
    }
}
