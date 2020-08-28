use super::{bytecode::Bytecode, instruction::Instruction, script::Script};
use crate::runtime::{
    core::{
        symbol::common::operators::{
            OP_ADD, OP_DIV, OP_EQ, OP_GT, OP_GTE, OP_LT, OP_LTE, OP_MUL, OP_NEG, OP_NEQ, OP_NOT, OP_REM, OP_SUB,
        },
        Value, ValueKey,
    },
    error::RuntimeError,
};
use std::{iter, ops::Range};

macro_rules! binary_op {
    ($bytecode:expr, $stack:expr, $op:ident) => {{
        let lhs = $stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
        let rhs = $stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
        let result = lhs.get(&ValueKey::Symbol($op))?.call(&[lhs, rhs])?;

        $stack.push(result);
    }};
}

macro_rules! unary_op {
    ($bytecode:expr, $stack:expr, $op:expr) => {{
        let value = $stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
        let result = value.get(&ValueKey::Symbol($op))?.call(&[value])?;
        $stack.push(result);
    }};
}

#[derive(Default)]
pub struct Runtime {
    stack: Vec<Value>,
}

impl Runtime {
    pub fn run_script(&mut self, mut script: Script) -> Result<Value, RuntimeError> {
        // TODO: Move stack frame handling off into separate methods called by execute_bytecode.
        let stack_frame = self.stack.len()..script.call_frame().slot_count;
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
                Instruction::PUSH_FALSE => self.stack.push(Value::new(false)),
                Instruction::PUSH_TRUE => self.stack.push(Value::new(true)),
                Instruction::PUSH_INT => {
                    let int = cursor.read_int();
                    self.stack.push(Value::new(int));
                }
                Instruction::PUSH_FLOAT => {
                    let float = cursor.read_float();
                    self.stack.push(Value::new(float));
                }
                Instruction::PUSH_CONST => {
                    let const_pos = cursor.read_int();
                    let value = bytecode.constants()[const_pos as usize].clone();
                    self.stack.push(value);
                }

                Instruction::POP => {
                    self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    // .with_span(|| bytecode.span())?;
                }
                Instruction::DUP => {
                    let value = self
                        .stack
                        .last()
                        .ok_or_else(|| RuntimeError::StackUnderflowed)?
                        // .with_span(|| bytecode.span())?
                        .clone();
                    self.stack.push(value);
                }

                Instruction::NEG => unary_op!(cursor, self.stack, OP_NEG),
                Instruction::NOT => unary_op!(cursor, self.stack, OP_NOT),

                Instruction::MUL => binary_op!(cursor, self.stack, OP_MUL),
                Instruction::DIV => binary_op!(cursor, self.stack, OP_DIV),
                Instruction::REM => binary_op!(cursor, self.stack, OP_REM),
                Instruction::ADD => binary_op!(cursor, self.stack, OP_ADD),
                Instruction::SUB => binary_op!(cursor, self.stack, OP_SUB),

                Instruction::GT => binary_op!(cursor, self.stack, OP_GT),
                Instruction::GTE => binary_op!(cursor, self.stack, OP_GTE),
                Instruction::LT => binary_op!(cursor, self.stack, OP_LT),
                Instruction::LTE => binary_op!(cursor, self.stack, OP_LTE),
                Instruction::EQ => binary_op!(cursor, self.stack, OP_EQ),
                Instruction::NEQ => binary_op!(cursor, self.stack, OP_NEQ),
                Instruction::HALT => return Ok(self.stack.pop().unwrap_or(Value::NONE)),

                Instruction::JUMP => {
                    let offset = cursor.read_offset();
                    cursor.offset_position(offset)
                }
                Instruction::JUMP_IF_FALSE => {
                    let offset = cursor.read_offset();
                    let value = *self.stack.pop().unwrap_or(Value::NONE).try_value::<bool>()?;

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
                    let value = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
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
