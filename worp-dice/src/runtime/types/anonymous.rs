use crate::interpreter::{
    error::RuntimeError,
    object::{reflection::TypeData, ObjectBase, ObjectInstance, ObjectKey},
    symbol::common::types::TY_OBJECT,
};
use std::collections::HashMap;

thread_local! {
    static TYPE_DATA: TypeData = TypeData::new(TY_OBJECT, Vec::new());
}

#[derive(Debug)]
pub struct AnonymouseObject(HashMap<ObjectKey, ObjectInstance>);

impl AnonymouseObject {
    pub fn new(data: HashMap<ObjectKey, ObjectInstance>) -> Self {
        Self(data)
    }
}

impl ObjectBase for AnonymouseObject {
    fn get(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        self.0.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone()))
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        self.0
            .clone()
            .into_iter()
            .map(|(key, value)| (key, value.instance_type_data()))
            .collect::<Vec<_>>()
    }

    fn type_data() -> TypeData {
        TYPE_DATA.with(Clone::clone)
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data()
    }

    fn format_value(&self) -> String {
        let fields = self
            .0
            .iter()
            .map(|(key, value)| format!("{}: {}", key.to_string(), value.format_value()))
            .collect::<Vec<_>>()
            .join(", ");

        format!("{{ {} }}", fields)
    }
}
