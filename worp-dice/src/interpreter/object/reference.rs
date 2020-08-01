use super::{Object, ObjectBase};
use std::{ops::Deref, rc::Rc};

// TODO: Figure out how to optimizie representation of strings and lists.
#[derive(Clone, Debug)]
enum ObjectVariant {
    None(()),
    Bool(bool),
    Int(i64),
    Float(f64),
    List(Rc<Vec<ObjectRef>>),
    String(Rc<String>),
    Object(Rc<dyn Object>),
}

#[derive(Clone, Debug)]
pub struct ObjectRef(ObjectVariant);

impl ObjectRef {
    pub const NONE: Self = ObjectRef(ObjectVariant::None(()));

    pub const fn new_bool(value: bool) -> Self {
        Self(ObjectVariant::Bool(value))
    }

    pub const fn new_int(value: i64) -> Self {
        Self(ObjectVariant::Int(value))
    }

    pub const fn new_float(value: f64) -> Self {
        Self(ObjectVariant::Float(value))
    }

    pub fn new_list(value: Vec<ObjectRef>) -> Self {
        Self(ObjectVariant::List(Rc::new(value)))
    }

    pub fn new_string(value: String) -> Self {
        Self(ObjectVariant::String(Rc::new(value)))
    }

    pub fn new(value: impl ObjectBase + 'static) -> Self {
        Self(ObjectVariant::Object(Rc::new(value) as Rc<dyn Object>))
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
            ObjectVariant::List(ref obj) => &**obj,
            ObjectVariant::String(ref obj) => &**obj,
            ObjectVariant::Object(ref obj) => &**obj,
        }
    }
}
