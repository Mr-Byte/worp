use std::ops::Range;

use crate::Value;

const MAX_STACK_SIZE: usize = 512;

pub struct Stack {
    values: [Value; MAX_STACK_SIZE],
    stack_ptr: usize,
}

// TODO: Enforce stack overflows and underflows.
impl Stack {
    pub fn push(&mut self, value: Value) {
        self.values[self.stack_ptr] = value;
        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> Value {
        let value = std::mem::replace(&mut self.values[self.stack_ptr - 1], Value::NONE);
        self.stack_ptr -= 1;

        value
    }

    pub fn pop_count(&mut self, count: usize) -> Vec<Value> {
        let mut result = Vec::with_capacity(count);
        let items_to_pop = &mut self.values[self.stack_ptr - count..self.stack_ptr];
        self.stack_ptr -= count;

        for item in items_to_pop {
            let item = std::mem::replace(item, Value::NONE);
            result.push(item);
        }

        result
    }

    pub fn reserve_slots(&mut self, count: usize) {
        self.stack_ptr += count;
    }

    pub fn release_slots(&mut self, count: usize) {
        for _ in 0..count {
            self.pop();
        }
    }

    pub fn slots(&mut self, slots: Range<usize>) -> &mut [Value] {
        &mut self.values[slots]
    }

    pub fn len(&self) -> usize {
        self.stack_ptr
    }

    pub fn top(&mut self) -> &mut Value {
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
