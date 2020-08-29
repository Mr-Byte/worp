use super::{Symbol, TypeInstance};
use crate::runtime::{
    error::RuntimeError,
    lib::{self, Func, List},
};
use gc::{Finalize, Gc, Trace};
use std::{fmt::Display, ops::Deref};

#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub enum Value {
    None(lib::None),
    Unit(lib::Unit),
    Bool(bool),
    Int(i64),
    Float(f64),
    Function(Func),
    List(List),
    String(String),
    Object(Gc<Box<dyn TypeInstance>>),
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::None(none) => none.fmt(fmt),
            Value::Unit(unit) => unit.fmt(fmt),
            Value::Bool(bool) => bool.fmt(fmt),
            Value::Int(int) => int.fmt(fmt),
            Value::Float(float) => float.fmt(fmt),
            Value::Function(func) => func.fmt(fmt),
            Value::List(list) => list.fmt(fmt),
            Value::String(string) => string.fmt(fmt),
            Value::Object(object) => object.fmt(fmt),
        }
    }
}

impl Value {
    pub const NONE: Self = Value::None(lib::None);
    pub const UNIT: Self = Value::Unit(lib::Unit);

    pub fn new<T>(value: T) -> Self
    where
        T: TypeInstance + 'static,
    {
        Self::new_object(value)
    }

    fn new_object(value: impl TypeInstance + 'static) -> Self {
        match_type! {
            &value as &dyn TypeInstance,
                as_none: lib::None => Value::None(as_none.clone()),
                as_unit: lib::Unit => Value::Unit(as_unit.clone()),
                as_bool: bool => Value::Bool(*as_bool),
                as_int: i64 => Value::Int(*as_int),
                as_float: f64 => Value::Float(*as_float),
                as_func: Func => Value::Function(as_func.clone()),
                as_list: List => Value::List(as_list.clone()),
                as_string: String => Value::String(as_string.clone()),
                _ => Value::Object(Gc::new(Box::new(value)))
        }
    }

    #[inline]
    pub fn assert_type(self, expected: &Symbol) -> Result<Self, RuntimeError> {
        if self.instance_type().name() == expected {
            Ok(self)
        } else {
            Err(RuntimeError::InvalidType(
                expected.clone(),
                self.instance_type().name().clone(),
            ))
        }
    }
}

impl Deref for Value {
    type Target = dyn TypeInstance;

    fn deref(&self) -> &Self::Target {
        match self {
            Value::None(ref obj) => &*obj,
            Value::Unit(ref obj) => &*obj,
            Value::Bool(ref obj) => &*obj,
            Value::Int(ref obj) => &*obj,
            Value::Float(ref obj) => &*obj,
            Value::List(ref obj) => obj,
            Value::String(ref obj) => obj,
            Value::Function(ref obj) => obj,
            Value::Object(ref obj) => &***obj,
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
