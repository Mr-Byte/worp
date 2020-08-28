use crate::runtime::bytecode::Bytecode;

// TODO: Move modue parts to an inner struct, stored in an Rc.
#[derive(Clone)]
pub struct Module {
    bytecode: Bytecode,
}

impl Module {
    pub fn new(bytecode: Bytecode) -> Self {
        Self { bytecode }
    }

    pub fn bytecode(&mut self) -> &mut Bytecode {
        &mut self.bytecode
    }
}
