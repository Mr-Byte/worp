use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use crate::Value;

#[derive(Debug)]
pub enum UpvalueState {
    Open(usize),
    Closed(Value),
}

#[derive(Clone, Debug)]
pub struct Upvalue(Rc<RefCell<UpvalueState>>);

impl Upvalue {
    pub fn new_open(slot: usize) -> Self {
        Self(Rc::new(RefCell::new(UpvalueState::Open(slot))))
    }

    pub fn close(&mut self, value: Value) {
        *self.0.borrow_mut() = UpvalueState::Closed(value);
    }

    pub fn state(&mut self) -> RefMut<'_, UpvalueState> {
        self.0.borrow_mut()
    }
}
