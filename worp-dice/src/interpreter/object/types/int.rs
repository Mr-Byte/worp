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
        ObjectKey::Symbol(OP_NEG) => ObjectRef::new(Func1::new(negate)),
        ObjectKey::Symbol(OP_MUL) => ObjectRef::new(Func2::new(mul)),
        ObjectKey::Symbol(OP_DIV) => ObjectRef::new(Func2::new(div)),
        ObjectKey::Symbol(OP_REM) => ObjectRef::new(Func2::new(rem)),
        ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func2::new(add)),
        ObjectKey::Symbol(OP_SUB) => ObjectRef::new(Func2::new(sub)),
        ObjectKey::Symbol(OP_GT) => ObjectRef::new(Func2::new(gt)),
        ObjectKey::Symbol(OP_LT) => ObjectRef::new(Func2::new(lt)),
        ObjectKey::Symbol(OP_GTE) => ObjectRef::new(Func2::new(gte)),
        ObjectKey::Symbol(OP_LTE) => ObjectRef::new(Func2::new(lte)),
        ObjectKey::Symbol(OP_EQ) => ObjectRef::new(Func2::new(eq)),
        ObjectKey::Symbol(OP_NE) => ObjectRef::new(Func2::new(ne)),
        ObjectKey::Symbol(OP_COALESCE) => ObjectRef::new(Func2::from_raw(coalesce))
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

fn negate(arg: &i64) -> i64 {
    -arg
}

fn mul(lhs: &i64, rhs: &i64) -> i64 {
    lhs * rhs
}

fn div(lhs: &i64, rhs: &i64) -> i64 {
    lhs / rhs
}

fn rem(lhs: &i64, rhs: &i64) -> i64 {
    lhs % rhs
}

fn add(lhs: &i64, rhs: &i64) -> i64 {
    lhs + rhs
}

fn sub(lhs: &i64, rhs: &i64) -> i64 {
    lhs - rhs
}

fn gt(lhs: &i64, rhs: &i64) -> bool {
    lhs > rhs
}

fn lt(lhs: &i64, rhs: &i64) -> bool {
    lhs < rhs
}

fn gte(lhs: &i64, rhs: &i64) -> bool {
    lhs >= rhs
}

fn lte(lhs: &i64, rhs: &i64) -> bool {
    lhs <= rhs
}

fn eq(lhs: &i64, rhs: &i64) -> bool {
    lhs == rhs
}

fn ne(lhs: &i64, rhs: &i64) -> bool {
    lhs != rhs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_with_two_ints() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new(40);
        let rhs = ObjectRef::new(2);
        let result = lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(vec![lhs, rhs].as_slice())?;

        assert_eq!(42, *result.value::<i64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_add_with_lhs_int_rhs_none() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new(40);
        let rhs = ObjectRef::NONE;
        let result = lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(vec![lhs, rhs].as_slice());

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_eq_with_two_ints() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new(40);
        let rhs = ObjectRef::new(2);
        let result = lhs.get(&ObjectKey::Symbol(OP_EQ))?.call(vec![lhs, rhs].as_slice())?;

        assert_eq!(false, *result.value::<bool>().unwrap());

        Ok(())
    }
}
