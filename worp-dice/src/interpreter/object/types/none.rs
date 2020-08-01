use crate::interpreter::{
    object::ObjectBase,
    symbol::{common::types::TY_NONE, Symbol},
};

impl ObjectBase for () {
    fn type_name(&self) -> Symbol {
        TY_NONE
    }
}
