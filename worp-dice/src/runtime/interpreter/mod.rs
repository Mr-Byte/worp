#[macro_use]
mod macros;
mod stack;

pub(crate) mod bytecode;
pub(crate) mod callframe;
pub(crate) mod instruction;

use crate::{
    runtime::core::{
        symbol::common::operators::{
            OP_ADD, OP_DIV, OP_EQ, OP_GT, OP_GTE, OP_LT, OP_LTE, OP_MUL, OP_NEG, OP_NEQ, OP_NOT, OP_REM, OP_SUB,
        },
        Value, ValueKey,
    },
    RuntimeError,
};
use bytecode::Bytecode;
use instruction::Instruction;
use stack::Stack;
use std::ops::Range;

pub struct Runtime {
    stack: Stack,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            stack: Stack::default(),
        }
    }
}

impl Runtime {
    pub fn run_script(&mut self, bytecode: Bytecode) -> Result<Value, RuntimeError> {
        let slot_count = bytecode.call_frame().slot_count;
        let stack_frame_start = self.stack.len();
        let stack_frame_end = stack_frame_start + slot_count;
        let stack_frame = stack_frame_start..stack_frame_end;

        self.stack.reserve_slots(slot_count);

        let result = self.execute_bytecode(&bytecode, stack_frame);

        self.stack.release_slots(slot_count);

        Ok(result?)
    }

    fn execute_bytecode(&mut self, bytecode: &Bytecode, stack_frame: Range<usize>) -> Result<Value, RuntimeError> {
        let mut cursor = bytecode.cursor();

        while let Some(instruction) = cursor.read_instruction() {
            match instruction {
                Instruction::PUSH_NONE => self.stack.push(Value::NONE),
                Instruction::PUSH_UNIT => self.stack.push(Value::UNIT),
                Instruction::PUSH_FALSE => self.stack.push(Value::Bool(false)),
                Instruction::PUSH_TRUE => self.stack.push(Value::Bool(true)),
                Instruction::PUSH_I0 => self.stack.push(Value::Int(0)),
                Instruction::PUSH_I1 => self.stack.push(Value::Int(1)),
                Instruction::PUSH_F0 => self.stack.push(Value::Float(0.0)),
                Instruction::PUSH_F1 => self.stack.push(Value::Float(1.0)),
                Instruction::PUSH_CONST => {
                    let const_pos = cursor.read_u8() as usize;
                    let value = bytecode.constants()[const_pos].clone();
                    self.stack.push(value);
                }

                Instruction::POP => {
                    self.stack.pop();
                }
                Instruction::DUP => {
                    let value = self.stack.top().clone();
                    self.stack.push(value);
                }

                Instruction::BUILD_LIST => {
                    let count = cursor.read_u8() as usize;
                    let items = self.stack.pop_count(count);

                    self.stack.push(Value::List(items.into()));
                }

                Instruction::NEG => match self.stack.top() {
                    Value::Int(value) => *value = -*value,
                    Value::Float(value) => *value = -*value,
                    value => *value = value.get(&ValueKey::Symbol(OP_NEG))?.call(&[value.clone()])?,
                },
                Instruction::NOT => match self.stack.top() {
                    Value::Bool(value) => *value = !*value,
                    value => *value = value.get(&ValueKey::Symbol(OP_NOT))?.call(&[value.clone()])?,
                },

                Instruction::MUL => arithmetic_op!(self.stack, OP_MUL),
                Instruction::DIV => arithmetic_op!(self.stack, OP_DIV),
                Instruction::REM => arithmetic_op!(self.stack, OP_REM),
                Instruction::ADD => arithmetic_op!(self.stack, OP_ADD),
                Instruction::SUB => arithmetic_op!(self.stack, OP_SUB),

                Instruction::GT => comparison_op!(self.stack, OP_GT),
                Instruction::GTE => comparison_op!(self.stack, OP_GTE),
                Instruction::LT => comparison_op!(self.stack, OP_LT),
                Instruction::LTE => comparison_op!(self.stack, OP_LTE),
                Instruction::EQ => comparison_op!(self.stack, OP_EQ),
                Instruction::NEQ => comparison_op!(self.stack, OP_NEQ),

                Instruction::JUMP => {
                    let offset = cursor.read_offset();
                    cursor.offset_position(offset)
                }
                Instruction::JUMP_IF_FALSE => {
                    let offset = cursor.read_offset();
                    let value = match self.stack.pop() {
                        Value::Bool(value) => value,
                        _ => todo!(),
                    };

                    if !value {
                        cursor.offset_position(offset)
                    }
                }

                Instruction::LOAD_LOCAL => {
                    // TODO Bounds check the slot?
                    let slot = cursor.read_u8() as usize;
                    let frame = self.stack.slots(stack_frame.clone());
                    let value = frame[slot].clone();
                    self.stack.push(value);
                }
                Instruction::STORE_LOCAL => {
                    // TODO Bounds check the slot?
                    let slot = cursor.read_u8() as usize;
                    let value = self.stack.pop();
                    let frame = self.stack.slots(stack_frame.clone());

                    frame[slot] = value;
                    let result = frame[slot].clone();
                    self.stack.push(result);
                }
                Instruction::MUL_ASSIGN_LOCAL => {
                    let slot = cursor.read_u8() as usize;
                    let value = self.stack.pop();
                    let frame = self.stack.slots(stack_frame.clone());

                    match (&mut frame[slot], value) {
                        (Value::Int(lhs), Value::Int(rhs)) => *lhs *= rhs,
                        (Value::Float(lhs), Value::Float(rhs)) => *lhs *= rhs,
                        (lhs, rhs) => *lhs = lhs.get(&ValueKey::Symbol(OP_MUL))?.call(&[lhs.clone(), rhs])?,
                    }

                    let result = frame[slot].clone();
                    self.stack.push(result);
                }
                Instruction::DIV_ASSIGN_LOCAL => {
                    let slot = cursor.read_u8() as usize;
                    let value = self.stack.pop();
                    let frame = self.stack.slots(stack_frame.clone());

                    match (&mut frame[slot], value) {
                        (Value::Int(lhs), Value::Int(rhs)) => *lhs /= rhs,
                        (Value::Float(lhs), Value::Float(rhs)) => *lhs /= rhs,
                        (lhs, rhs) => *lhs = lhs.get(&ValueKey::Symbol(OP_DIV))?.call(&[lhs.clone(), rhs])?,
                    }

                    let result = frame[slot].clone();
                    self.stack.push(result);
                }
                Instruction::ADD_ASSIGN_LOCAL => {
                    let slot = cursor.read_u8() as usize;
                    let value = self.stack.pop();
                    let frame = self.stack.slots(stack_frame.clone());

                    match (&mut frame[slot], value) {
                        (Value::Int(lhs), Value::Int(rhs)) => *lhs += rhs,
                        (Value::Float(lhs), Value::Float(rhs)) => *lhs += rhs,
                        (lhs, rhs) => *lhs = lhs.get(&ValueKey::Symbol(OP_ADD))?.call(&[lhs.clone(), rhs])?,
                    }

                    let result = frame[slot].clone();
                    self.stack.push(result);
                }
                Instruction::SUB_ASSIGN_LOCAL => {
                    let slot = cursor.read_u8() as usize;
                    let value = self.stack.pop();
                    let frame = self.stack.slots(stack_frame.clone());

                    match (&mut frame[slot], value) {
                        (Value::Int(lhs), Value::Int(rhs)) => *lhs -= rhs,
                        (Value::Float(lhs), Value::Float(rhs)) => *lhs -= rhs,
                        (lhs, rhs) => *lhs = lhs.get(&ValueKey::Symbol(OP_SUB))?.call(&[lhs.clone(), rhs])?,
                    }

                    let result = frame[slot].clone();
                    self.stack.push(result);
                }

                unknown => return Err(RuntimeError::UnknownInstruction(unknown.value())),
            }
        }

        Ok(self.stack.pop())
    }
}
