use crate::CompilerError;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(super) enum ScopeKind {
    Module,
    Script,
    Function,
    Block,
    Loop,
}

pub(super) struct ScopeContext {
    pub depth: usize,
    pub kind: ScopeKind,
}

pub(super) struct ScopeStack {
    stack: Vec<ScopeKind>,
}

impl ScopeStack {
    pub fn new(kind: ScopeKind) -> Self {
        Self { stack: vec![kind] }
    }

    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.stack.push(kind);
    }

    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }

    pub fn context(&self) -> Result<ScopeContext, CompilerError> {
        self.stack
            .last()
            .copied()
            .map(|kind| ScopeContext {
                kind,
                depth: self.stack.len() - 1,
            })
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }
}
