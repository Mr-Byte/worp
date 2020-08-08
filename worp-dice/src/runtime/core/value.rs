use super::TypeInstance;
use crate::runtime::lib::{func::Func, list::List, none, string::DiceString};
use std::{ops::Deref, rc::Rc};

#[derive(Clone, Debug)]
enum Variant {
    None(none::None),
    Bool(bool),
    Int(i64),
    Float(f64),
    Function(Func),
    List(List),
    String(DiceString),
    Object(Rc<dyn TypeInstance>),
}

#[derive(Clone, Debug)]
pub struct Value(Variant);

impl Value {
    pub const NONE: Self = Value(Variant::None(none::None));

    pub fn new<O>(value: O) -> Self
    where
        O: TypeInstance + 'static,
    {
        Self::new_object(value)
    }

    fn new_object(value: impl TypeInstance + 'static) -> Self {
        let value_ref = &value as &dyn TypeInstance;
        let variant = if value_ref.value::<none::None>().is_some() {
            Variant::None(none::None)
        } else if let Some(value) = value_ref.value::<bool>() {
            Variant::Bool(*value)
        } else if let Some(value) = value_ref.value::<i64>() {
            Variant::Int(*value)
        } else if let Some(value) = value_ref.value::<f64>() {
            Variant::Float(*value)
        } else if let Some(value) = value_ref.value::<Func>() {
            Variant::Function(value.clone())
        } else if let Some(value) = value_ref.value::<List>() {
            Variant::List(value.clone())
        } else if let Some(value) = value_ref.value::<DiceString>() {
            Variant::String(value.clone())
        } else {
            Variant::Object(Rc::new(value))
        };

        Self(variant)
    }
}

impl Deref for Value {
    type Target = dyn TypeInstance;

    fn deref(&self) -> &Self::Target {
        match self.0 {
            Variant::None(ref obj) => &*obj,
            Variant::Bool(ref obj) => &*obj,
            Variant::Int(ref obj) => &*obj,
            Variant::Float(ref obj) => &*obj,
            Variant::List(ref obj) => obj,
            Variant::String(ref obj) => obj,
            Variant::Function(ref obj) => obj,
            Variant::Object(ref obj) => &**obj,
        }
    }
}

impl<O> From<O> for Value
where
    O: TypeInstance + Into<O>,
{
    fn from(value: O) -> Self {
        Value::new(value)
    }
}
