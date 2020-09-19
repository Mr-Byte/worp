#[macro_use]
mod macros;
mod stack;

pub(crate) mod bytecode;
pub(crate) mod instruction;

use crate::{
    runtime::{core::Value, interpreter::bytecode::BytecodeCursor},
    RuntimeError,
};
use bytecode::Bytecode;
use instruction::Instruction;
use stack::Stack;
use std::ops::Range;

use super::{core::Upvalue, core::UpvalueState, lib::FnClosure};

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
        let stack_frame = self.stack.reserve_slots(bytecode.slot_count());
        let result = self.execute_bytecode(&bytecode, stack_frame, None);
        self.stack.release_slots(bytecode.slot_count());

        Ok(result?)
    }

    fn execute_bytecode(
        &mut self,
        bytecode: &Bytecode,
        stack_frame: Range<usize>,
        mut closure: Option<FnClosure>,
    ) -> Result<Value, RuntimeError> {
        let initial_stack_depth = self.stack.len();
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
                    let value = self.stack.peek(0).clone();
                    self.stack.push(value);
                }

                Instruction::BUILD_LIST => {
                    let count = cursor.read_u8() as usize;
                    let items = self.stack.pop_count(count);

                    self.stack.push(Value::List(items.into()));
                }

                Instruction::NEG => match self.stack.peek(0) {
                    Value::Int(value) => *value = -*value,
                    Value::Float(value) => *value = -*value,
                    _ => todo!(),
                },
                Instruction::NOT => match self.stack.peek(0) {
                    Value::Bool(value) => *value = !*value,
                    _ => todo!(),
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
                Instruction::LOAD_UPVALUE => {
                    if let Some(closure) = closure.as_mut() {
                        let upvalue_slot = cursor.read_u8() as usize;
                        let upvalue = &mut closure.borrow_mut().upvalues[upvalue_slot];
                        let value = match upvalue.state() {
                            UpvalueState::Open(slot) => self.stack.slot(*slot).clone(),
                            _ => todo!(),
                        };

                        self.stack.push(value);
                    } else {
                        unreachable!("LOAD_UPVALUE used in non-closure context.")
                    }
                }
                Instruction::STORE_UPVALUE => {
                    if let Some(closure) = closure.as_mut() {
                        let upvalue_slot = cursor.read_u8() as usize;
                        let upvalue = &mut closure.borrow_mut().upvalues[upvalue_slot];
                        let result = match upvalue.state() {
                            UpvalueState::Open(slot) => {
                                let value = self.stack.pop();
                                *self.stack.slot(*slot) = value.clone();
                                value
                            }
                            _ => todo!(),
                        };

                        self.stack.push(result)
                    } else {
                        unreachable!("STORE_UPVALUE used in non-closure context.")
                    }
                }
                Instruction::MUL_ASSIGN_LOCAL => {
                    let slot = cursor.read_u8() as usize;
                    let value = self.stack.pop();
                    let frame = self.stack.slots(stack_frame.clone());

                    match (&mut frame[slot], value) {
                        (Value::Int(lhs), Value::Int(rhs)) => *lhs *= rhs,
                        (Value::Float(lhs), Value::Float(rhs)) => *lhs *= rhs,
                        _ => todo!(),
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
                        _ => todo!(),
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
                        _ => todo!(),
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
                        _ => todo!(),
                    }

                    let result = frame[slot].clone();
                    self.stack.push(result);
                }

                Instruction::CLOSURE => {
                    let const_pos = cursor.read_u8() as usize;

                    match bytecode.constants()[const_pos] {
                        Value::FnScript(ref fn_script) => {
                            let upvalue_count = fn_script.bytecode.upvalue_count();
                            let mut upvalues = Vec::with_capacity(upvalue_count);

                            for _ in 0..upvalue_count {
                                let is_parent_local = cursor.read_u8() == 1;
                                let index = cursor.read_u8() as usize;

                                if is_parent_local {
                                    upvalues.push(Upvalue::new_open(stack_frame.start + index));
                                } else {
                                    todo!();
                                }
                            }

                            let closure =
                                Value::FnClosure(FnClosure::new(fn_script.clone(), upvalues.into_boxed_slice()));
                            self.stack.push(closure);
                        }
                        _ => return Err(RuntimeError::NotAFunction),
                    }
                }

                Instruction::CALL => {
                    self.call_fn(&mut cursor)?;
                }

                Instruction::RETURN => {
                    break;
                }

                unknown => return Err(RuntimeError::UnknownInstruction(unknown.value())),
            }
        }

        // NOTE: subtract 1 to compensate for the last item of the stack not yet being popped.
        let final_stack_depth = self.stack.len() - 1;

        assert!(
            initial_stack_depth == final_stack_depth,
            "Stack was left in a bad state. Initial depth {}, final depth {}",
            initial_stack_depth,
            final_stack_depth
        );

        Ok(self.stack.pop())
    }

    fn call_fn(&mut self, cursor: &mut BytecodeCursor<'_>) -> Result<(), RuntimeError> {
        let arg_count = cursor.read_u8() as usize;
        let mut target = self.stack.peek(arg_count).clone();

        match &mut target {
            Value::FnClosure(closure) => {
                let bytecode = {
                    let fn_script = &closure.borrow().fn_script;
                    // TODO: Make this a RuntimeError
                    if arg_count != fn_script.arity {
                        return Err(RuntimeError::InvalidFunctionArgs(fn_script.arity, arg_count));
                    }

                    fn_script.bytecode.clone()
                };

                let slots = bytecode.slot_count();
                let reserved = slots - arg_count;
                // NOTE: Reserve only the slots needed to cover locals beyond the arguments already on the stack.
                let stack_frame = self.stack.reserve_slots(reserved);
                let stack_frame = (stack_frame.start - arg_count)..stack_frame.end;
                let result = self.execute_bytecode(&bytecode, stack_frame, Some(closure.clone()))?;

                // NOTE: Release the number of reserved slots plus the number of arguments plus a slot for the function itself.
                self.stack.release_slots(reserved + arg_count + 1);
                self.stack.push(result);
            }
            _ => return Err(RuntimeError::NotAFunction),
        }

        Ok(())
    }
}
