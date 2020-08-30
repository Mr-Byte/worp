use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};

decl_type! {
    impl TypeFloat for f64 as "Float";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            match_type! { value,
                as_float: f64 => Ok(Value::Float(*as_float)),
                as_int: i64 => Ok(Value::Float(*as_int as f64)),
                as_string: String => Ok(Value::Float(as_string.parse::<f64>()?)),
                _ => Err(RuntimeError::InvalidType(TypeFloat::NAME, value.instance_type().name().clone()))
            }
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }
}

impl TypeInstance for f64 {}
