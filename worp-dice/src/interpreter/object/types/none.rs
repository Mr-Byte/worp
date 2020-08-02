use crate::interpreter::{
    object::{reflection::TypeData, ObjectBase, ObjectKey},
    symbol::common::types::TY_NONE,
};

impl ObjectBase for () {
    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_NONE, Vec::new())
    }
}
