use super::object::ObjectRef;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct ExecutionContext {
    inner: Rc<Environment>,
}

#[derive(Default, Debug)]
struct Environment {
    parent: Option<Rc<Environment>>,
    variables: RefCell<HashMap<String, ObjectRef>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }
}

impl ExecutionContext {
    pub fn eval(&self, _input: &str) -> Result<(), super::error::RuntimeError> {
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

    pub fn add_variable(&mut self, name: impl Into<String>, instance: ObjectRef) {
        self.inner.variables.borrow_mut().insert(name.into(), instance);
    }
}
