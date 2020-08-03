use crate::interpreter::{
    error::RuntimeError,
    object::{reflection::TypeData, ObjectBase, ObjectKey, ObjectRef},
    symbol::common::types::TY_NONE,
};

thread_local! {
    static TYPE_DATA: TypeData = TypeData::new(TY_NONE, Vec::new());
}

impl ObjectBase for () {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        Err(RuntimeError::MissingField(key.clone()))
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        vec![]
    }

    fn type_data() -> TypeData {
        TypeData::new(TY_NONE, Vec::new())
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data().clone()
    }

    fn to_string(&self) -> String {
        String::from("none")
    }
}
