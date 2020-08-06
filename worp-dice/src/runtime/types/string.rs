use super::func::Func;
use crate::runtime::{
    core::{key::ValueKey, reflection::Type, value::Value, TypeInstanceBase},
    error::RuntimeError,
    symbol::{common::operators::OP_ADD, common::types::TY_STRING, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, fmt::Display, ops::Deref, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeString> = Default::default();
}

#[derive(Debug)]
struct TypeString {
    name: Symbol,
    members: HashMap<ValueKey, Value>,
}

impl Default for TypeString {
    fn default() -> Self {
        Self {
            name: TY_STRING,
            members: hashmap! [
                ValueKey::Symbol(OP_ADD) => Value::new(Func::new_func2(concat)),
                "length".into() => Value::new(Func::new_func1(length)),
                "is_empty".into() => Value::new(Func::new_func1(is_empty)),
            ],
        }
    }
}

impl Type for TypeString {
    fn name(&self) -> &Symbol {
        &self.name
    }

    fn impl_names(&self) -> &[&Symbol] {
        &[]
    }

    fn members(&self) -> &HashMap<ValueKey, Value> {
        &self.members
    }
}

#[derive(Debug, Clone)]
pub struct RcString(Rc<str>);

impl TypeInstanceBase for RcString {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

impl Display for RcString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for RcString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl From<String> for RcString {
    fn from(value: String) -> Self {
        RcString(value.into())
    }
}

fn concat(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<RcString>(&TY_STRING)?;
    let result: RcString = format!("{}{}", lhs, &*rhs).into();

    Ok(Value::new(result))
}

fn length(this: Value) -> Result<Value, RuntimeError> {
    let this = this.try_value::<RcString>(&TY_STRING)?;

    Ok(Value::new(this.len() as i64))
}

fn is_empty(this: Value) -> Result<Value, RuntimeError> {
    let this = this.try_value::<RcString>(&TY_STRING)?;

    Ok(Value::new(this.is_empty() as bool))
}
