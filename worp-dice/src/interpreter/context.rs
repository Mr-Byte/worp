use super::{
    error::RuntimeError,
    object::{AnonymouseObject, ObjectKey, ObjectRef},
    symbol::{
        common::{
            operators::{OP_ADD, OP_AND, OP_DIV, OP_EQ, OP_GT, OP_GTE, OP_LT, OP_LTE, OP_MUL, OP_NE, OP_NEG, OP_NOT, OP_OR, OP_REM, OP_SUB},
            types::{TY_BOOL, TY_NONE},
        },
        Symbol,
    },
};
use crate::expression::{BinaryOperator, Expression, Literal, UnaryOperator};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct ExecutionContext {
    inner: Rc<Environment>,
}

#[derive(Default, Debug)]
struct Environment {
    parent: Option<Rc<Environment>>,
    variables: RefCell<HashMap<Symbol, ObjectRef>>,
}

impl Environment {
    fn variable(&self, name: &Symbol) -> Result<ObjectRef, RuntimeError> {
        if let Some(variable) = self.variables.borrow().get(name) {
            Ok(variable.clone())
        } else if let Some(variable) = self.parent.as_ref().map(|parent| parent.variable(name)).transpose()? {
            Ok(variable.clone())
        } else {
            Err(RuntimeError::VariableNotFound(name.clone()))
        }
    }
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }
}

impl ExecutionContext {
    pub fn eval_expression(&self, input: &str) -> Result<ObjectRef, RuntimeError> {
        let expr: Expression = input.parse()?;
        eval_expression(expr, &self.inner)
    }

    pub fn scoped(&self) -> ExecutionContext {
        ExecutionContext {
            inner: Rc::new(Environment {
                parent: Some(self.inner.clone()),
                variables: RefCell::new(HashMap::new()),
            }),
        }
    }

    pub fn variable(&self, name: &Symbol) -> Result<ObjectRef, RuntimeError> {
        self.inner.variable(name)
    }

    pub fn add_variable(&mut self, name: Symbol, instance: ObjectRef) {
        self.inner.variables.borrow_mut().insert(name, instance);
    }
}

fn eval_expression(expr: Expression, environment: &Environment) -> Result<ObjectRef, RuntimeError> {
    match expr {
        Expression::Literal(literal) => eval_literal(literal, environment),
        Expression::SafeAccess(expr, field) => eval_safe_field_access(*expr, field, environment),
        Expression::FieldAccess(expr, field) => eval_field_access(*expr, field, environment),
        Expression::FunctionCall(_, _) => Err(RuntimeError::Aborted), // TODO: Do I need method calls, too?
        Expression::Index(_, _) => Err(RuntimeError::Aborted),        //eval_index(expr, index, environment),
        Expression::Unary(op, expr) => eval_unary(op, *expr, environment),
        Expression::Binary(op, lhs, rhs) => eval_binary(op, *lhs, *rhs, environment),
        Expression::Range(_op, _lower, _upper) => Err(RuntimeError::Aborted),
        Expression::Conditional(condition, body, alternate) => eval_conditional(condition, body, alternate, environment),
    }
}

fn eval_conditional(
    condition: Box<Expression>,
    body: Box<Expression>,
    alternate: Option<Box<Expression>>,
    environment: &Environment,
) -> Result<ObjectRef, RuntimeError> {
    let condition_result = eval_expression(*condition, environment)?;
    let condition = *condition_result
        .value::<bool>()
        .ok_or_else(|| RuntimeError::InvalidType(TY_BOOL, condition_result.instance_type_data().type_tag().clone()))?;

    if condition {
        eval_expression(*body, environment)
    } else {
        if let Some(alternate) = alternate {
            eval_expression(*alternate, environment)
        } else {
            Ok(ObjectRef::NONE)
        }
    }
}

fn eval_unary(op: UnaryOperator, expr: Expression, environment: &Environment) -> Result<ObjectRef, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;

    match op {
        UnaryOperator::Negate => object_ref.get(&ObjectKey::Symbol(OP_NEG))?.call(&[object_ref]),
        UnaryOperator::Not => object_ref.get(&ObjectKey::Symbol(OP_NOT))?.call(&[object_ref]),
    }
}

fn eval_binary(op: BinaryOperator, lhs: Expression, rhs: Expression, environment: &Environment) -> Result<ObjectRef, RuntimeError> {
    let lhs = eval_expression(lhs, environment)?;
    // TODO: Only evaluate this when needed.
    let rhs = eval_expression(rhs, environment)?;

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
        BinaryOperator::Coalesce => {
            if *lhs.instance_type_data().type_tag() != TY_NONE {
                Ok(lhs)
            } else {
                Ok(rhs)
            }
        }
        BinaryOperator::Discard => Ok(rhs),
    }
}

fn eval_safe_field_access(expr: Expression, field: Symbol, environment: &Environment) -> Result<ObjectRef, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;

    if *object_ref.instance_type_data().type_tag() != TY_NONE {
        object_ref.get(&ObjectKey::Symbol(field))
    } else {
        Ok(ObjectRef::NONE)
    }
}

fn eval_field_access(expr: Expression, field: Symbol, environment: &Environment) -> Result<ObjectRef, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;
    object_ref.get(&ObjectKey::Symbol(field))
}

fn eval_literal(literal: Literal, environment: &Environment) -> Result<ObjectRef, RuntimeError> {
    match literal {
        Literal::Identifier(ref symbol) => environment.variable(symbol),
        Literal::None => Ok(ObjectRef::NONE),
        Literal::Integer(int) => Ok(ObjectRef::new(int)),
        Literal::Float(float) => Ok(ObjectRef::new(float)),
        Literal::String(string) => Ok(ObjectRef::new(Into::<Rc<str>>::into(string))),
        Literal::Boolean(bool) => Ok(ObjectRef::new(bool)),
        Literal::List(list) => {
            let result = Vec::with_capacity(list.len());
            let result = list.iter().try_fold(result, |mut acc, value| {
                let value = eval_expression(value.clone(), environment)?;
                acc.push(value);
                Ok::<_, RuntimeError>(acc)
            })?;
            let result: Rc<[ObjectRef]> = result.into();

            Ok(ObjectRef::new(result))
        }
        Literal::Object(object) => {
            let result = HashMap::new();
            let result = object.iter().try_fold(result, |mut acc, (key, value)| {
                acc.insert(key.clone(), eval_expression(value.clone(), environment)?);

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
    fn test_negate() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("- -5")?;

        assert_eq!(5, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_not() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("!true")?;

        assert_eq!(false, *result.value::<bool>().unwrap());

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

    #[test]
    fn test_field_access() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"{ test: 5 + 5 }.test"#)?;
        assert_eq!(10, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_safe_field_access() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"(none)?.test"#)?;
        assert_eq!((), *result.value::<()>().unwrap());

        Ok(())
    }

    #[test]
    fn test_nested_safe_field_access() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"{ test: none }.test?.xy"#)?;
        assert_eq!((), *result.value::<()>().unwrap());

        Ok(())
    }

    #[test]
    fn test_coalesce() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"{ test: none }.test?.xy ?? 10"#)?;
        assert_eq!(10, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_variable() -> Result<(), RuntimeError> {
        let mut context = ExecutionContext::new();
        context.add_variable(Symbol::new("test"), ObjectRef::new(5));
        let result = context.eval_expression(r#"test + 5"#)?;

        assert_eq!(10, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_variable_from_parent_scope() -> Result<(), RuntimeError> {
        let mut context = ExecutionContext::new();
        context.add_variable(Symbol::new("test"), ObjectRef::new(5));
        let result = context.scoped().eval_expression(r#"test + 5"#)?;

        assert_eq!(10, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_conditional() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"if 5 == 5 { 10 } else { 12 }"#)?;

        assert_eq!(10, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_conditional_alternate() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"if 5 == 6 { 10 } else { 12 }"#)?;

        assert_eq!(12, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_conditional_no_alternate() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"if 5 == 6 { 10 }"#)?;

        assert_eq!((), *result.value::<()>().unwrap());

        Ok(())
    }
}
