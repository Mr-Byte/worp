#![allow(dead_code)]

use crate::runtime::{core::TypeInstance, interpreter::bytecode::Bytecode};
use std::{
    fmt::{Debug, Display},
    ops::Deref,
    rc::Rc,
};

decl_type! {
    impl TypeFunc for Func as "Func";
}

#[derive(Clone, PartialEq)]
pub enum FnType {
    FnNative,
    FnScript(FnScript),
    FnClosure,
}

#[derive(Clone, PartialEq)]
pub struct Func {
    target: FnType,
}

impl Func {
    pub fn new_fn(name: String, arity: usize, bytecode: Bytecode) -> Self {
        Self {
            target: FnType::FnScript(FnScript::new(name, arity, bytecode)),
        }
    }

    pub fn target(&self) -> &FnType {
        &self.target
    }
}

impl TypeInstance for Func {}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.target {
            FnType::FnScript(decl) => write!(f, "{}/{}", decl.name, decl.arity),
            _ => todo!(),
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.target {
            FnType::FnScript(decl) => write!(f, "[{}/{}]", decl.name, decl.arity),
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FnScript {
    inner: Rc<FnScriptInner>,
}

impl Deref for FnScript {
    type Target = FnScriptInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

#[derive(Debug)]
pub struct FnScriptInner {
    pub arity: usize,
    pub name: String,
    pub bytecode: Bytecode,
}

impl FnScript {
    fn new(name: String, arity: usize, bytecode: Bytecode) -> Self {
        Self {
            inner: Rc::new(FnScriptInner { arity, bytecode, name }),
        }
    }
}

// TODO: Create a way to more easily determine a unique function instance.
impl PartialEq for FnScript {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && self.name == other.name
    }
}
