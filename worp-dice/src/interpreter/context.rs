use super::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct ExecutionContext {
    inner: Rc<Environment>,
}

#[derive(Debug, Default)]
struct Environment {
    parent: Option<Rc<Environment>>,
    variables: RefCell<HashMap<String, Value>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }
}

impl ExecutionContext {
    pub fn eval(&self, _input: &str) -> Result<(), super::error::EvaluationError> {
        todo!()
    }

    pub fn scoped(&self) -> ExecutionContext {
        ExecutionContext {
            inner: Rc::new(Environment {
                parent: Some(self.inner.clone()),
                variables: RefCell::new(HashMap::new()),
            }),
        }
    }

    pub fn add_variable(&mut self, name: impl Into<String>, object: Value) {
        self.inner.variables.borrow_mut().insert(name.into(), object);
    }
}
