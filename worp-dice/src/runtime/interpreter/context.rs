use super::evaluator::eval;
use crate::{
    runtime::{error::RuntimeError, object::instance::ObjectInstance, symbol::Symbol},
    syntax::Expression,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Default)]
pub struct ExecutionContext {
    inner: Rc<Environment>,
}

#[derive(Default, Debug)]
pub struct Environment {
    parent: Option<Rc<Environment>>,
    variables: RefCell<HashMap<Symbol, ObjectInstance>>,
}

impl Environment {
    pub fn variable(&self, name: &Symbol) -> Result<ObjectInstance, RuntimeError> {
        if let Some(variable) = self.variables.borrow().get(name) {
            Ok(variable.clone())
        } else if let Some(variable) = self.parent.as_ref().map(|parent| parent.variable(name)).transpose()? {
            Ok(variable)
        } else {
            Err(RuntimeError::VariableNotFound(name.clone()))
        }
    }
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn eval_expression(&self, input: &str) -> Result<ObjectInstance, RuntimeError> {
        let expr: Expression = input.parse()?;
        eval(&expr, &self.inner)
    }

    pub fn scoped(&self) -> ExecutionContext {
        ExecutionContext {
            inner: Rc::new(Environment {
                parent: Some(self.inner.clone()),
                variables: RefCell::new(HashMap::new()),
            }),
        }
    }

    pub fn variable(&self, name: &Symbol) -> Result<ObjectInstance, RuntimeError> {
        self.inner.variable(name)
    }

    pub fn add_variable(&mut self, name: Symbol, instance: ObjectInstance) {
        self.inner.variables.borrow_mut().insert(name, instance);
    }
}
