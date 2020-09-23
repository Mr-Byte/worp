use lib::{FnClosure, FnNative, FnScript};

use super::{Symbol, TypeInstance};
use crate::runtime::{
    error::RuntimeError,
    lib::{self, List},
};
use std::{fmt::Display, ops::Deref, rc::Rc};

#[derive(Clone, Debug)]
#[repr(C)]
pub enum Value {
    None(lib::None),
    Unit(lib::Unit),
    Bool(bool),
    Int(i64),
    Float(f64),
    FnClosure(FnClosure),
    FnScript(FnScript),
    FnNative(FnNative),
    List(List),
    String(String),
    Object(Rc<dyn TypeInstance>),
}

impl Value {
    pub const NONE: Self = Value::None(lib::None);
    pub const UNIT: Self = Value::Unit(lib::Unit);

    pub fn boxed<T: TypeInstance>(value: T) -> Value {
        Value::Object(Rc::new(value))
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

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::None(_), Value::None(_)) => true,
            (Value::Unit(_), Value::Unit(_)) => true,
            (Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
            (Value::Int(lhs), Value::Int(rhs)) => lhs == rhs,
            (Value::Float(lhs), Value::Float(rhs)) => lhs == rhs,
            (Value::FnClosure(lhs), Value::FnClosure(rhs)) => lhs == rhs,
            (Value::FnScript(lhs), Value::FnScript(rhs)) => lhs == rhs,
            (Value::List(lhs), Value::List(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::Object(lhs), Value::Object(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::None(none) => none.fmt(fmt),
            Value::Unit(unit) => unit.fmt(fmt),
            Value::Bool(bool) => bool.fmt(fmt),
            Value::Int(int) => int.fmt(fmt),
            Value::Float(float) => float.fmt(fmt),
            Value::FnClosure(func) => func.fmt(fmt),
            Value::FnScript(func) => func.fmt(fmt),
            Value::FnNative(func) => func.fmt(fmt),
            Value::List(list) => list.fmt(fmt),
            Value::String(string) => string.fmt(fmt),
            Value::Object(object) => object.fmt(fmt),
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
            Value::FnClosure(ref obj) => obj,
            Value::FnScript(ref obj) => obj,
            Value::FnNative(ref obj) => obj,
            Value::Object(ref obj) => &**obj,
        }
    }
}
