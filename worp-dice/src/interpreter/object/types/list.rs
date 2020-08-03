use crate::interpreter::{
    object::{reflection::TypeData, ObjectBase, ObjectKey, ObjectRef},
    symbol::common::types::TY_LIST,
};
use std::rc::Rc;

impl ObjectBase for Rc<[ObjectRef]> {
    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data() -> TypeData {
        TypeData::new(TY_LIST, Vec::new())
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data().clone()
    }

    fn to_string(&self) -> String {
        let items = self.iter().map(|obj| obj.to_string()).collect::<Vec<_>>().join(", ");

        format!("[ {} ]", items)
    }
}
