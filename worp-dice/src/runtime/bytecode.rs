use super::instruction::Instruction;
use crate::runtime::core::{Span, Value};
use bytes::Buf as _;
use std::{collections::HashMap, io::Cursor, rc::Rc};

#[derive(Debug)]
struct BytecodeInner {
    constants: Box<[Value]>,
    source_map: HashMap<u64, Span>,
    data: Rc<[u8]>,
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    inner: Rc<BytecodeInner>,
}

impl Bytecode {
    pub fn new(constants: Box<[Value]>, source_map: HashMap<u64, Span>, data: Rc<[u8]>) -> Self {
        Self {
            inner: Rc::new(BytecodeInner {
                constants,
                source_map,
                data,
            }),
        }
    }

    pub fn source_map(&self) -> &HashMap<u64, Span> {
        &self.inner.source_map
    }

    pub fn constants(&self) -> &[Value] {
        &self.inner.constants
    }

    pub fn span_of(&self, position: usize) -> Option<Span> {
        self.inner.source_map.get(&(position as u64)).cloned()
    }

    pub fn cursor(&self) -> BytecodeCursor {
        BytecodeCursor {
            cursor: Cursor::new(self.inner.data.clone()),
        }
    }
}

pub struct BytecodeCursor {
    cursor: Cursor<Rc<[u8]>>,
}

impl BytecodeCursor {
    #[inline(always)]
    pub fn set_position(&mut self, position: u64) {
        self.cursor.set_position(position)
    }

    #[inline(always)]
    pub fn read_instruction(&mut self) -> Option<Instruction> {
        if self.cursor.has_remaining() {
            Some(self.cursor.get_u8().into())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn read_int(&mut self) -> i64 {
        self.cursor.get_i64()
    }

    #[inline(always)]
    pub fn read_float(&mut self) -> f64 {
        self.cursor.get_f64()
    }

    #[inline(always)]
    pub fn read_offset(&mut self) -> i16 {
        self.cursor.get_i16()
    }

    #[inline(always)]
    pub fn read_u16(&mut self) -> u16 {
        self.cursor.get_u16()
    }

    #[inline(always)]
    pub fn read_u8(&mut self) -> u8 {
        self.cursor.get_u8()
    }

    #[inline(always)]
    pub fn offset_position(&mut self, offset: i16) {
        self.set_position(self.cursor.position().wrapping_add(offset as u64));
    }
}
