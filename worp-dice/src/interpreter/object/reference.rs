use super::{types::func::Func, Object};
use std::{ops::Deref, rc::Rc};

#[derive(Clone, Debug)]
enum ObjectVariant {
    None(()),
    Bool(bool),
    Int(i64),
    Float(f64),
    Function(Func),
    List(Rc<[ObjectRef]>),
    String(Rc<str>),
    Object(Rc<dyn Object>),
}

#[derive(Clone, Debug)]
pub struct ObjectRef(ObjectVariant);

impl ObjectRef {
    pub const NONE: Self = ObjectRef(ObjectVariant::None(()));

    pub fn new_list(value: impl Into<Rc<[ObjectRef]>>) -> Self {
        Self(ObjectVariant::List(value.into()))
    }

    pub fn new_string(value: impl Into<Rc<str>>) -> Self {
        Self(ObjectVariant::String(value.into()))
    }

    pub fn new(value: impl Object + 'static) -> Self {
        let value_ref = &value as &dyn Object;
        let variant = if let Some(_) = value_ref.value::<()>() {
            ObjectVariant::None(())
        } else if let Some(value) = value_ref.value::<bool>() {
            ObjectVariant::Bool(*value)
        } else if let Some(value) = value_ref.value::<i64>() {
            ObjectVariant::Int(*value)
        } else if let Some(value) = value_ref.value::<f64>() {
            ObjectVariant::Float(*value)
        } else if let Some(value) = value_ref.value::<Func>() {
            ObjectVariant::Function(value.clone())
        } else if let Some(value) = value_ref.value::<Rc<[ObjectRef]>>() {
            ObjectVariant::List(value.clone())
        } else if let Some(value) = value_ref.value::<Rc<str>>() {
            ObjectVariant::String(value.clone())
        } else {
            ObjectVariant::Object(Rc::new(value))
        };

        Self(variant)
    }
}

impl Deref for ObjectRef {
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
