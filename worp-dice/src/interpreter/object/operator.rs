use super::ObjectRef;
use crate::interpreter::{error::RuntimeError, symbol::common::types::TY_NONE};

pub fn coalesce(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    if lhs.tag() == TY_NONE {
        Ok(rhs.clone())
    } else {
        Ok(lhs.clone())
    }
}
