use super::func::Func;
use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, key::ObjectKey, reflection::Type, ObjectBase},
    symbol::{common::operators::OP_ADD, common::types::TY_STRING, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, fmt::Display, ops::Deref, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeString> = Default::default();
}

struct TypeString {
    name: Symbol,
    members: HashMap<ObjectKey, ObjectInstance>,
}

impl Default for TypeString {
    fn default() -> Self {
        Self {
            name: TY_STRING,
            members: hashmap! [
                ObjectKey::Symbol(OP_ADD) => ObjectInstance::new(Func::new_func2(concat)),
                "length".into() => ObjectInstance::new(Func::new_func1(length)),
                "is_empty".into() => ObjectInstance::new(Func::new_func1(is_empty)),
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

    fn members(&self) -> &HashMap<ObjectKey, ObjectInstance> {
        &self.members
    }
}

#[derive(Debug, Clone)]
pub struct RcString(Rc<str>);

impl ObjectBase for RcString {
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

fn concat(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let lhs = lhs.try_value::<RcString>(&TY_STRING)?;
    let result: RcString = format!("{}{}", lhs, &*rhs).into();

    Ok(ObjectInstance::new(result))
}

fn length(this: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let this = this.try_value::<RcString>(&TY_STRING)?;

    Ok(ObjectInstance::new(this.len() as i64))
}

fn is_empty(this: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    let this = this.try_value::<RcString>(&TY_STRING)?;

    Ok(ObjectInstance::new(this.is_empty() as bool))
}
