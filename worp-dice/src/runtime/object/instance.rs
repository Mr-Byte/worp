use super::Object;
use crate::runtime::types::{func::Func, list::List, none, string::RcString};
use std::{ops::Deref, rc::Rc};

#[derive(Clone, Debug)]
enum ObjectVariant {
    None(none::None),
    Bool(bool),
    Int(i64),
    Float(f64),
    Function(Func),
    List(List),
    String(RcString),
    Object(Rc<dyn Object>),
}

#[derive(Clone, Debug)]
pub struct ObjectInstance(ObjectVariant);

impl ObjectInstance {
    pub const NONE: Self = ObjectInstance(ObjectVariant::None(none::None));

    pub fn new<O>(value: O) -> Self
    where
        O: Object + 'static,
    {
        Self::new_object(value)
    }

    fn new_object(value: impl Object + 'static) -> Self {
        let value_ref = &value as &dyn Object;
        let variant = if value_ref.value::<none::None>().is_some() {
            ObjectVariant::None(none::None)
        } else if let Some(value) = value_ref.value::<bool>() {
            ObjectVariant::Bool(*value)
        } else if let Some(value) = value_ref.value::<i64>() {
            ObjectVariant::Int(*value)
        } else if let Some(value) = value_ref.value::<f64>() {
            ObjectVariant::Float(*value)
        } else if let Some(value) = value_ref.value::<Func>() {
            ObjectVariant::Function(value.clone())
        } else if let Some(value) = value_ref.value::<List>() {
            ObjectVariant::List(value.clone())
        } else if let Some(value) = value_ref.value::<RcString>() {
            ObjectVariant::String(value.clone())
        } else {
            ObjectVariant::Object(Rc::new(value))
        };

        Self(variant)
    }
}

impl Deref for ObjectInstance {
    type Target = dyn Object;

    fn deref(&self) -> &Self::Target {
        match self.0 {
            ObjectVariant::None(ref obj) => &*obj,
            ObjectVariant::Bool(ref obj) => &*obj,
            ObjectVariant::Int(ref obj) => &*obj,
            ObjectVariant::Float(ref obj) => &*obj,
            ObjectVariant::List(ref obj) => obj,
            ObjectVariant::String(ref obj) => obj,
            ObjectVariant::Function(ref obj) => obj,
            ObjectVariant::Object(ref obj) => &**obj,
        }
    }
}

impl<O> From<O> for ObjectInstance
where
    O: Object + Into<O>,
{
    fn from(value: O) -> Self {
        ObjectInstance::new(value)
    }
}
