use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};

decl_type! {
    impl TypeBool for bool as "Bool";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            match_type! { value,
                as_bool: bool => Ok(Value::Bool(*as_bool)),
                as_string: String => Ok(Value::Bool(as_string.parse::<bool>()?)),
                _ => Err(RuntimeError::InvalidType(TypeBool::NAME, value.instance_type().name().clone()))
            }
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }
}

impl TypeInstance for bool {}
