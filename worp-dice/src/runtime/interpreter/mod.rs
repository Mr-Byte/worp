#[macro_use]
mod macros;

pub(crate) mod bytecode;
pub(crate) mod callframe;
pub(crate) mod instruction;
pub(crate) mod script;

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
use script::Script;
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
    pub fn run_script(&mut self, mut script: Script) -> Result<Value, RuntimeError> {
        let stack_frame = self.stack.len()..(script.call_frame().slot_count + self.stack.len());
        let slot_count = script.call_frame().slot_count;

        self.stack.reserve_slots(slot_count);

        let result = self.execute_bytecode(&script.bytecode(), stack_frame);

        self.stack.release_slots(slot_count);

        println!("{:?}", self.stack);

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
                    let items = self.stack.pop_count(count).to_vec();

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

        // TODO: Make it an error for the stack to be empty at the end of execution.
        // Also assert that the stack hasn't underflowed into the call frame.
        println!("{:?}", self.stack);

        Ok(self.stack.pop())
    }
}

const MAX_STACK_SIZE: usize = 8;

#[derive(Debug)]
struct Stack {
    values: [Value; MAX_STACK_SIZE],
    stack_ptr: usize,
}

// TODO: Enforce stack overflows and underflows.
impl Stack {
    fn push(&mut self, value: Value) {
        self.values[self.stack_ptr] = value;
        self.stack_ptr += 1;
    }

    fn pop(&mut self) -> Value {
        let value = std::mem::replace(&mut self.values[self.stack_ptr - 1], Value::NONE);
        self.stack_ptr -= 1;

        value
    }

    fn pop_count(&mut self, count: usize) -> &mut [Value] {
        let items = &mut self.values[self.stack_ptr - count..self.stack_ptr];
        self.stack_ptr -= count;
        items
    }

    fn reserve_slots(&mut self, count: usize) {
        self.stack_ptr += count;
    }

    fn release_slots(&mut self, count: usize) {
        self.stack_ptr -= count;
    }

    fn slots(&mut self, slots: Range<usize>) -> &mut [Value] {
        &mut self.values[slots]
    }

    fn len(&self) -> usize {
        self.stack_ptr
    }

    fn top(&mut self) -> &mut Value {
        &mut self.values[self.stack_ptr - 1]
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            values: [Value::NONE; MAX_STACK_SIZE],
            stack_ptr: 0,
        }
    }
}
