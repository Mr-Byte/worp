use std::{cell::Ref, cell::RefCell, cell::RefMut, fmt::Display, rc::Rc};

use crate::runtime::core::{TypeInstance, Upvalue};

use super::FnScript;

#[derive(Debug)]
pub struct FnClosureInner {
    pub fn_script: FnScript,
    pub upvalues: Box<[Upvalue]>,
}

#[derive(Clone, Debug)]
pub struct FnClosure {
    inner: Rc<RefCell<FnClosureInner>>,
}

impl FnClosure {
    pub fn new(fn_script: FnScript, upvalues: Box<[Upvalue]>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(FnClosureInner { fn_script, upvalues })),
        }
    }

    pub fn borrow(&self) -> Ref<'_, FnClosureInner> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, FnClosureInner> {
        self.inner.borrow_mut()
    }
}

decl_type! {
    impl TypeFnClosure for FnClosure as "FnClosure";
}

impl TypeInstance for FnClosure {}

// TODO: Create a way to more easily determine a unique function instance.
impl PartialEq for FnClosure {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().fn_script == other.borrow().fn_script
    }
}

impl Display for FnClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "closure{{{}}}", self.borrow().fn_script.name)
    }
}
