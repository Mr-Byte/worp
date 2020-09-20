use crate::{CompilerError, Symbol};

#[derive(Clone)]
pub struct ScopeVariable {
    pub name: Symbol,
    pub slot: usize,
    pub is_captured: bool,
    // NOTE: Unlike variables, functions cannot shadow.
    pub state: State,
}

impl ScopeVariable {
    pub fn is_mutable(&self) -> bool {
        matches!(self.state, State::Local { is_mutable, .. } if is_mutable)
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self.state, State::Local { is_initialized, .. } | State::Function { is_initialized } if is_initialized)
    }
}

#[derive(Clone)]
pub enum State {
    Local { is_mutable: bool, is_initialized: bool },
    Function { is_initialized: bool },
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ScopeKind {
    Block,
    Loop,
}

// TODO: Move variables and variable resolution to scope context.
#[derive(Clone)]
pub struct ScopeContext {
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
            kind: ScopeKind::Block,
            entry_point: None,
            exit_points: Vec::new(),
            variables: Vec::new(),
            slot_count: 0,
        }
    }
}

pub struct ScopeStack {
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

    pub fn push_scope(&mut self, kind: ScopeKind, entry_point: Option<usize>) {
        self.stack.push(ScopeContext {
            kind,
            depth: self.stack.len(),
            entry_point,
            ..Default::default()
        });
    }

    pub fn pop_scope(&mut self) -> Result<ScopeContext, CompilerError> {
        self.stack
            .pop()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }

    pub fn in_context_of(&self, kind: ScopeKind) -> bool {
        self.first_of_kind(kind).is_some()
    }

    pub fn add_local(&mut self, name: Symbol, state: State) -> Result<usize, CompilerError> {
        self.top_mut()?.slot_count += 1;

        let mut slot_count = 0;

        // TODO: Revert this back to fancy sum()
        for scope in self.stack.iter().rev() {
            slot_count += scope.slot_count;
        }

        let slot = slot_count - 1;
        let local = ScopeVariable {
            name,
            slot,
            is_captured: false,
            state,
        };

        self.top_mut()?.variables.push(local);

        if slot_count > self.slot_count {
            self.slot_count = slot_count;
        }

        Ok(slot)
    }

    pub fn local(&mut self, name: Symbol) -> Option<&mut ScopeVariable> {
        self.stack
            .iter_mut()
            .rev()
            .flat_map(|scope| scope.variables.iter_mut().rev())
            .find(|var| var.name == name)
    }

    pub fn top_mut(&mut self) -> Result<&mut ScopeContext, CompilerError> {
        self.stack
            .last_mut()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }

    /// Push the bytecode location of an exit point to the inner most loop's scope, to later be patched.
    pub fn add_loop_exit_point(&mut self, exit_point: usize) -> Result<(), CompilerError> {
        let scope = self.first_of_kind_mut(ScopeKind::Loop).expect("Add error here.");

        scope.exit_points.push(exit_point);

        Ok(())
    }

    /// Get the entry point of the first scope to match the specified kind.
    pub fn entry_point(&mut self, kind: ScopeKind) -> Result<usize, CompilerError> {
        let scope = self.first_of_kind(kind).expect("Add error here.");

        scope
            .entry_point
            .clone()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Not in a loop context.")))
    }

    fn first_of_kind(&self, kind: ScopeKind) -> Option<&ScopeContext> {
        self.stack.iter().rev().find(|scope| scope.kind == kind)
    }

    fn first_of_kind_mut(&mut self, kind: ScopeKind) -> Option<&mut ScopeContext> {
        self.stack.iter_mut().rev().find(|scope| scope.kind == kind)
    }
}
