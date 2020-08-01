use crate::interpreter::{
    object::{ObjectBase, ObjectRef},
    symbol::{common::types::TY_LIST, Symbol},
};

impl ObjectBase for Vec<ObjectRef> {
    fn type_name(&self) -> Symbol {
        TY_LIST
    }
}
