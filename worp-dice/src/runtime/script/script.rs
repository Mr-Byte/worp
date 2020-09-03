use super::CallFrame;
use crate::runtime::{bytecode::Bytecode, instruction::Instruction};
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
        let mut cursor = self.bytecode.cursor();
        let mut position = 0;

        while let Some(instruction) = cursor.read_instruction() {
            write!(f, "{:6} | {:<24} | ", position, format!("{}", instruction))?;

            match instruction {
                Instruction::JUMP | Instruction::JUMP_IF_FALSE => write!(f, "{}", cursor.read_offset())?,
                Instruction::PUSH_CONST
                | Instruction::LOAD_LOCAL
                | Instruction::STORE_LOCAL
                | Instruction::MUL_ASSIGN_LOCAL
                | Instruction::DIV_ASSIGN_LOCAL
                | Instruction::ADD_ASSIGN_LOCAL
                | Instruction::SUB_ASSIGN_LOCAL
                | Instruction::BUILD_LIST => write!(f, "{}", cursor.read_u8())?,
                _ => (),
            }

            position = cursor.position();

            writeln!(f)?;
        }
        Ok(())
    }
}
