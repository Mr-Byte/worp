use crate::runtime::lib::{self, List};
use lib::{FnClosure, FnNative, FnScript};
use std::fmt::Display;

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
}

impl Value {
    pub const NONE: Self = Value::None(lib::None);
    pub const UNIT: Self = Value::Unit(lib::Unit);
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
        }
    }
}
