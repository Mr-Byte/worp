use crate::runtime::core::{Type, TypeInstanceBase, Value};
use std::{fmt::Display, rc::Rc};

#[derive(Debug)]
enum RangeKind {
    Exclusive(std::ops::Range<i64>),
    Inclusive(std::ops::RangeInclusive<i64>),
}

#[derive(Debug)]
pub struct Range {
    kind: RangeKind,
}

impl Range {}

impl Display for Range {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TypeInstanceBase for Range {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TypeRange::instance()
    }
}

decl_type! {
    type TypeRange = "Range";

    constructor(&self, _args: &[Value]) {
        todo!();
    }
}
