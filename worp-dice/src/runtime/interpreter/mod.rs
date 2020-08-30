#[macro_use]
mod macros;

use super::{bytecode::Bytecode, error::RuntimeError, instruction::Instruction, script::Script};
use crate::runtime::core::{
    symbol::common::operators::{
        OP_ADD, OP_DIV, OP_EQ, OP_GT, OP_GTE, OP_LT, OP_LTE, OP_MUL, OP_NEG, OP_NEQ, OP_NOT, OP_REM, OP_SUB,
    },
    Value, ValueKey,
};
use std::{iter, ops::Range};

pub struct Runtime {
    stack: Vec<Value>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            stack: Vec::with_capacity(512),
        }
    }
}

impl Runtime {
    pub fn run_script(&mut self, mut script: Script) -> Result<Value, RuntimeError> {
        // TODO: Move stack frame handling off into separate methods called by execute_bytecode.
        let stack_frame = self.stack.len()..(script.call_frame().slot_count + self.stack.len());
        let locals = iter::repeat(Value::NONE).take(script.call_frame().slot_count);
        self.stack.extend(locals);

        let result = self.execute_bytecode(&script.bytecode(), stack_frame)?;

        self.stack.truncate(self.stack.len() - script.call_frame().slot_count);

        Ok(result)
    }

    fn execute_bytecode(&mut self, bytecode: &Bytecode, stack_frame: Range<usize>) -> Result<Value, RuntimeError> {
        let mut cursor = bytecode.cursor();

        while let Some(instruction) = cursor.read_instruction() {
            match instruction {
                Instruction::PUSH_NONE => {
                    self.stack.push(Value::NONE);
                }
                Instruction::PUSH_UNIT => {
                    self.stack.push(Value::UNIT);
                }
                Instruction::PUSH_FALSE => self.stack.push(Value::Bool(false)),
                Instruction::PUSH_TRUE => self.stack.push(Value::Bool(true)),
                Instruction::PUSH_INT => {
                    let int = cursor.read_int();
                    self.stack.push(Value::Int(int));
                }
                Instruction::PUSH_FLOAT => {
                    let float = cursor.read_float();
                    self.stack.push(Value::Float(float));
                }
                Instruction::PUSH_CONST => {
                    let const_pos = cursor.read_int();
                    let value = bytecode.constants()[const_pos as usize].clone();
                    self.stack.push(value);
                }

                Instruction::POP => {
                    self.stack.pop();
                }
                Instruction::DUP => {
                    let value = self.stack.last().ok_or_else(|| RuntimeError::StackUnderflowed)?.clone();
                    self.stack.push(value);
                }

                Instruction::NEG => unary_op!(cursor, self.stack, OP_NEG),
                Instruction::NOT => unary_op!(cursor, self.stack, OP_NOT),

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
                        Some(Value::Bool(value)) => value,
                        _ => todo!(),
                    };

                    if !value {
                        cursor.offset_position(offset)
                    }
                }

                Instruction::LOAD_LOCAL => {
                    // TODO Bounds check the slot?
                    let slot = cursor.read_offset() as usize;
                    let frame = &self.stack[stack_frame.clone()];
                    let value = frame[slot].clone();
                    self.stack.push(value);
                }
                Instruction::STORE_LOCAL => {
                    // TODO Bounds check the slot?
                    let slot = cursor.read_offset() as usize;
                    let value = self.stack.pop().unwrap(); //.ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let frame = &mut self.stack[stack_frame.clone()];

                    frame[slot] = value;
                }

                unknown => return Err(RuntimeError::UnknownInstruction(unknown.into())),
            }
        }

        // TODO: Make it an error for the stack to be empty at the end of execution.
        // Also assert that the stack hasn't underflowed into the call frame.
        Ok(self.stack.pop().unwrap_or(Value::NONE))
    }
}
