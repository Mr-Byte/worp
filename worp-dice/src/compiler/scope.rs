use crate::{CompilerError, Symbol};

#[derive(Clone)]
pub struct ScopeVariable {
    pub name: Symbol,
    pub is_mutable: bool,
    pub slot: usize,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(super) enum ScopeKind {
    Script,
    Module,
    Function,
    Block,
    Loop,
}

// TODO: Move variables and variable resolution to scope context.
#[derive(Clone)]
pub(super) struct ScopeContext {
    pub depth: usize,
    pub kind: ScopeKind,
    pub entry_point: Option<usize>,
    pub exit_points: Vec<usize>,
    pub variables: Vec<ScopeVariable>,
    slot_count: usize,
}

impl Default for ScopeContext {
    fn default() -> Self {
        Self {
            depth: 0,
            kind: ScopeKind::Script,
            entry_point: None,
            exit_points: Vec::new(),
            variables: Vec::new(),
            slot_count: 0,
        }
    }
}

pub(super) struct ScopeStack {
    stack: Vec<ScopeContext>,
    pub slot_count: usize,
}

impl ScopeStack {
    pub fn new(kind: ScopeKind) -> Self {
        Self {
            stack: vec![ScopeContext {
                kind,
                depth: 0,
                ..Default::default()
            }],
            slot_count: 0,
        }
    }

    // TODO: Include a parameter for an optional entry point into a scope.
    // TODO: Include a way to append an exit point to the nearest loop scope.
    //       These exit points will be patched by the compiler when the scope is finished compiling.
    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.stack.push(ScopeContext {
            kind,
            depth: self.stack.len(),
            ..Default::default()
        });
    }

    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }

    pub fn context(&mut self) -> Result<ScopeContext, CompilerError> {
        self.top().map(|context| context.clone())
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

    pub fn add_local(&mut self, name: Symbol, is_mutable: bool) -> Result<usize, CompilerError> {
        self.top()?.slot_count += 1;

        let slot_count = self.stack.iter().map(|scope| scope.slot_count).sum::<usize>();
        let slot = slot_count - 1;
        let local = ScopeVariable { name, is_mutable, slot };

        self.top()?.variables.push(local);

        if slot_count > self.slot_count {
            self.slot_count = slot_count;
        }

        Ok(slot)
    }

    pub fn local(&mut self, name: Symbol) -> Result<ScopeVariable, CompilerError> {
        self.stack
            .iter()
            .rev()
            .flat_map(|scope| scope.variables.iter().rev())
            .find(|local| local.name == name)
            .cloned()
            .ok_or_else(|| CompilerError::UndeclaredVariable(name.clone()))
    }

    fn top(&mut self) -> Result<&mut ScopeContext, CompilerError> {
        self.stack
            .last_mut()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }
}
