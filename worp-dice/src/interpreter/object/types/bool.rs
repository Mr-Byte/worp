use crate::interpreter::{
    object::{reflection::TypeData, ObjectBase, ObjectKey},
    symbol::common::types::TY_BOOL,
};

impl ObjectBase for bool {
    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_BOOL, Vec::new())
    }
}
