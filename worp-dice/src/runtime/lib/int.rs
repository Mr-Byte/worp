use super::DiceString;
use crate::runtime::{
    core::{Type, TypeInstanceBase, Value},
    error::RuntimeError,
};
use std::rc::Rc;

decl_type! {
    type TypeInt = "Int";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            match_type! { value,
                as_int: i64 => Ok(Value::new(*as_int)),
                as_float: f64 => Ok(Value::new(*as_float as i64)),
                as_string: DiceString => Ok(Value::new(as_string.parse::<i64>()?)),
                _ => Err(RuntimeError::InvalidType(TypeInt::NAME, value.reflect_type().name().clone()))
            }
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn op_neg(value: Value) -> Result<Value, RuntimeError> {
        let value = value.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(-value))
    }

    fn op_mul(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs * rhs))
    }

    fn op_div(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs / rhs))
    }

    fn op_rem(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs % rhs))
    }

    fn op_add(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs + rhs))
    }

    fn op_sub(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs - rhs))
    }

    fn op_eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.value::<i64>();
        let result = match rhs {
            Some(rhs) => lhs == rhs,
            None => false,
        };

        Ok(Value::new(result))
    }

    fn op_ne(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.value::<i64>();
        let result = match rhs {
            Some(rhs) => lhs != rhs,
            None => true,
        };

        Ok(Value::new(result))
    }

    fn op_gt(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs > rhs))
    }

    fn op_gte(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs >= rhs))
    }

    fn op_lt(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs < rhs))
    }

    fn op_lte(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<i64>(&TypeInt::NAME)?;
        let rhs = rhs.try_value::<i64>(&TypeInt::NAME)?;

        Ok(Value::new(lhs <= rhs))
    }
}

impl TypeInstanceBase for i64 {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TypeInt::instance()
    }
}
