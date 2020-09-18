use crate::Symbol;

use super::Compiler;

#[derive(PartialEq, Eq)]
pub enum UpvalueDescriptor {
    ParentLocal { slot: usize, is_mutable: bool },
    Outer { upvalue_index: usize },
}

impl Compiler {
    pub(self) fn resolve_upvalue(&mut self, name: Symbol, depth: usize) -> Option<usize> {
        let descriptor_index = {
            let (current, parent) = self.compiler_stack.offset_with_parent(depth);
            let current = current?;
            let parent = parent?;
            let parent_local = parent.scope_stack().local(name.clone());

            parent_local.map(move |parent_local| {
                parent_local.is_captured = true;

                let descriptor = UpvalueDescriptor::ParentLocal {
                    slot: parent_local.slot,
                    is_mutable: parent_local.is_mutable,
                };

                current.add_upvalue(descriptor)
            })
        };

        descriptor_index.or_else(|| {
            let outer_index = self.resolve_upvalue(name, depth + 1)?;
            let descriptor = UpvalueDescriptor::Outer {
                upvalue_index: outer_index,
            };

            let (current, _) = self.compiler_stack.offset_with_parent(depth);
            let current = current?;
            Some(current.add_upvalue(descriptor))
        })
    }
}
