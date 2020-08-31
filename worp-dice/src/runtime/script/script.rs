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
        let mut instruction_count = 0;

        while let Some(instruction) = cursor.read_instruction() {
            write!(f, "{:6} | {:<20} | ", instruction_count, format!("{}", instruction))?;
            instruction_count += 1;

            match instruction {
                Instruction::PUSH_INT => write!(f, "{}", cursor.read_int())?,
                Instruction::PUSH_FLOAT => write!(f, "{}", cursor.read_float())?,
                Instruction::PUSH_CONST => write!(f, "{}", cursor.read_int())?,
                Instruction::JUMP => write!(f, "{}", cursor.read_offset())?,
                Instruction::JUMP_IF_FALSE => write!(f, "{}", cursor.read_offset())?,
                Instruction::LOAD_LOCAL => write!(f, "{}", cursor.read_offset())?,
                Instruction::STORE_LOCAL => write!(f, "{}", cursor.read_offset())?,

                _ => (),
            }

            writeln!(f)?;
        }
        Ok(())
    }
}
