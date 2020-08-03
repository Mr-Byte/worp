use crate::interpreter::{
    error::RuntimeError,
    object::{operator::coalesce, reflection::TypeData, types::func::Func, ObjectBase, ObjectKey, ObjectRef},
    symbol::common::{operators::OP_COALESCE, types::TY_NONE},
};
use maplit::hashmap;
use std::collections::HashMap;

thread_local! {
    static OPERATIONS: HashMap<ObjectKey, ObjectRef> = hashmap! [
        ObjectKey::Symbol(OP_COALESCE) => ObjectRef::new(Func::from_raw_func2(coalesce))
    ];

    static TYPE_DATA: TypeData = TypeData::new(TY_NONE, Vec::new());
}

impl ObjectBase for () {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        OPERATIONS.with(|ops_table| ops_table.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone())))
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        OPERATIONS.with(|ops| {
            ops.clone()
                .into_iter()
                .map(|(key, value)| (key, value.instance_type_data().clone()))
                .collect::<Vec<_>>()
        })
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
