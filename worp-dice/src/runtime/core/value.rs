use super::{Symbol, TypeInstance};
use crate::runtime::{
    error::RuntimeError,
    lib::{self, DiceString, Func, List},
};
use gc::{Finalize, Gc, Trace};
use std::{fmt::Display, ops::Deref};

#[derive(Clone, Debug, Trace, Finalize)]
enum Variant {
    None(lib::None),
    Bool(bool),
    Int(i64),
    Float(f64),
    Function(Func),
    List(List),
    String(DiceString),
    Object(Gc<Box<dyn TypeInstance>>),
}

#[derive(Clone, Debug, Trace, Finalize)]
pub struct Value(Variant);

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Variant::None(none) => none.fmt(fmt),
            Variant::Bool(bool) => bool.fmt(fmt),
            Variant::Int(int) => int.fmt(fmt),
            Variant::Float(float) => float.fmt(fmt),
            Variant::Function(func) => func.fmt(fmt),
            Variant::List(list) => list.fmt(fmt),
            Variant::String(string) => string.fmt(fmt),
            Variant::Object(object) => object.fmt(fmt),
        }
    }
}

impl Value {
    pub const NONE: Self = Value(Variant::None(lib::None));

    pub fn new<T>(value: T) -> Self
    where
        T: TypeInstance + 'static,
    {
        Self::new_object(value)
    }

    fn new_object(value: impl TypeInstance + 'static) -> Self {
        let variant = match_type! {
            &value as &dyn TypeInstance,
                as_none: lib::None => Variant::None(as_none.clone()),
                as_bool: bool => Variant::Bool(*as_bool),
                as_int: i64 => Variant::Int(*as_int),
                as_float: f64 => Variant::Float(*as_float),
                as_func: Func => Variant::Function(as_func.clone()),
                as_list: List => Variant::List(as_list.clone()),
                as_string: DiceString => Variant::String(as_string.clone()),
                _ => Variant::Object(Gc::new(Box::new(value)))
        };

        Self(variant)
    }

    #[inline]
    pub fn assert_type(self, expected: &Symbol) -> Result<Self, RuntimeError> {
        if self.instance_type().name() == expected {
            Ok(self)
        } else {
            Err(RuntimeError::InvalidType(expected.clone(), self.instance_type().name().clone()))
        }
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
            Variant::Object(ref obj) => &***obj,
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
