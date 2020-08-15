use super::DiceString;
use crate::runtime::{
    core::{Type, TypeInstanceBase, Value},
    error::RuntimeError,
};
use std::rc::Rc;

decl_type! {
    type TypeBool = "Bool";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            match_type! { value,
                as_bool: bool => Ok(Value::new(*as_bool)),
                as_string: DiceString => Ok(Value::new(as_string.parse::<bool>()?)),
                _ => Err(RuntimeError::InvalidType(TypeBool::NAME, value.reflect_type().name().clone()))
            }
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn op_not(value: Value) -> Result<Value, RuntimeError> {
        let value = value.try_value::<bool>(&TypeBool::NAME)?;

        Ok(Value::new(!value))
    }

    fn op_eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<bool>(&TypeBool::NAME)?;
        let rhs = rhs.value::<bool>();
        let result = match rhs {
            Some(rhs) => lhs == rhs,
            None => false,
        };

        Ok(Value::new(result))
    }

    fn op_ne(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<bool>(&TypeBool::NAME)?;
        let rhs = rhs.value::<bool>();
        let result = match rhs {
            Some(rhs) => lhs != rhs,
            None => true,
        };

        Ok(Value::new(result))
    }

    fn op_gt(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<bool>(&TypeBool::NAME)?;
        let rhs = rhs.try_value::<bool>(&TypeBool::NAME)?;

        Ok(Value::new(lhs > rhs))
    }

    fn op_gte(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<bool>(&TypeBool::NAME)?;
        let rhs = rhs.try_value::<bool>(&TypeBool::NAME)?;

        Ok(Value::new(lhs >= rhs))
    }

    fn op_lt(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<bool>(&TypeBool::NAME)?;
        let rhs = rhs.try_value::<bool>(&TypeBool::NAME)?;

        Ok(Value::new(lhs < rhs))
    }

    fn op_lte(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<bool>(&TypeBool::NAME)?;
        let rhs = rhs.try_value::<bool>(&TypeBool::NAME)?;

        Ok(Value::new(lhs <= rhs))
    }
}

impl TypeInstanceBase for bool {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TypeBool::instance()
    }
}
