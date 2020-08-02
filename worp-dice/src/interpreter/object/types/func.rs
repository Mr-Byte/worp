use crate::interpreter::{
    error::RuntimeError,
    object::{reference::ObjectRef, reflection::TypeData, ObjectBase, ObjectKey},
    symbol::common::types::TY_FUNC,
};
use std::fmt::Debug;

pub struct Func0<F>(pub F)
where
    F: Fn() -> Result<ObjectRef, RuntimeError>;

impl<F> ObjectBase for Func0<F>
where
    F: Fn() -> Result<ObjectRef, RuntimeError> + Send + Sync + 'static,
{
    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [] = args {
            Ok((self.0)()?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(0, args.len()))
        }
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_FUNC, vec![])
    }
}

impl<F> Debug for Func0<F>
where
    F: Fn() -> Result<ObjectRef, RuntimeError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function/0")
    }
}

pub struct Func1<F>(pub F)
where
    F: Fn(ObjectRef) -> Result<ObjectRef, RuntimeError>;

impl<F> ObjectBase for Func1<F>
where
    F: Fn(ObjectRef) -> Result<ObjectRef, RuntimeError> + Send + Sync + 'static,
{
    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [arg1] = args {
            Ok((self.0)(arg1.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_FUNC, vec![])
    }
}

impl<F> Debug for Func1<F>
where
    F: Fn(ObjectRef) -> Result<ObjectRef, RuntimeError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function/1")
    }
}

pub struct Func2<F>(pub F)
where
    F: Fn(ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError>;

impl<F> ObjectBase for Func2<F>
where
    F: Fn(ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError> + Send + Sync + 'static,
{
    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [arg1, arg2] = args {
            Ok((self.0)(arg1.clone(), arg2.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_FUNC, vec![])
    }
}

impl<F> Debug for Func2<F>
where
    F: Fn(ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function/2")
    }
}

pub struct Func3<F>(pub F)
where
    F: Fn(ObjectRef, ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError>;

impl<F> ObjectBase for Func3<F>
where
    F: Fn(ObjectRef, ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError> + Send + Sync + 'static,
{
    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [arg1, arg2, arg3] = args {
            Ok((self.0)(arg1.clone(), arg2.clone(), arg3.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(3, args.len()))
        }
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_FUNC, vec![])
    }
}

impl<F> Debug for Func3<F>
where
    F: Fn(ObjectRef, ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function/3")
    }
}

pub struct Func4<F>(pub F)
where
    F: Fn(ObjectRef, ObjectRef, ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError>;

impl<F> ObjectBase for Func4<F>
where
    F: Fn(ObjectRef, ObjectRef, ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError> + Send + Sync + 'static,
{
    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef, RuntimeError> {
        if let [arg1, arg2, arg3, arg4] = args {
            Ok((self.0)(arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(4, args.len()))
        }
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        Vec::new()
    }

    fn type_data(&self) -> TypeData {
        TypeData::new(TY_FUNC, vec![])
    }
}

impl<F> Debug for Func4<F>
where
    F: Fn(ObjectRef, ObjectRef, ObjectRef, ObjectRef) -> Result<ObjectRef, RuntimeError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function/4")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn func1_executes_successfully_with_one_argument_to_call() -> Result<(), RuntimeError> {
        let arg = ObjectRef::new(42i64);
        let test_func = ObjectRef::new(Func1(|arg: ObjectRef| {
            let arg1 = arg.value::<i64>();

            assert_eq!(42, *arg1.unwrap());

            Ok(ObjectRef::NONE)
        }));

        let result = test_func.call(&[arg.clone()])?;

        assert_eq!(ObjectRef::NONE.value::<()>(), result.value::<()>());

        Ok(())
    }
}
