use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use gc::{Finalize, Trace};
use std::{fmt::Display, ops::Deref, rc::Rc};

#[derive(Debug, Clone, Trace, Finalize)]
pub struct DiceString(#[unsafe_ignore_trace] Rc<str>);

impl TypeInstance for DiceString {}

impl Display for DiceString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for DiceString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl From<String> for DiceString {
    fn from(value: String) -> Self {
        DiceString(value.into())
    }
}

decl_type! {
    impl TypeString for DiceString as "String";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            let as_string: DiceString = value.to_string().into();

            Ok(Value::new(as_string))
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn op_add(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<DiceString>()?;
        let result: DiceString = format!("{}{}", lhs, &*rhs).into();

        Ok(Value::new(result))
    }

    fn length(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<DiceString>()?;

        Ok(Value::new(this.len() as i64))
    }

    fn is_empty(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<DiceString>()?;

        Ok(Value::new(this.is_empty() as bool))
    }
}
