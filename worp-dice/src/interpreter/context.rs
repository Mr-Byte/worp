use super::{
    error::RuntimeError,
    object::{AnonymouseObject, ObjectKey, ObjectRef},
    symbol::common::operators::{OP_ADD, OP_AND, OP_COALESCE, OP_DIV, OP_EQ, OP_GT, OP_GTE, OP_LT, OP_LTE, OP_MUL, OP_NE, OP_OR, OP_REM, OP_SUB},
};
use crate::expression::{BinaryOperator, Expression, Literal};
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
    pub fn eval_expression(&self, input: &str) -> Result<ObjectRef, RuntimeError> {
        let expr: Expression = input.parse()?;
        eval_expression(expr)
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

fn eval_expression(expression: Expression) -> Result<ObjectRef, RuntimeError> {
    match expression {
        Expression::Literal(literal) => eval_literal(literal),
        Expression::Symbol(_) => Err(RuntimeError::Aborted),
        Expression::SafeAccess(_) => Err(RuntimeError::Aborted),
        Expression::FieldAccess(_, _) => Err(RuntimeError::Aborted),
        Expression::FunctionCall(_, _) => Err(RuntimeError::Aborted),
        Expression::Index(_, _) => Err(RuntimeError::Aborted),
        Expression::Unary(_, _) => Err(RuntimeError::Aborted),
        Expression::Binary(op, lhs, rhs) => eval_bin_op(op, lhs, rhs),
        Expression::Range(_, _, _) => Err(RuntimeError::Aborted),
        Expression::Conditional(_, _, _) => Err(RuntimeError::Aborted),
    }
}

fn eval_bin_op(op: BinaryOperator, lhs: Box<Expression>, rhs: Box<Expression>) -> Result<ObjectRef, RuntimeError> {
    let lhs = eval_expression(*lhs)?;
    let rhs = eval_expression(*rhs)?;

    match op {
        BinaryOperator::DiceRoll => Err(RuntimeError::Aborted),
        BinaryOperator::Multiply => lhs.get(&ObjectKey::Symbol(OP_MUL))?.call(&[lhs, rhs]),
        BinaryOperator::Divide => lhs.get(&ObjectKey::Symbol(OP_DIV))?.call(&[lhs, rhs]),
        BinaryOperator::Remainder => lhs.get(&ObjectKey::Symbol(OP_REM))?.call(&[lhs, rhs]),
        BinaryOperator::Add => lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(&[lhs, rhs]),
        BinaryOperator::Subtract => lhs.get(&ObjectKey::Symbol(OP_SUB))?.call(&[lhs, rhs]),
        BinaryOperator::Equals => lhs.get(&ObjectKey::Symbol(OP_EQ))?.call(&[lhs, rhs]),
        BinaryOperator::NotEquals => lhs.get(&ObjectKey::Symbol(OP_NE))?.call(&[lhs, rhs]),
        BinaryOperator::GreaterThan => lhs.get(&ObjectKey::Symbol(OP_GT))?.call(&[lhs, rhs]),
        BinaryOperator::GreaterThanOrEquals => lhs.get(&ObjectKey::Symbol(OP_GTE))?.call(&[lhs, rhs]),
        BinaryOperator::LessThan => lhs.get(&ObjectKey::Symbol(OP_LT))?.call(&[lhs, rhs]),
        BinaryOperator::LessThanOrEquals => lhs.get(&ObjectKey::Symbol(OP_LTE))?.call(&[lhs, rhs]),
        BinaryOperator::LogicalAnd => lhs.get(&ObjectKey::Symbol(OP_AND))?.call(&[lhs, rhs]),
        BinaryOperator::LogicalOr => lhs.get(&ObjectKey::Symbol(OP_OR))?.call(&[lhs, rhs]),
        BinaryOperator::Coalesce => lhs.get(&ObjectKey::Symbol(OP_COALESCE))?.call(&[lhs, rhs]),
        BinaryOperator::Discard => Ok(rhs),
    }
}

fn eval_literal(literal: Literal) -> Result<ObjectRef, RuntimeError> {
    match literal {
        Literal::Identifier(_) => Err(RuntimeError::Aborted),
        Literal::None => Ok(ObjectRef::NONE),
        Literal::Integer(int) => Ok(ObjectRef::new(int)),
        Literal::Float(float) => Ok(ObjectRef::new(float)),
        Literal::String(string) => Ok(ObjectRef::new(Into::<Rc<str>>::into(string))),
        Literal::Boolean(bool) => Ok(ObjectRef::new(bool)),
        Literal::List(list) => {
            let result = Vec::with_capacity(list.len());
            let result = list.iter().try_fold(result, |mut acc, value| {
                let value = eval_expression(value.clone())?;
                acc.push(value);
                Ok::<_, RuntimeError>(acc)
            })?;
            let result: Rc<[ObjectRef]> = result.into();

            Ok(ObjectRef::new(result))
        }
        Literal::Object(object) => {
            let result = HashMap::new();
            let result = object.iter().try_fold(result, |mut acc, (key, value)| {
                acc.insert(key.clone(), eval_expression(value.clone())?);

                Ok::<_, RuntimeError>(acc)
            })?;

            Ok(ObjectRef::new(AnonymouseObject::new(result)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::interpreter::symbol::Symbol;

    #[test]
    fn test_multiplication() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("5 * 5 * 5")?;

        assert_eq!(125, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_addition() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("5 + 5 + 5")?;

        assert_eq!(15, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_equality() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("2 + 3 == 5")?;

        assert_eq!(true, *result.value::<bool>().unwrap());

        Ok(())
    }

    #[test]
    fn test_none() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("none")?;

        assert_eq!((), *result.value::<()>().unwrap());

        Ok(())
    }

    #[test]
    fn test_object() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"{ test: 5 + 5 }"#)?;
        let inner = result.get(&ObjectKey::Symbol(Symbol::new_static("test")))?;

        assert_eq!(10, *inner.value::<i64>().unwrap());

        Ok(())
    }
}
