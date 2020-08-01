use super::func::{Func1, Func2};
use crate::interpreter::{
    error::RuntimeError,
    object::{key::ObjectKey, reference::ObjectRef, ObjectBase},
    symbol::{
        common::{operators::*, types::TY_INT},
        Symbol,
    },
};
use maplit::hashmap;
use std::collections::HashMap;

thread_local! {
    static INTEGER_OPERATIONS: HashMap<ObjectKey, ObjectRef> = hashmap! [
        ObjectKey::Symbol(OP_NEG) => ObjectRef::new(Func1(negate)),
        ObjectKey::Symbol(OP_MUL) => ObjectRef::new(Func2(mul)),
        ObjectKey::Symbol(OP_DIV) => ObjectRef::new(Func2(div)),
        ObjectKey::Symbol(OP_REM) => ObjectRef::new(Func2(rem)),
        ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func2(add)),
        ObjectKey::Symbol(OP_SUB) => ObjectRef::new(Func2(sub)),
    ];
}

impl ObjectBase for i64 {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        INTEGER_OPERATIONS.with(|ops_table| ops_table.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone())))
    }

    fn to_string(&self) -> String {
        ToString::to_string(self)
    }

    fn type_name(&self) -> Symbol {
        TY_INT
    }
}

fn negate(arg: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    if let Some(value) = arg.value::<i64>() {
        Ok(ObjectRef::new(!value))
    } else {
        Err(RuntimeError::InvalidType(TY_INT, arg.type_name()))
    }
}

fn mul(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new(lhs * rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.type_name())),
        (None, Some(_)) => Err(RuntimeError::InvalidType(TY_INT, lhs.type_name())),
        (None, None) => Err(RuntimeError::Aborted), // TODO Figure out a good error for this?
    }
}

fn div(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new(lhs * rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.type_name())),
        (None, Some(_)) => Err(RuntimeError::InvalidType(TY_INT, lhs.type_name())),
        (None, None) => Err(RuntimeError::Aborted), // TODO Figure out a good error for this?
    }
}

fn rem(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new(lhs % rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.type_name())),
        (None, Some(_)) => Err(RuntimeError::InvalidType(TY_INT, lhs.type_name())),
        (None, None) => Err(RuntimeError::Aborted), // TODO Figure out a good error for this?
    }
}

fn add(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new(lhs + rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.type_name())),
        (None, Some(_)) => Err(RuntimeError::InvalidType(TY_INT, lhs.type_name())),
        (None, None) => Err(RuntimeError::Aborted), // TODO Figure out a good error for this?
    }
}

fn sub(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let args = (lhs.value::<i64>(), rhs.value::<i64>());

    match args {
        (Some(lhs), Some(rhs)) => Ok(ObjectRef::new(lhs - rhs)),
        (Some(_), None) => Err(RuntimeError::InvalidType(TY_INT, rhs.type_name())),
        (None, Some(_)) => Err(RuntimeError::InvalidType(TY_INT, lhs.type_name())),
        (None, None) => Err(RuntimeError::Aborted), // TODO Figure out a good error for this?
    }
}
