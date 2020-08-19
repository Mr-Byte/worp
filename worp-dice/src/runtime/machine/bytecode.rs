use super::instruction::Instruction;
use crate::runtime::core::Span;
use bytes::{Buf as _, BufMut as _};
use std::{collections::HashMap, io::Cursor};

#[derive(Clone)]
pub struct ByteCode {
    _source_map: HashMap<u64, Span>,
    data: Cursor<Box<[u8]>>,
}

impl ByteCode {
    pub fn builder() -> ByteCodeBuilder {
        ByteCodeBuilder::default()
    }

    pub fn set_position(&mut self, position: u64) {
        self.data.set_position(position)
    }

    pub fn read_instruction(&mut self) -> Option<Instruction> {
        if self.data.has_remaining() {
            Some(self.data.get_u8().into())
        } else {
            None
        }
    }

    pub fn read_bool(&mut self) -> bool {
        self.data.get_u8() != 0
    }

    pub fn read_int(&mut self) -> i64 {
        self.data.get_i64()
    }

    pub fn read_float(&mut self) -> f64 {
        self.data.get_f64()
    }
}

#[derive(Default)]
pub struct ByteCodeBuilder {
    source_map: HashMap<u64, Span>,
    data: bytes::BytesMut,
}

impl ByteCodeBuilder {
    pub fn build(self) -> ByteCode {
        ByteCode {
            _source_map: self.source_map,
            data: Cursor::new(self.data.to_vec().into_boxed_slice()),
        }
    }

    pub fn push_none(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::PUSH_NONE.into());
        self
    }

    pub fn push_int(&mut self, value: i64) -> &mut Self {
        self.data.put_u8(Instruction::PUSH_INT.into());
        self.data.put_i64(value);
        self
    }

    pub fn push_float(&mut self, value: f64) -> &mut Self {
        self.data.put_u8(Instruction::PUSH_FLOAT.into());
        self.data.put_f64(value);
        self
    }

    pub fn push_bool(&mut self, value: bool) -> &mut Self {
        self.data.put_u8(Instruction::PUSH_BOOL.into());
        self.data.put_u8(value as u8);
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::POP.into());
        self
    }

    pub fn dup(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::DUP.into());
        self
    }

    pub fn add(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::ADD.into());
        self
    }
}
