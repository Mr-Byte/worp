use std::{cell::RefCell, rc::Rc};

use crate::Value;

#[derive(Debug)]
pub enum UpvalueState {
    Open(usize),
    Closed(Rc<RefCell<Value>>),
}

#[derive(Debug)]
pub struct Upvalue(UpvalueState);

impl Upvalue {
    pub fn new_open(slot: usize) -> Self {
        Self(UpvalueState::Open(slot))
    }

    pub fn state(&mut self) -> &mut UpvalueState {
        &mut self.0
    }
}
