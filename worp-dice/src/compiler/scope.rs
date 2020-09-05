use crate::CompilerError;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(super) enum ScopeKind {
    Module,
    Script,
    Function,
    Block,
    Loop,
}

#[derive(Clone)]
pub(super) struct ScopeContext {
    pub depth: usize,
    pub kind: ScopeKind,
    pub exit_points: Vec<u64>,
}

pub(super) struct ScopeStack {
    stack: Vec<ScopeContext>,
}

impl ScopeStack {
    pub fn new(kind: ScopeKind) -> Self {
        Self {
            stack: vec![ScopeContext {
                kind,
                depth: 0,
                exit_points: Vec::new(),
            }],
        }
    }

    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.stack.push(ScopeContext {
            kind,
            depth: self.stack.len(),
            exit_points: Vec::new(),
        });
    }

    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }

    pub fn context(&self) -> Result<ScopeContext, CompilerError> {
        self.stack
            .last()
            .cloned()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }

    pub fn in_context_of(&self, kind: ScopeKind) -> bool {
        for context in self.stack.iter().rev() {
            if context.kind == kind {
                return true;
            }

            // If the kind being searched for is a loop or block and a script, module, or function is reached, terminate the scope kind search.
            if matches!(kind, ScopeKind::Loop | ScopeKind::Block)
                && matches!(
                    context.kind,
                    ScopeKind::Script | ScopeKind::Module | ScopeKind::Function
                )
            {
                return false;
            }
        }

        false
    }
}
