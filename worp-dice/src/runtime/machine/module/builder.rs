use super::Module;
use crate::runtime::machine::bytecode::ByteCodeBuilder;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct ModuleBuilderInner {
    // moudle_name: Option<Symbol>,
    // classes: (),
    // traits: (),
    // functions: (),
    bytecode: ByteCodeBuilder,
}

#[derive(Default)]
pub struct ModuleBuilder {
    inner: Box<ModuleBuilderInner>,
}

impl Deref for ModuleBuilder {
    type Target = ModuleBuilderInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl DerefMut for ModuleBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

impl ModuleBuilder {
    pub fn build(self) -> Module {
        Module::new(self.inner.bytecode.build())
    }

    pub fn bytecode(&mut self) -> &mut ByteCodeBuilder {
        &mut self.bytecode
    }
}
