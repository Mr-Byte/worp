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

impl ScopeKind {
    fn is_terminal(self) -> bool {
        matches!(self, ScopeKind::Script | ScopeKind::Module | ScopeKind::Function)
    }

    fn is_block(self) -> bool {
        matches!(self, ScopeKind::Block | ScopeKind::Loop)
    }

    fn is_loop(self) -> bool {
        self == ScopeKind::Loop
    }
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

    pub fn pop_scope(&mut self) -> Result<ScopeContext, CompilerError> {
        self.stack
            .pop()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }

    pub fn first_of_kind(&self, kind: ScopeKind) -> Option<&ScopeContext> {
        for context in self.stack.iter().rev() {
            if context.kind == kind {
                return Some(context);
            } else if kind.is_block() && context.kind.is_terminal() {
                return None;
            }
        }

        None
    }

    pub fn first_of_kind_mut(&mut self, kind: ScopeKind) -> Option<&mut ScopeContext> {
        for context in self.stack.iter_mut().rev() {
            if context.kind == kind {
                return Some(context);
            } else if kind.is_block() && context.kind.is_terminal() {
                return None;
            }
        }

        None
    }

    pub fn in_context_of(&self, kind: ScopeKind) -> bool {
        self.first_of_kind(kind).is_some()
    }

    pub fn add_local(&mut self, name: Symbol, is_mutable: bool) -> Result<usize, CompilerError> {
        self.top_mut()?.slot_count += 1;

        let slot_count = self.stack.iter().map(|scope| scope.slot_count).sum::<usize>();
        let slot = slot_count - 1;
        let local = ScopeVariable { name, is_mutable, slot };

        self.top_mut()?.variables.push(local);

        if slot_count > self.slot_count {
            self.slot_count = slot_count;
        }

        Ok(slot)
    }

    pub fn local(&self, name: Symbol) -> Result<ScopeVariable, CompilerError> {
        self.stack
            .iter()
            .rev()
            .flat_map(|scope| scope.variables.iter().rev())
            .find(|local| local.name == name)
            .cloned()
            .ok_or_else(|| CompilerError::UndeclaredVariable(name))
    }

    pub fn top_mut(&mut self) -> Result<&mut ScopeContext, CompilerError> {
        self.stack
            .last_mut()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }

    pub fn top(&self) -> Result<&ScopeContext, CompilerError> {
        self.stack
            .last()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }

    /// Push the bytecode location of an exit point to the inner most loop's scope, to later be patched.
    pub fn add_loop_exit_point(&mut self, exit_point: usize) -> Result<(), CompilerError> {
        let scope = self.first_of_kind_mut(ScopeKind::Loop).ok_or_else(|| todo!())?;

        scope.exit_points.push(exit_point);

        Ok(())
    }

    /// Set the bytecode location of the entry point to the inner most loop's scope.
    pub fn set_loop_entry_point(&mut self, entry_point: usize) -> Result<(), CompilerError> {
        let scope = self.first_of_kind_mut(ScopeKind::Loop).ok_or_else(|| todo!())?;

        scope.entry_point = Some(entry_point);

        Ok(())
    }

    pub fn loop_entry_point(&mut self) -> Result<usize, CompilerError> {
        let scope = self.first_of_kind(ScopeKind::Loop).ok_or_else(|| todo!())?;

        scope
            .entry_point
            .clone()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Not in a loop context.")))
    }
}
