use super::instruction::Instruction;
use crate::runtime::core::{Span, Value};
use bytes::Buf as _;
use std::{collections::HashMap, fmt::Display, io::Cursor, rc::Rc};

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

    pub fn cursor(&self) -> BytecodeCursor {
        BytecodeCursor {
            cursor: Cursor::new(self.inner.data.clone()),
        }
    }
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cursor = self.cursor();
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

pub struct BytecodeCursor {
    cursor: Cursor<Rc<[u8]>>,
}

impl BytecodeCursor {
    #[inline]
    pub fn set_position(&mut self, position: u64) {
        self.cursor.set_position(position)
    }

    #[inline]
    pub fn read_instruction(&mut self) -> Option<Instruction> {
        if self.cursor.has_remaining() {
            Some(self.cursor.get_u8().into())
        } else {
            None
        }
    }

    #[inline]
    pub fn read_offset(&mut self) -> i16 {
        self.cursor.get_i16()
    }

    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        self.cursor.get_u8()
    }

    #[inline]
    pub fn offset_position(&mut self, offset: i16) {
        self.set_position(self.cursor.position().wrapping_add(offset as u64));
    }

    pub fn position(&self) -> u64 {
        self.cursor.position()
    }
}
