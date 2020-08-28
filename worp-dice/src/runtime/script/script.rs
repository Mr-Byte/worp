use super::CallFrame;
use crate::runtime::bytecode::Bytecode;

#[derive(Debug)]
pub struct Script {
    bytecode: Bytecode,
    call_frame: CallFrame,
}

impl Script {
    pub fn new(bytecode: Bytecode, call_frame: CallFrame) -> Self {
        Self { bytecode, call_frame }
    }

    pub fn bytecode(&mut self) -> &mut Bytecode {
        &mut self.bytecode
    }

    pub fn call_frame(&self) -> &CallFrame {
        &self.call_frame
    }
}
