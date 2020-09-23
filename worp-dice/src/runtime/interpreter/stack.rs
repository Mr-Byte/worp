use std::ops::Range;

use crate::Value;

// NOTE: When sizeof(Value) = 16-bytes, this is 1MB of stack space.
const MAX_STACK_SIZE: usize = 65_536;

pub struct Stack {
    values: Box<[Value]>,
    stack_ptr: usize,
}

// TODO: Enforce stack overflows and underflows.
impl Stack {
    #[inline]
    pub fn push(&mut self, value: Value) {
        self.values[self.stack_ptr] = value;
        self.stack_ptr += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> Value {
        self.stack_ptr -= 1;
        std::mem::replace(&mut self.values[self.stack_ptr], Value::NONE)
    }

    pub fn pop_count(&mut self, count: usize) -> Vec<Value> {
        let mut result = vec![Value::NONE; count];
        let items_to_pop = &mut self.values[self.stack_ptr - count..self.stack_ptr];
        self.stack_ptr -= count;

        for index in (0..items_to_pop.len()).rev() {
            std::mem::swap(&mut items_to_pop[index], &mut result[index])
        }

        result
    }

    pub fn reserve_slots(&mut self, count: usize) -> Range<usize> {
        let start = self.stack_ptr;
        let new_stack_ptr = self.stack_ptr + count;

        self.stack_ptr = new_stack_ptr;
        assert!(self.stack_ptr < MAX_STACK_SIZE, "Stack Overflowed");

        start..new_stack_ptr
    }

    pub fn release_slots(&mut self, count: usize) {
        let new_stack_ptr = self.stack_ptr - count;
        for value in &mut self.values[new_stack_ptr..self.stack_ptr] {
            *value = Value::NONE;
        }

        self.stack_ptr = new_stack_ptr;

        // NOTE: If the stack ptr is greater than the stack size, the stack ptr underflowed.
        assert!(self.stack_ptr < MAX_STACK_SIZE, "Stack Underflowed")
    }

    #[inline]
    pub fn slots(&mut self, slots: Range<usize>) -> &mut [Value] {
        &mut self.values[slots]
    }

    #[inline]
    pub fn slot(&mut self, slot: usize) -> &mut Value {
        &mut self.values[slot]
    }

    // NOTE: Returns the value offset from the top of the stack.
    #[inline]
    pub fn peek(&mut self, offset: usize) -> &mut Value {
        &mut self.values[self.stack_ptr - offset - 1]
    }

    pub fn peek_n(&mut self, offset: usize) -> &mut [Value] {
        let start = self.stack_ptr - offset - 1;
        let end = self.stack_ptr;

        &mut self.values[start..end]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.stack_ptr
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            values: vec![Value::NONE; MAX_STACK_SIZE].into_boxed_slice(),
            stack_ptr: 0,
        }
    }
}
