use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};

impl TypeInstance for String {}

decl_type! {
    impl TypeString for String as "String";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            let as_string: String = value.to_string();

            Ok(Value::new(as_string))
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn op_add(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<String>()?;
        let result = format!("{}{}", lhs, &*rhs);

        Ok(Value::new(result))
    }

    fn length(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<String>()?;

        Ok(Value::new(this.len() as i64))
    }

    fn is_empty(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<String>()?;

        Ok(Value::new(this.is_empty() as bool))
    }
}
