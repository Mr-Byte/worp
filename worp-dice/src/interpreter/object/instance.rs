use super::Object;
use std::{fmt::Debug, ops::Deref, sync::Arc};

#[derive(Clone)]
pub struct ObjectInstance(Arc<dyn Object>);

impl ObjectInstance {
    pub fn new(value: impl Object + 'static) -> ObjectInstance {
        Self(Arc::new(value) as Arc<dyn Object>)
    }
}

impl Debug for ObjectInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: More fully implement this.
        self.0.instance_type().fmt(f)
    }
}

impl Deref for ObjectInstance {
    type Target = dyn Object;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
