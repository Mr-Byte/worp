pub mod context;
mod evaluator;

#[cfg(test)]
mod test {
    use super::*;
    use crate::runtime::{
        error::RuntimeError,
        object::{instance::ObjectInstance, key::ObjectKey},
        symbol::Symbol,
        types::{list::List, none, string::RcString},
    };
    use context::ExecutionContext;

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
    fn test_equality_with_none() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();

        let result = context.eval_expression("10 == none")?;
        assert_eq!(false, *result.value::<bool>().unwrap());

        let result = context.eval_expression("none == 10")?;
        assert_eq!(false, *result.value::<bool>().unwrap());

        let result = context.eval_expression("10 != none")?;
        assert_eq!(true, *result.value::<bool>().unwrap());

        let result = context.eval_expression("none != 10")?;
        assert_eq!(true, *result.value::<bool>().unwrap());

        let result = context.eval_expression("none == none")?;
        assert_eq!(true, *result.value::<bool>().unwrap());

        let result = context.eval_expression("none != none")?;
        assert_eq!(false, *result.value::<bool>().unwrap());

        Ok(())
    }

    #[test]
    fn test_none() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("none")?;

        assert_eq!(none::None, *result.value::<none::None>().unwrap());

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
        assert_eq!(none::None, *result.value::<none::None>().unwrap());

        Ok(())
    }

    #[test]
    fn test_nested_safe_field_access() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"{ test: none }.test?.xy"#)?;
        assert_eq!(none::None, *result.value::<none::None>().unwrap());

        Ok(())
    }

    #[test]
    fn test_coalesce() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"none ?? 10"#)?;
        assert_eq!(10, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_complex_coalesce() -> Result<(), RuntimeError> {
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
        context.add_variable(Symbol::new("test"), ObjectInstance::new(5));
        let result = context.eval_expression(r#"test + 5"#)?;

        assert_eq!(10, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_variable_from_parent_scope() -> Result<(), RuntimeError> {
        let mut context = ExecutionContext::new();
        context.add_variable(Symbol::new("test"), ObjectInstance::new(5));
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

        assert_eq!(none::None, *result.value::<none::None>().unwrap());

        Ok(())
    }

    #[test]
    fn test_conditional_gte_no_alternate() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"if 5 >= 6 { 10 }"#)?;

        assert_eq!(none::None, *result.value::<none::None>().unwrap());

        Ok(())
    }

    #[test]
    fn test_discard_expression_seps() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("5 + 5 ; none")?;

        assert_eq!(none::None, *result.value::<none::None>().unwrap());

        Ok(())
    }

    #[test]
    fn test_method_call() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression("5.to_string()")?;
        let actual = result.value::<RcString>().unwrap();

        assert_eq!("5", &**actual);

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
    fn test_method_call_with_invalid_index() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r##"5[5.0]"##);

        assert!(matches!(result, Err(RuntimeError::InvalidKeyType(_))));

        Ok(())
    }

    #[test]
    fn test_chained_method_call() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r##"5["#op_add"](5).to_string()"##)?;
        let actual = result.value::<RcString>().unwrap();

        assert_eq!("10", &**actual);

        Ok(())
    }

    #[test]
    fn test_string_concat() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r##""test" + "value""##)?;
        let actual = result.value::<RcString>().unwrap();

        assert_eq!("testvalue", &**actual);

        Ok(())
    }

    #[test]
    fn test_list_concat() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"[5] + [5, 5]"#)?;
        let actual = result.value::<List>().unwrap().as_ref();

        assert_eq!(3, actual.len());

        Ok(())
    }

    #[test]
    fn test_list_concat_with_value() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"[5] + 5"#)?;
        let actual = result.value::<List>().unwrap().as_ref();

        assert_eq!(2, actual.len());

        Ok(())
    }

    #[test]
    fn test_list_index() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"[5][0]"#)?;
        let actual = *result.value::<i64>().unwrap();

        assert_eq!(5, actual);

        Ok(())
    }

    #[test]
    fn test_list_negative_index() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"[5][-1]"#)?;
        let actual = *result.value::<i64>().unwrap();

        assert_eq!(5, actual);

        Ok(())
    }

    #[test]
    fn test_list_negative_index_out_of_bounds() -> Result<(), RuntimeError> {
        let context = ExecutionContext::new();
        let result = context.eval_expression(r#"[5][-2]"#);

        assert!(matches!(result, Err(RuntimeError::IndexOutOfBounds(1, -1))));

        Ok(())
    }
}
