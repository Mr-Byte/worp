use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};

decl_type! {
    impl TypeInt for i64 as "Int";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            match_type! { value,
                as_int: i64 => Ok(Value::Int(*as_int)),
                as_float: f64 => Ok(Value::Int(*as_float as i64)),
                as_string: String => Ok(Value::Int(as_string.parse::<i64>()?)),
                _ => Err(RuntimeError::InvalidType(TypeInt::NAME, value.instance_type().name().clone()))
            }
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }
}

impl TypeInstance for i64 {}
