use super::ObjectRef;
use crate::interpreter::error::RuntimeError;

pub fn to_string(input: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    Ok(ObjectRef::new_string(input.format_value()))
}
