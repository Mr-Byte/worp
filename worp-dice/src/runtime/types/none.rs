use crate::runtime::object::{reflection::Type, ObjectBase};
use std::{fmt::Display, rc::Rc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct None;

impl ObjectBase for None {
    fn reflect_type(&self) -> Rc<dyn Type> {
        todo!()
    }
}

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}
