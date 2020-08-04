use super::{error::RuntimeError, evaluator::eval, object::ObjectRef, symbol::Symbol};
use crate::expression::Expression;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct ExecutionContext {
    inner: Rc<Environment>,
}

#[derive(Default, Debug)]
pub struct Environment {
    parent: Option<Rc<Environment>>,
    variables: RefCell<HashMap<Symbol, ObjectRef>>,
}

impl Environment {
    pub fn variable(&self, name: &Symbol) -> Result<ObjectRef, RuntimeError> {
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

    pub fn variable(&self, name: &Symbol) -> Result<ObjectRef, RuntimeError> {
        self.inner.variable(name)
    }

    pub fn add_variable(&mut self, name: Symbol, instance: ObjectRef) {
        self.inner.variables.borrow_mut().insert(name, instance);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::interpreter::{object::ObjectKey, symbol::Symbol};

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
        let result = context.eval_expression(r#"none?.test"#)?;
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
    fn test_index_access() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"{ test: 5 + 5 }["test"]"#)?;
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
    fn test_conditional_multiple_alternate() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"if 5 == 6 { 10 } else if 5 == 5 { 42 } else { 12 }"#)?;

        assert_eq!(42, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_conditional_no_alternate() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"if 5 == 6 { 10 }"#)?;

        assert_eq!((), *result.value::<()>().unwrap());

        Ok(())
    }

    #[test]
    fn test_conditional_gte_no_alternate() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"if 5 >= 6 { 10 }"#)?;

        assert_eq!((), *result.value::<()>().unwrap());

        Ok(())
    }

    #[test]
    fn test_discard_expression_seps() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("5 + 5 ; none")?;

        assert_eq!((), *result.value::<()>().unwrap());

        Ok(())
    }

    #[test]
    fn test_method_call() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("5.to_string()")?;
        let actual = result.value::<Rc<str>>().unwrap().as_ref();

        assert_eq!("5", actual);

        Ok(())
    }

    #[test]
    fn test_method_call_with_index() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r##"5["#op_add"](5)"##)?;
        let actual = result.value::<i64>().unwrap();

        assert_eq!(10, *actual);

        Ok(())
    }

    #[test]
    fn test_chained_method_call() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r##"5["#op_add"](5).to_string()"##)?;
        let actual = result.value::<Rc<str>>().unwrap().as_ref();

        assert_eq!("10", actual);

        Ok(())
    }
}
