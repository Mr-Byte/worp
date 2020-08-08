use crate::runtime::{
    core::{Type, TypeInstanceBase, Value, ValueKey},
    error::RuntimeError,
};
use std::{collections::HashMap, fmt::Display, rc::Rc};

decl_type! {
    type TypeObject = "Object";
}

#[derive(Debug)]
pub struct Object(HashMap<ValueKey, Value>);

impl Object {
    pub fn new(data: HashMap<ValueKey, Value>) -> Self {
        Self(data)
    }
}

impl TypeInstanceBase for Object {
    fn get_instance_member(&self, key: &ValueKey) -> Result<Value, RuntimeError> {
        self.0.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone()))
    }

    fn reflect_type(&self) -> Rc<dyn Type> {
        TypeObject::instance()
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = self
            .0
            .iter()
            .map(|(key, value)| format!("{}: {}", key.to_string(), value.to_string()))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{{ {} }}", fields)
    }
}
