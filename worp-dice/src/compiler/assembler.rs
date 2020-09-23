use crate::{
    runtime::{
        core::{Span, Value},
        interpreter::{bytecode::Bytecode, instruction::Instruction},
    },
    CompilerError,
};
use bytes::BufMut as _;
use std::collections::HashMap;

use super::upvalue::UpvalueDescriptor;

#[derive(Default)]
pub struct Assembler {
    constants: Vec<Value>,
    source_map: HashMap<u64, Span>,
    data: Vec<u8>,
}

impl Assembler {
    pub fn generate(self, slot_count: usize, upvalue_count: usize) -> Bytecode {
        Bytecode::new(
            self.data.into(),
            slot_count,
            upvalue_count,
            self.constants.into_boxed_slice(),
            self.source_map,
        )
    }

    pub fn push_none(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_NONE.value());
    }

    pub fn push_unit(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_UNIT.value());
    }

    pub fn push_bool(&mut self, value: bool, span: Span) {
        let instruction = if value {
            Instruction::PUSH_TRUE
        } else {
            Instruction::PUSH_FALSE
        };

        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(instruction.value());
    }

    pub fn push_i0(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_I0.value());
    }

    pub fn push_i1(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_I1.value());
    }

    pub fn push_f0(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_F0.value());
    }

    pub fn push_f1(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_F1.value());
    }

    pub fn push_const(&mut self, value: Value, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_CONST.value());

        self.source_map.insert(self.data.len() as u64, span);
        let const_pos = self.make_constant(value)?;
        self.data.put_u8(const_pos);

        Ok(())
    }

    pub fn closure(&mut self, value: Value, upvalues: &[UpvalueDescriptor], span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CLOSURE.value());
        let fn_pos = self.make_constant(value)?;
        self.data.put_u8(fn_pos);

        if upvalues.len() > 255 {
            return Err(CompilerError::TooManyUpvalues);
        }

        for upvalue in upvalues {
            let (is_parent_local, index) = upvalue.description();

            self.data.put_u8(is_parent_local as u8);
            self.data.put_u8(index as u8);
        }

        Ok(())
    }

    pub fn pop(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::POP.value());
    }

    pub fn dup(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DUP.value());
    }

    pub fn build_list(&mut self, length: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::BUILD_LIST.value());
        self.data.put_u8(length);
    }

    pub fn mul(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::MUL.value());
    }

    pub fn div(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DIV.value());
    }

    pub fn rem(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::REM.value());
    }

    pub fn add(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::ADD.value());
    }

    pub fn sub(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::SUB.value());
    }

    pub fn eq(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::EQ.value());
    }

    pub fn neq(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NEQ.value());
    }

    pub fn gt(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::GT.value());
    }

    pub fn gte(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::GTE.value());
    }

    pub fn lt(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LT.value());
    }

    pub fn lte(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LTE.value());
    }

    pub fn neg(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NEG.value());
    }

    pub fn not(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NOT.value());
    }

    #[must_use = "Jumps must be patched."]
    pub fn jump(&mut self, span: Span) -> u64 {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::JUMP.value());

        self.data.put_i16(0);
        let patch_pos = self.data.len() - 2;

        patch_pos as u64
    }

    #[must_use = "Jumps must be patched."]
    pub fn jump_if_false(&mut self, span: Span) -> u64 {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::JUMP_IF_FALSE.value());

        self.data.put_i16(0);
        let patch_pos = self.data.len() - 2;

        patch_pos as u64
    }

    pub fn patch_jump(&mut self, jump_position: u64) {
        let offset = (self.current_position() - jump_position - 2) as i16;
        (&mut self.data[jump_position as usize..]).put_i16(offset)
    }

    pub fn jump_back(&mut self, position: u64, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::JUMP.value());
        let offset = -((self.current_position() - position + 2) as i16);
        self.data.put_i16(offset);
    }

    pub fn current_position(&self) -> u64 {
        (self.data.len()) as u64
    }

    pub fn store_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::STORE_LOCAL.value());
        self.data.put_u8(slot);
    }

    pub fn load_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LOAD_LOCAL.value());
        self.data.put_u8(slot);
    }

    pub fn store_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::STORE_UPVALUE.value());
        self.data.put_u8(index);
    }

    pub fn load_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LOAD_UPVALUE.value());
        self.data.put_u8(index);
    }

    pub fn close_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CLOSE_UPVALUE.value());
        self.data.put_u8(index);
    }

    pub fn load_global(&mut self, global: Value, span: Span) -> Result<(), CompilerError> {
        let const_slot = self.make_constant(global)?;

        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LOAD_GLOBAL.value());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn call(&mut self, arg_count: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CALL.value());
        self.data.put_u8(arg_count);
    }

    pub fn ret(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::RETURN.value());
    }

    fn make_constant(&mut self, value: Value) -> Result<u8, CompilerError> {
        let position = if let Some(position) = self.constants.iter().position(|current| *current == value) {
            position
        } else {
            self.constants.push(value);
            self.constants.len() - 1
        };

        // NOTE: This could be alleviated by offering a long-form PUSH_CONST.
        if position > 255 {
            return Err(CompilerError::TooManyConstants);
        }

        Ok(position as u8)
    }
}
