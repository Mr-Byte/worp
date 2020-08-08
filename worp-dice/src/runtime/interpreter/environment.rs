use crate::runtime::{
    core::{symbol::Symbol, Type, Value},
    error::RuntimeError,
};
use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

#[derive(Default, Debug)]
pub struct Environment {
    parent: Option<Rc<Environment>>,
    variables: RefCell<HashMap<Symbol, Value>>,
    known_types: RefCell<HashMap<Symbol, Rc<dyn Type>>>,
}

fn boxed<T: Error + 'static>(error: T) -> Box<dyn Error> {
    Box::new(error)
}

impl Environment {
    pub fn new(parent: Option<Rc<Environment>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
            known_types: Default::default(),
        }
    }

    pub fn variable(&self, name: &Symbol) -> Result<Value, RuntimeError> {
        if let Some(variable) = self.variables.try_borrow().map_err(boxed)?.get(name) {
            Ok(variable.clone())
        } else if let Some(variable) = self.parent.as_ref().map(|parent| parent.variable(name)).transpose()? {
            Ok(variable)
        } else {
            Err(RuntimeError::VariableNotFound(name.clone()))
        }
    }

    pub fn add_variable(&self, name: Symbol, value: Value) -> Result<(), RuntimeError> {
        self.variables.try_borrow_mut().map_err(boxed)?.insert(name, value);
        Ok(())
    }

    pub fn known_type(&self, name: &Symbol) -> Result<Option<Rc<dyn Type>>, RuntimeError> {
        if let Some(known_type) = self.known_types.try_borrow().map_err(boxed)?.get(name) {
            Ok(Some(known_type.clone()))
        } else if let Some(known_type) = self.parent.as_ref().map(|parent| parent.known_type(name)) {
            known_type
        } else {
            Ok(None)
        }
    }

    pub fn add_known_type(&self, known_type: Rc<dyn Type>) -> Result<(), RuntimeError> {
        self.known_types
            .try_borrow_mut()
            .map_err(boxed)?
            .insert(known_type.name().clone(), known_type);
        Ok(())
    }
}
