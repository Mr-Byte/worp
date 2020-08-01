use crate::interpreter::{
    object::ObjectBase,
    symbol::{common::types::TY_FLOAT, Symbol},
};

impl ObjectBase for f64 {
    fn type_name(&self) -> Symbol {
        TY_FLOAT
    }
}
