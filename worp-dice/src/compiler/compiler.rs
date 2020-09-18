use super::{assembler::Assembler, scope::ScopeKind, scope::ScopeStack, upvalue::UpvalueDescriptor};
use crate::{runtime::interpreter::bytecode::Bytecode, CompilerError};

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

    pub fn finish(self) -> Bytecode {
        let slot_count = self.scope_stack.slot_count;
        self.assembler.generate(slot_count)
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

    pub fn offset_with_parent(
        &mut self,
        offset: usize,
    ) -> (Option<&mut CompilerContext>, Option<&mut CompilerContext>) {
        let mut iter = self.stack.iter_mut();

        (iter.nth_back(offset), iter.nth_back(0))
    }
}
