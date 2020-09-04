use super::{bytecode::Bytecode, callframe::CallFrame};
use std::fmt::Display;

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

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bytecode.fmt(f)
    }
}
