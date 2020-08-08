use super::{environment::Environment, evaluator::eval};
use crate::{
    runtime::{
        core::Value,
        error::RuntimeError,
        lib::{TypeFloat, TypeInt},
    },
    syntax::Expression,
};
use std::{ops::Deref, rc::Rc};

#[derive(Debug, Default)]
pub struct ExecutionContext {
    inner: Rc<Environment>,
}

impl ExecutionContext {
    pub fn try_new() -> Result<Self, RuntimeError> {
        let inner: Rc<Environment> = Default::default();
        inner.add_known_type(TypeInt::instance())?;
        inner.add_known_type(TypeFloat::instance())?;

        Ok(Self { inner })
    }

    pub fn eval_expression(&self, input: &str) -> Result<Value, RuntimeError> {
        let expr: Expression = input.parse()?;
        eval(&expr, self)
    }

    pub fn scoped(&self) -> ExecutionContext {
        ExecutionContext {
            inner: Rc::new(Environment::new(Some(self.inner.clone()))),
        }
    }
}

impl Deref for ExecutionContext {
    type Target = Environment;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
