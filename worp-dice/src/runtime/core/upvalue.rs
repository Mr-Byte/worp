use std::{cell::RefCell, rc::Rc};

use crate::Value;

#[derive(Debug)]
pub enum UpvalueState {
    Open(usize),
    Closed(Rc<RefCell<Value>>),
}
