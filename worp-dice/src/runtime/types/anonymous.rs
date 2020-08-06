use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, key::ObjectKey, reflection::Type, ObjectBase},
    symbol::{common::types::TY_OBJECT, Symbol},
};
use std::{collections::HashMap, fmt::Display, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeAnonymousObject> = Default::default();
}

#[derive(Debug)]
struct TypeAnonymousObject {
    name: Symbol,
    members: HashMap<ObjectKey, ObjectInstance>,
}

impl Default for TypeAnonymousObject {
    fn default() -> Self {
        Self {
            name: TY_OBJECT,
            members: HashMap::new(),
        }
    }
}

impl Type for TypeAnonymousObject {
    fn name(&self) -> &Symbol {
        &self.name
    }

    fn impl_names(&self) -> &[&Symbol] {
        &[]
    }

    fn members(&self) -> &HashMap<ObjectKey, ObjectInstance> {
        &self.members
    }
}

#[derive(Debug)]
pub struct AnonymouseObject(HashMap<ObjectKey, ObjectInstance>);

impl AnonymouseObject {
    pub fn new(data: HashMap<ObjectKey, ObjectInstance>) -> Self {
        Self(data)
    }
}

impl ObjectBase for AnonymouseObject {
    fn get_instance_member(&self, key: &ObjectKey) -> Result<ObjectInstance, RuntimeError> {
        self.0.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone()))
    }

    fn reflect_type(&self) -> std::rc::Rc<dyn crate::runtime::object::reflection::Type> {
        TYPE.with(Clone::clone)
    }
}

impl Display for AnonymouseObject {
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
