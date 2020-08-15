use crate::runtime::{
    core::{Type, TypeInstanceBase, Value},
    error::RuntimeError,
};
use std::{fmt::Display, ops::Deref, rc::Rc};

decl_type! {
    type TypeString = "String";

    fn op_add(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<DiceString>(&TypeString::NAME)?;
        let result: DiceString = format!("{}{}", lhs, &*rhs).into();

        Ok(Value::new(result))
    }

    fn length(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<DiceString>(&TypeString::NAME)?;

        Ok(Value::new(this.len() as i64))
    }

    fn is_empty(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<DiceString>(&TypeString::NAME)?;

        Ok(Value::new(this.is_empty() as bool))
    }
}

#[derive(Debug, Clone)]
pub struct DiceString(Rc<str>);

impl TypeInstanceBase for DiceString {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TypeString::instance()
    }
}

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
