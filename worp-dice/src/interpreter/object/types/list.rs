use crate::interpreter::object::{Object, ObjectRef};
use std::any::Any;

impl Object for Vec<ObjectRef> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
