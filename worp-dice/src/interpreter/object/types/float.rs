use super::func::Func;
use crate::interpreter::{
    error::RuntimeError,
    object::{key::ObjectKey, methods::to_string, reference::ObjectRef, reflection::TypeData, ObjectBase},
    symbol::common::{methods::FN_TO_STRING, operators::*, types::TY_FLOAT},
};
use maplit::hashmap;
use std::collections::HashMap;

thread_local! {
    static OPERATIONS: HashMap<ObjectKey, ObjectRef> = hashmap! [
        ObjectKey::Symbol(OP_NEG) => ObjectRef::new(Func::new_func1(negate)),
        ObjectKey::Symbol(OP_MUL) => ObjectRef::new(Func::new_func2(mul)),
        ObjectKey::Symbol(OP_DIV) => ObjectRef::new(Func::new_func2(div)),
        ObjectKey::Symbol(OP_REM) => ObjectRef::new(Func::new_func2(rem)),
        ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func::new_func2(add)),
        ObjectKey::Symbol(OP_SUB) => ObjectRef::new(Func::new_func2(sub)),
        ObjectKey::Symbol(OP_GT) => ObjectRef::new(Func::new_func2(gt)),
        ObjectKey::Symbol(OP_LT) => ObjectRef::new(Func::new_func2(lt)),
        ObjectKey::Symbol(OP_GTE) => ObjectRef::new(Func::new_func2(gte)),
        ObjectKey::Symbol(OP_LTE) => ObjectRef::new(Func::new_func2(lte)),
        ObjectKey::Symbol(OP_EQ) => ObjectRef::new(Func::new_func2(eq)),
        ObjectKey::Symbol(OP_NE) => ObjectRef::new(Func::new_func2(ne)),
        ObjectKey::Symbol(FN_TO_STRING) => ObjectRef::new(Func::new_func1(to_string::<f64>)),
    ];

    static TYPE_DATA: TypeData = TypeData::new(TY_FLOAT, Vec::new());
}

impl ObjectBase for f64 {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        OPERATIONS.with(|ops_table| ops_table.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone())))
    }

    fn format_value(&self) -> String {
        ToString::to_string(self)
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        OPERATIONS.with(|ops| {
            ops.clone()
                .into_iter()
                .map(|(key, value)| (key, value.instance_type_data()))
                .collect::<Vec<_>>()
        })
    }

    fn type_data() -> TypeData {
        TYPE_DATA.with(Clone::clone)
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data()
    }
}

fn negate(arg: &f64) -> f64 {
    -arg
}

fn mul(lhs: &f64, rhs: &f64) -> f64 {
    lhs * rhs
}

fn div(lhs: &f64, rhs: &f64) -> f64 {
    lhs / rhs
}

fn rem(lhs: &f64, rhs: &f64) -> f64 {
    lhs % rhs
}

fn add(lhs: &f64, rhs: &f64) -> f64 {
    lhs + rhs
}

fn sub(lhs: &f64, rhs: &f64) -> f64 {
    lhs - rhs
}

fn gt(lhs: &f64, rhs: &f64) -> bool {
    lhs > rhs
}

fn lt(lhs: &f64, rhs: &f64) -> bool {
    lhs < rhs
}

fn gte(lhs: &f64, rhs: &f64) -> bool {
    lhs >= rhs
}

fn lte(lhs: &f64, rhs: &f64) -> bool {
    lhs <= rhs
}

fn eq(lhs: &f64, rhs: &f64) -> bool {
    lhs == rhs
}

fn ne(lhs: &f64, rhs: &f64) -> bool {
    lhs != rhs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_with_two_ints() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new(40.0);
        let rhs = ObjectRef::new(2.0);
        let result = lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(vec![lhs, rhs].as_slice())?;

        assert_eq!(42.0, *result.value::<f64>().unwrap());

        Ok(())
    }

    #[test]
    fn test_add_with_lhs_int_rhs_none() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new(40.0);
        let rhs = ObjectRef::NONE;
        let result = lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(vec![lhs, rhs].as_slice());

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_eq_with_two_ints() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new(40.0);
        let rhs = ObjectRef::new(2.0);
        let result = lhs.get(&ObjectKey::Symbol(OP_EQ))?.call(vec![lhs, rhs].as_slice())?;

        assert_eq!(false, *result.value::<bool>().unwrap());

        Ok(())
    }
}
