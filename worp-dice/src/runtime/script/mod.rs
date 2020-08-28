mod module;
mod script;

pub use module::*;
pub use script::*;

#[derive(Default, Debug)]
pub struct CallFrame {
    pub slot_count: usize,
}
