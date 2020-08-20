use super::ModuleBuilder;
use crate::runtime::machine::bytecode::ByteCode;

// TODO: Move modue parts to an inner struct, stored in an Rc.
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
