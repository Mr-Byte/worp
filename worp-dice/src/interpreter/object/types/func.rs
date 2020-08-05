#![allow(dead_code)]

use crate::interpreter::{
    error::RuntimeError,
    object::{reference::ObjectRef, reflection::TypeData, Object, ObjectBase, ObjectKey},
    symbol::common::types::TY_FUNC,
};
use std::{fmt::Debug, rc::Rc};

#[derive(Clone)]
enum FuncVariant {
    Func0(Func0),
    Func1(Func1),
    Func2(Func2),
}

#[derive(Clone)]
pub struct Func(FuncVariant);

impl Func {
    pub fn new_func0<R, F1>(func: F1) -> Self
    where
        R: Object,
        F1: Fn() -> R + 'static,
    {
        Self(FuncVariant::Func0(Func0(Rc::new(move || Ok(ObjectRef::new(func()))))))
    }

    pub fn from_raw_func0(func: impl Fn() -> Result<ObjectRef, RuntimeError> + 'static) -> Self {
        Self(FuncVariant::Func0(Func0(Rc::new(func))))
    }

    pub fn new_func1<A, R, F1>(func: F1) -> Self
    where
        A: Object,
        R: Object,
        F1: Fn(&A) -> R + 'static,
    {
        Self(FuncVariant::Func1(Func1(Rc::new(move |arg| {
            if let Some(value) = arg.value::<A>() {
                Ok(ObjectRef::new(func(value)))
            } else {
                Err(RuntimeError::InvalidType(
                    A::type_data().type_tag().clone(),
                    arg.instance_type_data().type_tag().clone(),
                ))
            }
        }))))
    }

    pub fn from_raw_func1(func: impl Fn(ObjectRef) -> Result<ObjectRef, RuntimeError> + 'static) -> Self {
        Self(FuncVariant::Func1(Func1(Rc::new(func))))
    }

    pub fn new_func2<A1, A2, R, F1>(func: F1) -> Self
    where
        A1: Object,
        A2: Object,
        R: Object,
        F1: Fn(&A1, &A2) -> R + 'static,
    {
        Func(FuncVariant::Func2(Func2(Rc::new(move |arg1, arg2| {
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
        }))))
    }

    pub fn from_raw_func2(func: impl Fn(ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError> + 'static) -> Self {
        Self(FuncVariant::Func2(Func2(Rc::new(func))))
    }
}

impl ObjectBase for Func {
    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        match &self.0 {
            FuncVariant::Func0(func0) => func0.call(args),
            FuncVariant::Func1(func1) => func1.call(args),
            FuncVariant::Func2(func2) => func2.call(args),
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
        Self::type_data()
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

#[derive(Clone)]
struct Func0(Rc<dyn Fn() -> Result<ObjectRef, RuntimeError>>);

impl Func0 {
    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [] = args {
            Ok((self.0)()?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(0, args.len()))
        }
    }
}

#[derive(Clone)]
struct Func1(Rc<dyn Fn(ObjectRef) -> Result<ObjectRef, RuntimeError>>);

impl Func1 {
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
        let test_func = ObjectRef::new(Func::new_func1(|arg: &i64| {
            assert_eq!(42, *arg);
        }));

        let result = test_func.call(&[arg])?;

        assert_eq!(ObjectRef::NONE.value::<()>(), result.value::<()>());

        Ok(())
    }
}
