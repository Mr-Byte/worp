#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UpvalueDescriptor {
    ParentLocal { slot: usize, is_mutable: bool },
    Outer { upvalue_index: usize, is_mutable: bool },
}

impl UpvalueDescriptor {
    pub fn is_mutable(&self) -> bool {
        matches!(self,
            UpvalueDescriptor::ParentLocal { is_mutable, .. } | UpvalueDescriptor::Outer { is_mutable, .. } if *is_mutable
        )
    }

    pub fn description(&self) -> (bool, usize) {
        match self {
            UpvalueDescriptor::ParentLocal { slot, .. } => (true, *slot),
            UpvalueDescriptor::Outer { upvalue_index, .. } => (false, *upvalue_index),
        }
    }
}
