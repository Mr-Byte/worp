use std::{
    fmt::{Debug, Display},
    ops::Deref,
    rc::Rc,
};

use crate::runtime::core::{TypeInstance, Upvalue};

use super::FnScript;

pub struct FnClosureInner {
    pub fn_script: FnScript,
    pub upvalues: Box<[Upvalue]>,
}

impl Debug for FnClosureInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "closure{{{}}}", self.fn_script)
    }
}

#[derive(Clone)]
pub struct FnClosure {
    inner: Rc<FnClosureInner>,
}

impl FnClosure {
    pub fn new(fn_script: FnScript, upvalues: Box<[Upvalue]>) -> Self {
        Self {
            inner: Rc::new(FnClosureInner { fn_script, upvalues }),
        }
    }
}

impl Deref for FnClosure {
    type Target = FnClosureInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Debug for FnClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

decl_type! {
    impl TypeFnClosure for FnClosure as "FnClosure";
}

impl TypeInstance for FnClosure {}

impl PartialEq for FnClosure {
    fn eq(&self, other: &Self) -> bool {
        self.fn_script == other.fn_script
            && std::ptr::eq(
                &*self.upvalues as *const [Upvalue] as *const u8,
                &*other.upvalues as *const [Upvalue] as *const u8,
            )
    }
}

impl Display for FnClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "closure{{{}}}", self.fn_script)
    }
}
