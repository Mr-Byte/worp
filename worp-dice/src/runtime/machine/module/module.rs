use super::ModuleBuilder;
use crate::runtime::machine::bytecode::ByteCode;

#[derive(Clone)]
pub struct Module {
    bytecode: ByteCode,
}

impl Module {
    pub fn new(bytecode: ByteCode) -> Self {
        Self { bytecode }
    }

    pub fn bytecode(&mut self) -> &mut ByteCode {
        &mut self.bytecode
    }

    pub fn builder() -> ModuleBuilder {
        Default::default()
    }
}
