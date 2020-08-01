use crate::interpreter::{
    object::ObjectBase,
    symbol::{common::types::TY_BOOL, Symbol},
};

impl ObjectBase for bool {
    fn type_name(&self) -> Symbol {
        TY_BOOL
    }
}
