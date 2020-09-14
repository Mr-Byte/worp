use std::{fmt::Display, ops::Deref, rc::Rc};

use crate::runtime::{core::TypeInstance, interpreter::bytecode::Bytecode};

#[derive(Debug)]
pub struct FnScriptInner {
    pub arity: usize,
    pub name: String,
    pub bytecode: Bytecode,
}

#[derive(Clone, Debug)]
pub struct FnScript {
    inner: Rc<FnScriptInner>,
}

impl FnScript {
    pub fn new(name: String, arity: usize, bytecode: Bytecode) -> Self {
        Self {
            inner: Rc::new(FnScriptInner { arity, bytecode, name }),
        }
    }
}

decl_type! {
    impl TypeFnScript for FnScript as "FnScript";
}

impl TypeInstance for FnScript {}

impl Deref for FnScript {
    type Target = FnScriptInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

// TODO: Create a way to more easily determine a unique function instance.
impl PartialEq for FnScript {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && self.name == other.name
    }
}

impl Display for FnScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.name, self.arity)
    }
}
