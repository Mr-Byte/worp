use super::{
    assembler::Assembler,
    scope_stack::{ScopeKind, ScopeStack},
    upvalue::UpvalueDescriptor,
};
use crate::{runtime::interpreter::bytecode::Bytecode, CompilerError, Symbol};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CompilerKind {
    Script,
    Module,
    Function,
}

pub struct CompilerContext {
    kind: CompilerKind,
    assembler: Assembler,
    upvalues: Vec<UpvalueDescriptor>,
    scope_stack: ScopeStack,
}

impl CompilerContext {
    pub fn new(kind: CompilerKind) -> Self {
        Self {
            assembler: Assembler::default(),
            scope_stack: ScopeStack::new(ScopeKind::Block),
            upvalues: Vec::new(),
            kind,
        }
    }

    pub fn assembler(&mut self) -> &mut Assembler {
        &mut self.assembler
    }

    pub fn scope_stack(&mut self) -> &mut ScopeStack {
        &mut self.scope_stack
    }

    pub fn upvalues(&mut self) -> &mut Vec<UpvalueDescriptor> {
        &mut self.upvalues
    }

    pub fn add_upvalue(&mut self, descriptor: UpvalueDescriptor) -> usize {
        let index = match self.upvalues.iter().position(|upvalue| *upvalue == descriptor) {
            Some(position) => position,
            None => {
                self.upvalues.push(descriptor);
                self.upvalues.len() - 1
            }
        };

        index
    }

    pub fn kind(&self) -> CompilerKind {
        self.kind
    }

    pub fn finish(mut self) -> Bytecode {
        let slot_count = self.scope_stack.slot_count;
        let upvalue_count = self.upvalues().len();
        self.assembler.generate(slot_count, upvalue_count)
    }
}

pub struct CompilerStack {
    stack: Vec<CompilerContext>,
}

impl CompilerStack {
    pub fn new(kind: CompilerKind) -> Self {
        Self {
            stack: vec![CompilerContext::new(kind)],
        }
    }

    pub fn push(&mut self, kind: CompilerKind) {
        self.stack.push(CompilerContext::new(kind));
    }

    pub fn pop(&mut self) -> Result<CompilerContext, CompilerError> {
        self.stack
            .pop()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Compiler stack cannot be empty.")))
    }

    pub fn top_mut(&mut self) -> Result<&mut CompilerContext, CompilerError> {
        self.stack
            .last_mut()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Compiler stack cannot be empty.")))
    }

    pub fn offset(&mut self, offset: usize) -> Option<&mut CompilerContext> {
        if offset >= self.stack.len() {
            return None;
        }

        let index = self.stack.len() - offset - 1;
        self.stack.get_mut(index)
    }

    pub fn resolve_upvalue(&mut self, name: Symbol, depth: usize) -> Option<usize> {
        let parent_local = self.offset(depth + 1)?.scope_stack().local(name.clone());
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
                let parent = self.offset(depth + 1)?;
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

        let current = self.offset(depth)?;
        Some(current.add_upvalue(descriptor))
    }
}
