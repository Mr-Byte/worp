#![allow(dead_code)]

use crate::runtime::{
    core::{reflection::Type, value::Value, TypeInstanceBase},
    error::RuntimeError,
};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

// TODO: Add TypeFunc.

#[derive(Clone)]
enum FuncVariant {
    Func0(Func0),
    Func1(Func1),
    Func2(Func2),
}

#[derive(Clone)]
pub struct Func(FuncVariant);

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

impl TypeInstanceBase for Func {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        match &self.0 {
            FuncVariant::Func0(func0) => func0.call(args),
            FuncVariant::Func1(func1) => func1.call(args),
            FuncVariant::Func2(func2) => func2.call(args),
        }
    }

    fn reflect_type(&self) -> Rc<dyn Type> {
        todo!()
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

#[cfg(test)]
mod test {

    // #[test]
    // fn func1_executes_successfully_with_one_argument_to_call() -> Result<(), RuntimeError> {
    //     let arg = ObjectInstance::new(42i64);
    //     let test_func = ObjectInstance::new(Func::new_func1(|arg: &i64| {
    //         assert_eq!(42, *arg);
    //     }));

    //     let result = test_func.call(&[arg])?;

    //     assert_eq!(ObjectInstance::NONE.value::<()>(), result.value::<()>());

    //     Ok(())
    // }
}
