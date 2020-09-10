use cursor::BytecodeCursor;

use super::instruction::Instruction;
use crate::runtime::core::{Span, Value};
use std::{collections::HashMap, fmt::Display, rc::Rc};

mod cursor;

#[derive(Debug)]
struct BytecodeInner {
    slot_count: usize,
    constants: Box<[Value]>,
    data: Box<[u8]>,
    source_map: HashMap<u64, Span>,
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    inner: Rc<BytecodeInner>,
}

impl Bytecode {
    pub fn new(data: Box<[u8]>, slot_count: usize, constants: Box<[Value]>, source_map: HashMap<u64, Span>) -> Self {
        Self {
            inner: Rc::new(BytecodeInner {
                constants,
                slot_count,
                source_map,
                data,
            }),
        }
    }

    #[allow(dead_code)]
    pub fn source_map(&self) -> &HashMap<u64, Span> {
        &self.inner.source_map
    }

    pub fn constants(&self) -> &[Value] {
        &self.inner.constants
    }

    pub fn cursor(&self) -> BytecodeCursor<'_> {
        BytecodeCursor::new(&*self.inner.data)
    }

    pub fn slot_count(&self) -> usize {
        self.inner.slot_count
    }
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Code")?;
        writeln!(f, "--------")?;

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
                | Instruction::BUILD_LIST
                | Instruction::CALL => write!(f, "{}", cursor.read_u8())?,
                _ => (),
            }

            position = cursor.position();

            writeln!(f)?;
        }

        writeln!(f)?;

        for const_value in self.constants() {
            if let Value::Func(func) = const_value {
                if let Some(bytecode) = func.bytecode() {
                    writeln!(f, "Function: {:?}", func.name())?;
                    writeln!(f, "--------")?;
                    bytecode.fmt(f)?;
                }
            }
        }

        Ok(())
    }
}
