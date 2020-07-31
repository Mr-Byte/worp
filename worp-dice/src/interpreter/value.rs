use crate::expression::ObjectKey;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Object(HashMap<ObjectKey, Value>),
    // TODO: Create a function representation
    Function(),
}
