use super::{bytecode::Bytecode, instruction::Instruction, script::Script};
use crate::runtime::{
    core::{
        symbol::common::operators::{
            OP_ADD, OP_DIV, OP_EQ, OP_GT, OP_GTE, OP_LT, OP_LTE, OP_MUL, OP_NEG, OP_NEQ, OP_NOT, OP_REM, OP_SUB,
        },
        Value, ValueKey,
    },
    error::{RuntimeError, Spanned as _, SpannedRuntimeError},
};
use std::{iter, ops::Range};

macro_rules! binary_op {
    ($bytecode:expr, $stack:expr, $op:ident) => {{
        let lhs = $stack
            .pop()
            .ok_or_else(|| RuntimeError::StackUnderflowed)
            .with_span(|| $bytecode.span())?;
        let rhs = $stack
            .pop()
            .ok_or_else(|| RuntimeError::StackUnderflowed)
            .with_span(|| $bytecode.span())?;
        let result = lhs
            .get(&ValueKey::Symbol($op))
            .with_span(|| $bytecode.span())?
            .call(&[lhs, rhs])
            .with_span(|| $bytecode.span())?;
        $stack.push(result);
    }};
}

macro_rules! unary_op {
    ($bytecode:expr, $stack:expr, $op:expr) => {{
        let value = $stack
            .pop()
            .ok_or_else(|| RuntimeError::StackUnderflowed)
            .with_span(|| $bytecode.span())?;
        let result = value
            .get(&ValueKey::Symbol($op))
            .with_span(|| $bytecode.span())?
            .call(&[value])
            .with_span(|| $bytecode.span())?;
        $stack.push(result);
    }};
}

#[derive(Default)]
pub struct Runtime {
    stack: Vec<Value>,
}

impl Runtime {
    pub fn run_script(&mut self, mut script: Script) -> Result<Value, SpannedRuntimeError> {
        let locals_frame = self.stack.len()..script.call_frame().slot_count;
        let locals = iter::repeat(Value::NONE).take(script.call_frame().slot_count);
        self.stack.extend(locals);

        let result = self.execute_bytecode(script.bytecode().clone(), locals_frame)?;
        self.stack.truncate(self.stack.len() - script.call_frame().slot_count);

        Ok(result)
    }

    fn execute_bytecode(
        &mut self,
        mut bytecode: Bytecode,
        locals_frame: Range<usize>,
    ) -> Result<Value, SpannedRuntimeError> {
        while let Some(instruction) = bytecode.read_instruction() {
            match instruction {
                Instruction::PUSH_NONE => {
                    self.stack.push(Value::NONE);
                }
                Instruction::PUSH_FALSE => self.stack.push(Value::new(false)),
                Instruction::PUSH_TRUE => self.stack.push(Value::new(true)),
                Instruction::PUSH_INT => {
                    let int = bytecode.read_int();
                    self.stack.push(Value::new(int));
                }
                Instruction::PUSH_FLOAT => {
                    let float = bytecode.read_float();
                    self.stack.push(Value::new(float));
                }
                Instruction::PUSH_CONST => {
                    let const_pos = bytecode.read_int();
                    let value = bytecode.constants()[const_pos as usize].clone();
                    self.stack.push(value);
                }

                Instruction::POP => {
                    self.stack.pop();
                }
                Instruction::DUP => {
                    let value = self
                        .stack
                        .last()
                        .ok_or_else(|| RuntimeError::StackUnderflowed)
                        .with_span(|| bytecode.span())?
                        .clone();
                    self.stack.push(value);
                }

                Instruction::NEG => unary_op!(bytecode, self.stack, OP_NEG),
                Instruction::NOT => unary_op!(bytecode, self.stack, OP_NOT),

                Instruction::MUL => binary_op!(bytecode, self.stack, OP_MUL),
                Instruction::DIV => binary_op!(bytecode, self.stack, OP_DIV),
                Instruction::REM => binary_op!(bytecode, self.stack, OP_REM),
                Instruction::ADD => binary_op!(bytecode, self.stack, OP_ADD),
                Instruction::SUB => binary_op!(bytecode, self.stack, OP_SUB),

                Instruction::GT => binary_op!(bytecode, self.stack, OP_GT),
                Instruction::GTE => binary_op!(bytecode, self.stack, OP_GTE),
                Instruction::LT => binary_op!(bytecode, self.stack, OP_LT),
                Instruction::LTE => binary_op!(bytecode, self.stack, OP_LTE),
                Instruction::EQ => binary_op!(bytecode, self.stack, OP_EQ),
                Instruction::NEQ => binary_op!(bytecode, self.stack, OP_NEQ),
                Instruction::HALT => return Ok(self.stack.pop().unwrap_or(Value::NONE)),

                Instruction::JUMP => {
                    let offset = bytecode.read_offset();
                    bytecode.offset_position(offset)
                }
                Instruction::JUMP_IF_FALSE => {
                    let offset = bytecode.read_offset();

                    let value = *self
                        .stack
                        .pop()
                        .unwrap_or(Value::NONE)
                        .try_value::<bool>()
                        .with_span(|| bytecode.span())?;

                    if !value {
                        bytecode.offset_position(offset)
                    }
                }

                Instruction::LOAD_LOCAL => {
                    // TODO Bounds check the slot?
                    let slot = bytecode.read_offset() as usize;
                    let frame = &self.stack[locals_frame.clone()];
                    let value = frame[slot].clone();
                    self.stack.push(value);
                }
                Instruction::STORE_LOCAL => {
                    // TODO Bounds check the slot?
                    let slot = bytecode.read_offset() as usize;
                    let value = self
                        .stack
                        .pop()
                        .ok_or_else(|| RuntimeError::StackUnderflowed)
                        .with_span(|| bytecode.span())?;
                    let frame = &mut self.stack[locals_frame.clone()];

                    frame[slot] = value;
                }

                unknown => return Err(RuntimeError::UnknownInstruction(unknown.into())).with_span(|| bytecode.span()),
            }
        }

        Ok(self.stack.pop().unwrap_or(Value::NONE))
    }
}
