use crate::runtime::{
    core::{TypeInstance, Value, ValueKey},
    error::RuntimeError,
};
use gc::{Finalize, Gc, Trace};
use std::{fmt::Display, iter, ops::Deref};

#[derive(Debug, Clone, Trace, Finalize)]
pub struct List(Gc<Vec<Value>>);

impl TypeInstance for List {
    fn get_instance_member(&self, key: &ValueKey) -> Result<Value, RuntimeError> {
        if let ValueKey::Index(index) = key {
            let index = if *index >= 0 { *index } else { (self.len() as i64) + *index };

            if (index as usize) >= self.len() || index < 0 {
                Err(RuntimeError::IndexOutOfBounds(self.len(), index))
            } else {
                Ok(self[index as usize].clone())
            }
        } else {
            Err(RuntimeError::MissingField(key.clone()))
        }
    }
}

impl Display for List {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self.0.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ");

        write!(fmt, "[{}]", items)
    }
}

impl Deref for List {
    type Target = [Value];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl From<Vec<Value>> for List {
    fn from(value: Vec<Value>) -> Self {
        Self(Gc::new(value))
    }
}

decl_type! {
    impl TypeList for List as "List";

    fn op_add(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let lhs = lhs.try_value::<List>()?;
        let output: List = if let Some(list) = rhs.value::<List>() {
            lhs.iter().chain(list.iter()).cloned().collect::<Vec<_>>().into()
        } else {
            lhs.iter().chain(iter::once(&rhs)).cloned().collect::<Vec<_>>().into()
        };

        Ok(Value::new(output))
    }

    fn length(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<List>()?;

        Ok(Value::new(this.len() as i64))
    }

    fn is_empty(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<List>()?;

        Ok(Value::new(this.is_empty() as bool))
    }
}
