use std::{cell::RefCell, rc::Rc};

use crate::Value;

#[derive(Debug)]
pub enum Upvalue {
    Open(usize),
    Closed(Rc<RefCell<Value>>),
}
