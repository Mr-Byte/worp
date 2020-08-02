#![allow(dead_code)]

use crate::interpreter::{
    error::RuntimeError,
    object::{reference::ObjectRef, reflection::TypeData, Object, ObjectBase, ObjectKey},
    symbol::common::types::TY_FUNC,
};
use std::{fmt::Debug, rc::Rc};

#[derive(Clone)]
pub enum Func {
    Func0(Func0),
    Func1(Func1),
    Func2(Func2),
}

impl ObjectBase for Func {
    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        match self {
            Func::Func0(func0) => func0.call(args),
            Func::Func1(func1) => func1.call(args),
            Func::Func2(func2) => func2.call(args),
        }
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data() -> TypeData
    where
        Self: Sized,
    {
        TypeData::new(TY_FUNC, vec![])
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data().clone()
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Func::Func0(_) => write!(f, "Function/0"),
            Func::Func1(_) => write!(f, "Function/1"),
            Func::Func2(_) => write!(f, "Function/2"),
        }
    }
}

#[derive(Clone)]
pub struct Func0(Rc<dyn Fn() -> Result<ObjectRef, RuntimeError>>);

impl Func0 {
    pub fn new<R, F1>(func: F1) -> Func
    where
        R: Object,
        F1: Fn() -> R + 'static,
    {
        Func::Func0(Self(Rc::new(move || Ok(ObjectRef::new(func())))))
    }

    pub fn from_raw(func: impl Fn() -> Result<ObjectRef, RuntimeError> + 'static) -> Func {
        Func::Func0(Self(Rc::new(func)))
    }

    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [] = args {
            Ok((self.0)()?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(0, args.len()))
        }
    }
}

#[derive(Clone)]
pub struct Func1(Rc<dyn Fn(ObjectRef) -> Result<ObjectRef, RuntimeError>>);

impl Func1 {
    pub fn new<A, R, F1>(func: F1) -> Func
    where
        A: Object,
        R: Object,
        F1: Fn(&A) -> R + 'static,
    {
        Func::Func1(Self(Rc::new(move |arg| {
            if let Some(value) = arg.value::<A>() {
                Ok(ObjectRef::new(func(value)))
            } else {
                Err(RuntimeError::InvalidType(
                    A::type_data().type_tag().clone(),
                    arg.instance_type_data().type_tag().clone(),
                ))
            }
        })))
    }

    pub fn from_raw(func: impl Fn(ObjectRef) -> Result<ObjectRef, RuntimeError> + 'static) -> Func {
        Func::Func1(Self(Rc::new(func)))
    }

    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [arg1] = args {
            Ok((self.0)(arg1.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }
}

#[derive(Clone)]
pub struct Func2(Rc<dyn Fn(ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError>>);

impl Func2 {
    pub fn new<A1, A2, R, F1>(func: F1) -> Func
    where
        A1: Object,
        A2: Object,
        R: Object,
        F1: Fn(&A1, &A2) -> R + 'static,
    {
        Func::Func2(Self(Rc::new(move |arg1, arg2| {
            let args = (arg1.value::<A1>(), arg2.value::<A2>());

            match args {
                (Some(value1), Some(value2)) => Ok(ObjectRef::new(func(value1, value2))),
                (None, _) => Err(RuntimeError::InvalidType(
                    A1::type_data().type_tag().clone(),
                    arg1.instance_type_data().type_tag().clone(),
                )),
                (_, None) => Err(RuntimeError::InvalidType(
                    A2::type_data().type_tag().clone(),
                    arg2.instance_type_data().type_tag().clone(),
                )),
            }
        })))
    }

    pub fn from_raw(func: impl Fn(ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError> + 'static) -> Func {
        Func::Func2(Self(Rc::new(func)))
    }

    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [arg1, arg2] = args {
            Ok((self.0)(arg1.clone(), arg2.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn func1_executes_successfully_with_one_argument_to_call() -> Result<(), RuntimeError> {
        let arg = ObjectRef::new(42i64);
        let test_func = ObjectRef::new(Func1::new(|arg: &i64| {
            assert_eq!(42, *arg);
        }));

        let result = test_func.call(&[arg.clone()])?;

        assert_eq!(ObjectRef::NONE.value::<()>(), result.value::<()>());

        Ok(())
    }
}
