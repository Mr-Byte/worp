#[macro_use]
mod macros;
mod stack;

pub(crate) mod bytecode;
pub(crate) mod instruction;

use crate::{
    runtime::{core::Value, interpreter::bytecode::BytecodeCursor},
    RuntimeError, Symbol,
};
use bytecode::Bytecode;
use instruction::Instruction;
use stack::Stack;
use std::{collections::HashMap, collections::VecDeque, ops::Range};

use super::{
    core::{Upvalue, UpvalueState},
    lib::FnClosure,
    lib::FnNative,
    lib::NativeFn,
};

#[derive(Default)]
pub struct Runtime {
    stack: Stack,
    open_upvalues: VecDeque<Upvalue>,
    globals: HashMap<String, Value>,
}

impl Runtime {
    pub fn run_script(&mut self, bytecode: Bytecode) -> Result<Value, RuntimeError> {
        // self.globals
        //     .insert(String::from("print"), Value::FnNative(FnNative::new(print_value)));

        let stack_frame = self.stack.reserve_slots(bytecode.slot_count());
        let result = self.execute_bytecode(&bytecode, stack_frame, None);
        self.stack.release_slots(bytecode.slot_count());

        Ok(result?)
    }

    pub fn register_native_fn(&mut self, name: String, native_fn: NativeFn) {
        self.globals.insert(name, Value::FnNative(FnNative::new(native_fn)));
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
                Instruction::PUSH_CONST => self.push_const(bytecode, &mut cursor),
                Instruction::POP => {
                    self.stack.pop();
                }
                Instruction::DUP => {
                    let value = self.stack.peek(0).clone();
                    self.stack.push(value);
                }
                Instruction::BUILD_LIST => self.build_list(&mut cursor),

                Instruction::NEG => self.neg(),
                Instruction::NOT => self.not(),

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
                    cursor.offset_position(offset);
                }
                Instruction::JUMP_IF_FALSE => self.jump_if_false(&mut cursor),

                Instruction::LOAD_LOCAL => self.load_local(&stack_frame, &mut cursor),
                Instruction::STORE_LOCAL => self.store_local(&stack_frame, &mut cursor),
                Instruction::LOAD_UPVALUE => self.load_upvalue(&mut closure, &mut cursor),
                Instruction::STORE_UPVALUE => self.store_upvalue(&mut closure, &mut cursor),
                Instruction::CLOSE_UPVALUE => self.close_upvalue(&stack_frame, &mut cursor),
                Instruction::LOAD_GLOBAL => self.load_global(bytecode, &mut cursor)?,

                Instruction::CLOSURE => self.closure(bytecode, &stack_frame, &mut closure, &mut cursor)?,
                Instruction::CALL => self.call_fn(&mut cursor)?,
                Instruction::RETURN => break,

                unknown => return Err(RuntimeError::UnknownInstruction(unknown.value())),
            }
        }

        // NOTE: subtract 1 to compensate for the last item of the stack not yet being popped.
        let final_stack_depth = self.stack.len() - 1;

        assert_eq!(
            initial_stack_depth, final_stack_depth,
            "Stack was left in a bad state. Initial depth {}, final depth {}",
            initial_stack_depth, final_stack_depth
        );

        Ok(self.stack.pop())
    }

    fn not(&mut self) {
        match self.stack.peek(0) {
            Value::Bool(value) => *value = !*value,
            _ => todo!(),
        }
    }

    fn neg(&mut self) {
        match self.stack.peek(0) {
            Value::Int(value) => *value = -*value,
            Value::Float(value) => *value = -*value,
            _ => todo!(),
        }
    }

    fn build_list(&mut self, cursor: &mut BytecodeCursor) {
        let count = cursor.read_u8() as usize;
        let items = self.stack.pop_count(count);

        self.stack.push(Value::List(items.into()));
    }

    fn push_const(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) {
        let const_pos = cursor.read_u8() as usize;
        let value = bytecode.constants()[const_pos].clone();
        self.stack.push(value);
    }

    fn jump_if_false(&mut self, cursor: &mut BytecodeCursor) {
        let offset = cursor.read_offset();
        let value = match self.stack.pop() {
            Value::Bool(value) => value,
            _ => unreachable!("JUMP_IF_FALSE requires a boolean operand."),
        };

        if !value {
            cursor.offset_position(offset)
        }
    }

    fn load_local(&mut self, stack_frame: &Range<usize>, cursor: &mut BytecodeCursor) {
        // TODO Bounds check the slot?
        let slot = cursor.read_u8() as usize;
        let frame = self.stack.slots(stack_frame.clone());
        let value = frame[slot].clone();
        self.stack.push(value);
    }

    fn store_local(&mut self, stack_frame: &Range<usize>, cursor: &mut BytecodeCursor) {
        // TODO Bounds check the slot?
        let slot = cursor.read_u8() as usize;
        let value = self.stack.pop();
        let frame = self.stack.slots(stack_frame.clone());

        frame[slot] = value;
        let result = frame[slot].clone();
        self.stack.push(result);
    }

    fn load_upvalue(&mut self, closure: &mut Option<FnClosure>, cursor: &mut BytecodeCursor) {
        if let Some(closure) = closure.as_mut() {
            let upvalue_slot = cursor.read_u8() as usize;
            let upvalue = &mut closure.borrow_mut().upvalues[upvalue_slot];
            let value = match &*upvalue.state() {
                UpvalueState::Open(slot) => self.stack.slot(*slot).clone(),
                UpvalueState::Closed(value) => value.clone(),
            };

            self.stack.push(value);
        } else {
            unreachable!("LOAD_UPVALUE used in non-closure context.")
        }
    }

    fn store_upvalue(&mut self, closure: &mut Option<FnClosure>, cursor: &mut BytecodeCursor) {
        if let Some(closure) = closure.as_mut() {
            let upvalue_slot = cursor.read_u8() as usize;
            let upvalue = &mut closure.borrow_mut().upvalues[upvalue_slot];
            let value = self.stack.pop();
            let result = match &mut *upvalue.state() {
                UpvalueState::Open(slot) => {
                    *self.stack.slot(*slot) = value.clone();
                    value
                }
                UpvalueState::Closed(closed_value) => {
                    *closed_value = value.clone();
                    value
                }
            };

            self.stack.push(result)
        } else {
            unreachable!("STORE_UPVALUE used in non-closure context.")
        }
    }

    fn close_upvalue(&mut self, stack_frame: &Range<usize>, cursor: &mut BytecodeCursor) {
        let offset = cursor.read_u8() as usize;
        let value = std::mem::replace(&mut self.stack.slots(stack_frame.clone())[offset], Value::NONE);
        let offset = stack_frame.start + offset;
        let mut found_index = None;

        for (index, upvalue) in self.open_upvalues.iter_mut().enumerate() {
            if let UpvalueState::Open(upvalue_offset) = &*upvalue.state() {
                if *upvalue_offset == offset {
                    found_index = Some(index);
                }
            }
        }

        if let Some(index) = found_index {
            if let Some(mut upvalue) = self.open_upvalues.remove(index) {
                upvalue.close(value);
            }
        }
    }

    fn load_global(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let const_pos = cursor.read_u8() as usize;
        let value = &bytecode.constants()[const_pos];

        if let Value::String(global) = value {
            let value = self
                .globals
                .get(global)
                .cloned()
                .ok_or_else(|| RuntimeError::VariableNotFound(Symbol::new(global)))?;

            self.stack.push(value);
        } else {
            return Err(RuntimeError::InvalidGlobalNameType);
        }

        Ok(())
    }

    fn closure(
        &mut self,
        bytecode: &Bytecode,
        stack_frame: &Range<usize>,
        closure: &mut Option<FnClosure>,
        cursor: &mut BytecodeCursor,
    ) -> Result<(), RuntimeError> {
        let const_pos = cursor.read_u8() as usize;

        match bytecode.constants()[const_pos] {
            Value::FnScript(ref fn_script) => {
                let upvalue_count = fn_script.bytecode.upvalue_count();
                let mut upvalues = Vec::with_capacity(upvalue_count);

                for _ in 0..upvalue_count {
                    let is_parent_local = cursor.read_u8() == 1;
                    let index = cursor.read_u8() as usize;

                    if is_parent_local {
                        let upvalue = Upvalue::new_open(stack_frame.start + index);
                        self.open_upvalues.push_back(upvalue.clone());
                        upvalues.push(upvalue);
                    } else if let Some(closure) = closure.as_mut() {
                        let upvalue = closure.borrow().upvalues[index].clone();
                        upvalues.push(upvalue);
                    } else {
                        // NOTE: Produce an unreachable here. This case should never execute, but this is a sanity check to ensure it doesn't.
                        unreachable!("No parent scope found.")
                    }
                }

                let closure = Value::FnClosure(FnClosure::new(fn_script.clone(), upvalues.into_boxed_slice()));
                self.stack.push(closure);
            }
            _ => return Err(RuntimeError::NotAFunction),
        }

        Ok(())
    }

    // TODO: Replace this mutually recursive call with an execution stack to prevent the thread's stack from overflowing.
    fn call_fn(&mut self, cursor: &mut BytecodeCursor<'_>) -> Result<(), RuntimeError> {
        let arg_count = cursor.read_u8() as usize;
        let mut target = self.stack.peek(arg_count).clone();

        let (bytecode, closure) = match &mut target {
            Value::FnClosure(closure) => {
                let fn_script = &closure.borrow().fn_script;

                if arg_count != fn_script.arity {
                    return Err(RuntimeError::InvalidFunctionArgs(fn_script.arity, arg_count));
                }

                (fn_script.bytecode.clone(), Some(closure.clone()))
            }
            Value::FnScript(fn_script) => {
                if arg_count != fn_script.arity {
                    return Err(RuntimeError::InvalidFunctionArgs(fn_script.arity, arg_count));
                }

                (fn_script.bytecode.clone(), None)
            }
            Value::FnNative(fn_native) => {
                let fn_native = fn_native.clone();

                let result = fn_native.call(self.stack.peek_n(arg_count - 1))?;
                self.stack.release_slots(arg_count + 1);
                self.stack.push(result);

                return Ok(());
            }
            _ => return Err(RuntimeError::NotAFunction),
        };

        let slots = bytecode.slot_count();
        let reserved = slots - arg_count;
        // NOTE: Reserve only the slots needed to cover locals beyond the arguments already on the stack.
        let stack_frame = self.stack.reserve_slots(reserved);
        let stack_frame = (stack_frame.start - arg_count)..stack_frame.end;
        let result = self.execute_bytecode(&bytecode, stack_frame, closure)?;

        // NOTE: Release the number of reserved slots plus the number of arguments plus a slot for the function itself.
        self.stack.release_slots(reserved + arg_count + 1);
        self.stack.push(result);

        Ok(())
    }
}
