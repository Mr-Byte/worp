use super::{environment::Environment, evaluator::eval};
use crate::{
    runtime::{core::value::Value, error::RuntimeError, symbol::Symbol},
    syntax::Expression,
};
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct ExecutionContext {
    inner: Rc<Environment>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn eval_expression(&self, input: &str) -> Result<Value, RuntimeError> {
        let expr: Expression = input.parse()?;
        eval(&expr, &self.inner)
    }

    pub fn scoped(&self) -> ExecutionContext {
        ExecutionContext {
            inner: Rc::new(Environment::new(Some(self.inner.clone()))),
        }
    }

    pub fn variable(&self, name: &Symbol) -> Result<Value, RuntimeError> {
        self.inner.variable(name)
    }

    pub fn add_variable(&mut self, name: Symbol, instance: Value) {
        self.inner.add_variable(name, instance);
    }
}
