#![allow(dead_code)]

use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
    interpreter::bytecode::Bytecode,
};
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
    Func0(Func0),
    Func1(Func1),
    Func2(Func2),
    FnNative,
    FnScript(FnScript),
}

#[derive(Clone, PartialEq)]
pub struct Func {
    target: FnType,
}

impl Func {
    pub fn new_func0(func: impl Fn() -> Result<Value, RuntimeError> + 'static) -> Self {
        Self {
            target: FnType::Func0(Func0(Rc::new(func))),
        }
    }

    pub fn new_func1(func: impl Fn(Value) -> Result<Value, RuntimeError> + 'static) -> Self {
        Self {
            target: FnType::Func1(Func1(Rc::new(func))),
        }
    }

    pub fn new_func2(func: impl Fn(Value, Value) -> Result<Value, RuntimeError> + 'static) -> Self {
        Self {
            target: FnType::Func2(Func2(Rc::new(func))),
        }
    }

    pub fn new_fn(name: String, arity: usize, bytecode: Bytecode) -> Self {
        Self {
            target: FnType::FnScript(FnScript::new(name, arity, bytecode)),
        }
    }

    pub fn target(&self) -> &FnType {
        &self.target
    }
}

impl TypeInstance for Func {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        match &self.target {
            FnType::Func0(func0) => func0.call(args),
            FnType::Func1(func1) => func1.call(args),
            FnType::Func2(func2) => func2.call(args),
            _ => todo!(),
        }
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.target {
            FnType::Func0(_) => write!(f, "Function/0"),
            FnType::Func1(_) => write!(f, "Function/1"),
            FnType::Func2(_) => write!(f, "Function/2"),
            FnType::FnScript(decl) => write!(f, "{}/{}", decl.name, decl.arity),
            _ => todo!(),
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.target {
            FnType::Func0(_) => write!(f, "[Function/0]"),
            FnType::Func1(_) => write!(f, "[Function/1]"),
            FnType::Func2(_) => write!(f, "[Function/2]"),
            FnType::FnScript(decl) => write!(f, "[{}/{}]", decl.name, decl.arity),
            _ => todo!(),
        }
    }
}

type Func0Object = dyn Fn() -> Result<Value, RuntimeError>;

#[derive(Clone)]
pub struct Func0(Rc<Func0Object>);

impl Func0 {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        if let [] = args {
            Ok((self.0)()?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(0, args.len()))
        }
    }
}

impl PartialEq for Func0 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            &*self.0 as *const Func0Object as *const u8,
            &*other.0 as *const Func0Object as *const u8,
        )
    }
}

type Func1Object = dyn Fn(Value) -> Result<Value, RuntimeError>;

#[derive(Clone)]
pub struct Func1(Rc<Func1Object>);

impl Func1 {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        if let [arg1] = args {
            Ok((self.0)(arg1.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }
}

impl PartialEq for Func1 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            &*self.0 as *const Func1Object as *const u8,
            &*other.0 as *const Func1Object as *const u8,
        )
    }
}

type Func2Object = dyn Fn(Value, Value) -> Result<Value, RuntimeError>;

#[derive(Clone)]
pub struct Func2(Rc<Func2Object>);

impl Func2 {
    fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        if let [arg1, arg2] = args {
            Ok((self.0)(arg1.clone(), arg2.clone())?)
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }
}

impl PartialEq for Func2 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            &*self.0 as *const Func2Object as *const u8,
            &*other.0 as *const Func2Object as *const u8,
        )
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
