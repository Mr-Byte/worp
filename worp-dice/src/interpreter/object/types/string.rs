use crate::interpreter::{
    object::ObjectBase,
    symbol::{common::types::TY_STRING, Symbol},
};

impl ObjectBase for String {
    fn type_name(&self) -> Symbol {
        TY_STRING
    }
}
