use crate::interpreter::{
    object::{reflection::TypeData, ObjectBase, ObjectKey, ObjectRef},
    symbol::common::types::TY_LIST,
};

impl ObjectBase for Vec<ObjectRef> {
    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_LIST, Vec::new())
    }
}
