use crate::interpreter::{
    error::RuntimeError,
    object::{operator::coalesce, reflection::TypeData, types::func::Func, ObjectBase, ObjectKey, ObjectRef},
    symbol::common::{operators::OP_COALESCE, types::TY_OBJECT},
};
use maplit::hashmap;
use std::collections::HashMap;

thread_local! {
    static OPERATIONS: HashMap<ObjectKey, ObjectRef> = hashmap! [
        ObjectKey::Symbol(OP_COALESCE) => ObjectRef::new(Func::from_raw_func2(coalesce))
    ];

    static TYPE_DATA: TypeData = TypeData::new(TY_OBJECT, Vec::new());
}

#[derive(Debug)]
pub struct AnonymouseObject(HashMap<ObjectKey, ObjectRef>);

impl AnonymouseObject {
    pub fn new(data: HashMap<ObjectKey, ObjectRef>) -> Self {
        Self(data)
    }
}

impl ObjectBase for AnonymouseObject {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        self.0
            .get(key)
            .cloned()
            .or_else(|| OPERATIONS.with(|ops_table| ops_table.get(key).cloned()))
            .ok_or_else(|| RuntimeError::MissingField(key.clone()))
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        self.0
            .clone()
            .into_iter()
            .map(|(key, value)| (key, value.instance_type_data().clone()))
            .collect::<Vec<_>>()
    }

    fn type_data() -> TypeData {
        TYPE_DATA.with(Clone::clone)
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data().clone()
    }

    fn to_string(&self) -> String {
        let fields = self
            .0
            .iter()
            .map(|(key, value)| format!("{}: {}", key.to_string(), value.to_string()))
            .collect::<Vec<_>>()
            .join(", ");

        format!("{{ {} }}", fields)
    }
}
