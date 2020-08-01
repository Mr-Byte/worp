use crate::interpreter::object::Object;
use std::any::Any;

impl Object for f64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
