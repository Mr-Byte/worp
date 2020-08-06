use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, reflection::Type},
    symbol::Symbol,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default, Debug)]
pub struct Environment {
    parent: Option<Rc<Environment>>,
    variables: RefCell<HashMap<Symbol, ObjectInstance>>,
    types: RefCell<HashMap<Symbol, Rc<dyn Type>>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<Environment>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
            types: Default::default(),
        }
    }

    pub fn variable(&self, name: &Symbol) -> Result<ObjectInstance, RuntimeError> {
        if let Some(variable) = self.variables.borrow().get(name) {
            Ok(variable.clone())
        } else if let Some(variable) = self.parent.as_ref().map(|parent| parent.variable(name)).transpose()? {
            Ok(variable)
        } else {
            Err(RuntimeError::VariableNotFound(name.clone()))
        }
    }

    pub fn add_variable(&self, name: Symbol, value: ObjectInstance) {
        self.variables.borrow_mut().insert(name, value);
    }
}
