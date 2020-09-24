use crate::runtime::core::Value;
use std::{fmt::Display, ops::Deref, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct List(Rc<Vec<Value>>);

impl Display for List {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .0
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(fmt, "[{}]", items)
    }
}

impl Deref for List {
    type Target = [Value];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl From<Vec<Value>> for List {
    fn from(value: Vec<Value>) -> Self {
        Self(Rc::new(value))
    }
}
