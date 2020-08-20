#![allow(dead_code)]

use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use gc::{Finalize, Trace};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

decl_type! {
    impl TypeFunc for Func as "Func";
}

#[derive(Clone, PartialEq)]
enum FuncVariant {
    Func0(Func0),
    Func1(Func1),
    Func2(Func2),
}

#[derive(Clone, Trace, Finalize, PartialEq)]
pub struct Func(#[unsafe_ignore_trace] FuncVariant);

impl Func {
    pub fn new_func0(func: impl Fn() -> Result<Value, RuntimeError> + 'static) -> Self {
        Self(FuncVariant::Func0(Func0(Rc::new(func))))
    }

    pub fn new_func1(func: impl Fn(Value) -> Result<Value, RuntimeError> + 'static) -> Self {
        Self(FuncVariant::Func1(Func1(Rc::new(func))))
    }

    pub fn new_func2(func: impl Fn(Value, Value) -> Result<Value, RuntimeError> + 'static) -> Self {
        Self(FuncVariant::Func2(Func2(Rc::new(func))))
    }
}

impl TypeInstance for Func {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        match &self.0 {
            FuncVariant::Func0(func0) => func0.call(args),
            FuncVariant::Func1(func1) => func1.call(args),
            FuncVariant::Func2(func2) => func2.call(args),
        }
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            FuncVariant::Func0(_) => write!(f, "Function/0"),
            FuncVariant::Func1(_) => write!(f, "Function/1"),
            FuncVariant::Func2(_) => write!(f, "Function/2"),
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            FuncVariant::Func0(_) => write!(f, "[Function/0]"),
            FuncVariant::Func1(_) => write!(f, "[Function/1]"),
            FuncVariant::Func2(_) => write!(f, "[Function/2]"),
        }
    }
}

#[derive(Clone)]
struct Func0(Rc<dyn Fn() -> Result<Value, RuntimeError>>);

impl Func0 {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        if let [] = args {
            Ok((self.0)()?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(0, args.len()))
        }
    }
}

impl PartialEq for Func0 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}

#[derive(Clone)]
struct Func1(Rc<dyn Fn(Value) -> Result<Value, RuntimeError>>);

impl Func1 {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        if let [arg1] = args {
            Ok((self.0)(arg1.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }
}

impl PartialEq for Func1 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}

#[derive(Clone)]
pub struct Func2(Rc<dyn Fn(Value, Value) -> Result<Value, RuntimeError>>);

impl Func2 {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        if let [arg1, arg2] = args {
            Ok((self.0)(arg1.clone(), arg2.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }
}

impl PartialEq for Func2 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}
