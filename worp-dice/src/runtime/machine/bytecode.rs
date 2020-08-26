use super::instruction::Instruction;
use crate::runtime::core::{Span, Value};
use bytes::Buf as _;
use std::{collections::HashMap, io::Cursor};

#[derive(Clone)]
pub struct Bytecode {
    constants: Box<[Value]>,
    source_map: HashMap<u64, Span>,
    data: Cursor<Box<[u8]>>,
}

impl Bytecode {
    pub fn new(constants: Box<[Value]>, source_map: HashMap<u64, Span>, data: Cursor<Box<[u8]>>) -> Self {
        Self {
            constants,
            source_map,
            data,
        }
    }

    pub fn source_map(&self) -> &HashMap<u64, Span> {
        &self.source_map
    }

    pub fn constants(&self) -> &[Value] {
        &self.constants
    }

    pub fn span(&self) -> Option<Span> {
        self.source_map.get(&(self.data.position() - 1)).cloned()
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

    pub fn read_offset(&mut self) -> u16 {
        self.data.get_u16()
    }

    pub fn offset_position(&mut self, offset: u16) {
        self.set_position(self.data.position() + offset as u64);
    }
}
