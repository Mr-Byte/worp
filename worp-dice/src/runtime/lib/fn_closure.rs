use std::{fmt::Display, ops::Deref, rc::Rc};

use crate::runtime::core::TypeInstance;

use super::FnScript;

#[derive(Debug)]
pub struct FnClosureInner {
    pub fn_script: FnScript,
}

#[derive(Clone, Debug)]
pub struct FnClosure {
    inner: Rc<FnClosureInner>,
}

impl FnClosure {
    pub fn new(fn_script: FnScript) -> Self {
        Self {
            inner: Rc::new(FnClosureInner { fn_script }),
        }
    }
}

decl_type! {
    impl TypeFnClosure for FnClosure as "FnClosure";
}

impl TypeInstance for FnClosure {}

impl Deref for FnClosure {
    type Target = FnClosureInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

// TODO: Create a way to more easily determine a unique function instance.
impl PartialEq for FnClosure {
    fn eq(&self, other: &Self) -> bool {
        self.fn_script == other.fn_script
    }
}

impl Display for FnClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "closure{{{}}}", self.fn_script.name)
    }
}
