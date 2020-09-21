use crate::Symbol;

use super::Compiler;

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

impl Compiler {
    // TODO: Can this be moved to the compiler stack itself?
    // ^ This might make some of this logic and borrowing easier.
    pub(super) fn resolve_upvalue(&mut self, name: Symbol, depth: usize) -> Option<usize> {
        let parent_local = self.compiler_stack.offset(depth + 1)?.scope_stack().local(name.clone());
        let descriptor = match parent_local {
            Some(parent_local) => {
                parent_local.is_captured = true;

                UpvalueDescriptor::ParentLocal {
                    slot: parent_local.slot,
                    is_mutable: parent_local.is_mutable(),
                }
            }
            None => {
                let outer_index = self.resolve_upvalue(name, depth + 1)?;
                let parent = self.compiler_stack.offset(depth + 1)?;
                let is_mutable = match parent.upvalues()[outer_index] {
                    UpvalueDescriptor::ParentLocal { is_mutable, .. } | UpvalueDescriptor::Outer { is_mutable, .. } => {
                        is_mutable
                    }
                };

                UpvalueDescriptor::Outer {
                    upvalue_index: outer_index,
                    is_mutable,
                }
            }
        };

        let current = self.compiler_stack.offset(depth)?;
        Some(current.add_upvalue(descriptor))
    }
}
