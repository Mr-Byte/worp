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

    pub fn push_none(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_NONE.into());
    }

    pub fn push_int(&mut self, value: i64, span: Span) {
        self.source_map.insert(self.data.len() as u64, span.clone());
        self.data.put_u8(Instruction::PUSH_INT.into());

        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_i64(value);
    }

    pub fn push_float(&mut self, value: f64, span: Span) {
        self.source_map.insert(self.data.len() as u64, span.clone());
        self.data.put_u8(Instruction::PUSH_FLOAT.into());

        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_f64(value);
    }

    pub fn push_bool(&mut self, value: bool, span: Span) {
        let instruction = if value {
            Instruction::PUSH_TRUE
        } else {
            Instruction::PUSH_FALSE
        };

        self.source_map.insert(self.data.len() as u64, span.clone());
        self.data.put_u8(instruction.into());
    }

    pub fn push_const(&mut self, value: Value, span: Span) {
        let position = if let Some(position) = self.constants.iter().position(|current| *current == value) {
            position
        } else {
            self.constants.push(value);
            self.constants.len() - 1
        };

        self.source_map.insert(self.data.len() as u64, span.clone());
        self.data.put_u8(Instruction::PUSH_CONST.into());

        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u64(position as u64);
    }

    #[allow(dead_code)]
    pub fn pop(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::POP.into());
    }

    #[allow(dead_code)]
    pub fn dup(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DUP.into());
    }

    pub fn mul(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::MUL.into());
    }

    pub fn div(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DIV.into());
    }

    pub fn rem(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::REM.into());
    }

    pub fn add(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::ADD.into());
    }

    pub fn sub(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::SUB.into());
    }

    pub fn eq(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::EQ.into());
    }

    pub fn neq(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NEQ.into());
    }

    pub fn gt(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::GT.into());
    }

    pub fn gte(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::GTE.into());
    }

    pub fn lt(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LT.into());
    }

    pub fn lte(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LTE.into());
    }

    pub fn neg(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NEG.into());
    }

    pub fn not(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NOT.into());
    }
}
