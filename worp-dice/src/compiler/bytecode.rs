use crate::runtime::{
    core::{Span, Value},
    machine::{bytecode::Bytecode, instruction::Instruction},
};
use bytes::BufMut as _;
use std::{collections::HashMap, io::Cursor};

#[derive(Default)]
pub struct BytecodeGenerator {
    constants: Vec<Value>,
    source_map: HashMap<u64, Span>,
    data: Vec<u8>,
}

impl BytecodeGenerator {
    pub fn generate(self) -> Bytecode {
        Bytecode::new(
            self.constants.into_boxed_slice(),
            self.source_map,
            Cursor::new(self.data.into_boxed_slice()),
        )
    }

    pub fn push_none(&mut self) {
        self.data.put_u8(Instruction::PUSH_NONE.into());
    }

    pub fn push_int(&mut self, value: i64) {
        self.data.put_u8(Instruction::PUSH_INT.into());
        self.data.put_i64(value);
    }

    pub fn push_float(&mut self, value: f64) {
        self.data.put_u8(Instruction::PUSH_FLOAT.into());
        self.data.put_f64(value);
    }

    pub fn push_bool(&mut self, value: bool) {
        let instruction = if value {
            Instruction::PUSH_TRUE
        } else {
            Instruction::PUSH_FALSE
        };

        self.data.put_u8(instruction.into());
        self.data.put_u8(value as u8);
    }

    pub fn push_const(&mut self, value: Value) {
        // TODO: Dedupe constants?
        let position = if let Some(position) = self.constants.iter().position(|current| *current == value) {
            position
        } else {
            self.constants.push(value);
            self.constants.len() - 1
        };

        self.data.put_u8(Instruction::PUSH_CONST.into());
        self.data.put_u64(position as u64);
    }

    pub fn pop(&mut self) {
        self.data.put_u8(Instruction::POP.into());
    }

    pub fn dup(&mut self) {
        self.data.put_u8(Instruction::DUP.into());
    }

    pub fn mul(&mut self) {
        self.data.put_u8(Instruction::MUL.into());
    }

    pub fn div(&mut self) {
        self.data.put_u8(Instruction::DIV.into());
    }

    pub fn rem(&mut self) {
        self.data.put_u8(Instruction::REM.into());
    }

    pub fn add(&mut self) {
        self.data.put_u8(Instruction::ADD.into());
    }

    pub fn sub(&mut self) {
        self.data.put_u8(Instruction::SUB.into());
    }
}
