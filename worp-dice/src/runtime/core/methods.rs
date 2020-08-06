use super::instance::ObjectInstance;
use crate::runtime::error::RuntimeError;

pub fn to_string(input: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
    Ok(ObjectInstance::new_string(input.format_value()))
}
