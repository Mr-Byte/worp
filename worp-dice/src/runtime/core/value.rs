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
        let variant = match_type! { &value as &dyn TypeInstance,
            as_none: none::None => Variant::None(*as_none),
            as_bool: bool => Variant::Bool(*as_bool),
            as_int: i64 => Variant::Int(*as_int),
            as_float: f64 => Variant::Float(*as_float),
            as_func: Func => Variant::Function(as_func.clone()),
            as_list: List => Variant::List(as_list.clone()),
            as_string: DiceString => Variant::String(as_string.clone()),
            _ => Variant::Object(Rc::new(value))
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
