use super::func::{Func1, Func2};
use crate::interpreter::{
    error::RuntimeError,
    object::{key::ObjectKey, operator::coalesce, reference::ObjectRef, reflection::TypeData, ObjectBase},
    symbol::common::{operators::*, types::TY_INT},
};
use maplit::hashmap;
use std::collections::HashMap;

thread_local! {
    static OPERATIONS: HashMap<ObjectKey, ObjectRef> = hashmap! [
        ObjectKey::Symbol(OP_NEG) => ObjectRef::new(Func1(negate)),
        ObjectKey::Symbol(OP_MUL) => ObjectRef::new(Func2(mul)),
        ObjectKey::Symbol(OP_DIV) => ObjectRef::new(Func2(div)),
        ObjectKey::Symbol(OP_REM) => ObjectRef::new(Func2(rem)),
        ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func2(add)),
        ObjectKey::Symbol(OP_SUB) => ObjectRef::new(Func2(sub)),
        ObjectKey::Symbol(OP_GT) => ObjectRef::new(Func2(gt)),
        ObjectKey::Symbol(OP_LT) => ObjectRef::new(Func2(lt)),
        ObjectKey::Symbol(OP_GTE) => ObjectRef::new(Func2(gte)),
        ObjectKey::Symbol(OP_LTE) => ObjectRef::new(Func2(lte)),
        ObjectKey::Symbol(OP_EQ) => ObjectRef::new(Func2(eq)),
        ObjectKey::Symbol(OP_NE) => ObjectRef::new(Func2(ne)),
        ObjectKey::Symbol(OP_COALESCE) => ObjectRef::new(Func2(coalesce))
    ];

    static TYPE_DATA: TypeData = TypeData::new(TY_INT, Vec::new());
}

impl ObjectBase for i64 {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        OPERATIONS.with(|ops_table| ops_table.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone())))
    }

    fn to_string(&self) -> String {
        ToString::to_string(self)
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        OPERATIONS.with(|ops| {
            ops.clone()
                .into_iter()
                .map(|(key, value)| (key, value.instance_type_data().clone()))
                .collect::<Vec<_>>()
        })
    }

    fn type_data() -> TypeData {
        TYPE_DATA.with(Clone::clone)
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data().clone()
    }
}

fn negate(arg: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    if let Some(value) = arg.value::<i64>() {
        Ok(ObjectRef::new_int(-value))
    } else {
        Err(RuntimeError::InvalidType(TY_INT, arg.instance_type_data().type_tag().clone()))
    }
}

fn mul(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_int(lhs * rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn div(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_int(lhs * rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn rem(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_int(lhs % rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn add(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_int(lhs + rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn sub(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_int(lhs - rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn gt(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_bool(lhs > rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn lt(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_bool(lhs < rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn gte(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_bool(lhs >= rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn lte(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_bool(lhs <= rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn eq(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_bool(lhs == rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

fn ne(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new_bool(lhs != rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.instance_type_data().type_tag().clone())),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_with_two_ints() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new_int(40);
        let rhs = ObjectRef::new_int(2);
        let result = lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(vec![lhs, rhs].as_slice())?;

        assert_eq!(42, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_add_with_lhs_int_rhs_none() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new_int(40);
        let rhs = ObjectRef::NONE;
        let result = lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(vec![lhs, rhs].as_slice());

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_eq_with_two_ints() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new_int(40);
        let rhs = ObjectRef::new_int(2);
        let result = lhs.get(&ObjectKey::Symbol(OP_EQ))?.call(vec![lhs, rhs].as_slice())?;

        assert_eq!(false, *result.value::<bool>().unwrap());

        Ok(())
    }
}
