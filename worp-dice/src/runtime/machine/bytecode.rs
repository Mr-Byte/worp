use super::instruction::Instruction;
use crate::runtime::core::{Span, Value};
use bytes::{Buf as _, BufMut as _};
use std::{collections::HashMap, io::Cursor};

#[derive(Clone)]
pub struct ByteCode {
    constants: Box<[Value]>,
    source_map: HashMap<u64, Span>,
    data: Cursor<Box<[u8]>>,
}

impl ByteCode {
    pub fn builder() -> ByteCodeBuilder {
        ByteCodeBuilder::default()
    }

    pub fn constants(&self) -> &[Value] {
        &self.constants
    }

    pub fn span(&self) -> Option<Span> {
        self.source_map.get(&self.data.position()).cloned()
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
    constants: Vec<Value>,
    source_map: HashMap<u64, Span>,
    data: Vec<u8>,
}

impl ByteCodeBuilder {
    pub fn build(self) -> ByteCode {
        ByteCode {
            constants: self.constants.into_boxed_slice(),
            source_map: self.source_map,
            data: Cursor::new(self.data.into_boxed_slice()),
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
        let instruction = if value {
            Instruction::PUSH_TRUE
        } else {
            Instruction::PUSH_FALSE
        };

        self.data.put_u8(instruction.into());
        self.data.put_u8(value as u8);
        self
    }

    pub fn push_const(&mut self, value: Value) -> &mut Self {
        // TODO: Dedupe constants?
        let position = if let Some(position) = self.constants.iter().position(|current| *current == value) {
            position
        } else {
            self.constants.push(value);
            self.constants.len() - 1
        };

        self.data.put_u8(Instruction::PUSH_CONST.into());
        self.data.put_u64(position as u64);
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

    pub fn mul(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::MUL.into());
        self
    }

    pub fn div(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::DIV.into());
        self
    }

    pub fn rem(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::REM.into());
        self
    }

    pub fn add(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::ADD.into());
        self
    }

    pub fn sub(&mut self) -> &mut Self {
        self.data.put_u8(Instruction::SUB.into());
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::runtime::lib::DiceString;

    #[test]
    fn same_string_is_inserted_into_constants_only_once() {
        let first_string = Value::new(DiceString::from("test"));
        let second_string = Value::new(DiceString::from("test"));
        let mut builder = ByteCode::builder();
        builder.push_const(first_string);
        builder.push_const(second_string);
        let bytecode = builder.build();

        assert_eq!(1, bytecode.constants.len());
        assert_eq!(18, bytecode.data.remaining());
    }
}
