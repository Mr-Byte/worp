use crate::interpreter::object::Object;
use std::any::Any;

impl Object for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
