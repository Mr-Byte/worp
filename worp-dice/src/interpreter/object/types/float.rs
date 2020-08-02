use crate::interpreter::{
    object::{reflection::TypeData, ObjectBase, ObjectKey},
    symbol::common::types::TY_FLOAT,
};

impl ObjectBase for f64 {
    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_FLOAT, Vec::new())
    }
}
