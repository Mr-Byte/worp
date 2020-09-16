use crate::{CompilerError, Symbol};

#[derive(Clone)]
pub struct ScopeVariable {
    pub name: Symbol,
    pub slot: usize,
    pub is_mutable: bool,
    pub is_captured: bool,
}

#[derive(Clone, PartialEq)]
pub enum ScopeUpvalue {
    /// This upvalue points to a value in the parent scope.
    Parent { slot: usize },
    /// This upvalue points to a value in the grandparent scope.
    Outer { upvalue_slot: usize },
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ScopeKind {
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
}

// TODO: Move variables and variable resolution to scope context.
#[derive(Clone)]
pub struct ScopeContext {
    pub depth: usize,
    pub kind: ScopeKind,
    pub entry_point: Option<usize>,
    pub exit_points: Vec<usize>,
    pub variables: Vec<ScopeVariable>,
    pub upvalues: Vec<ScopeUpvalue>,
    slot_count: usize,
}

// impl ScopeContext {
//     /// Get a list of captured variables. Used while closing any upvalues from the scope.
//     pub fn captured_variables(&self) -> impl Iterator<Item = &ScopeVariable> {
//         self.variables.iter().filter(|variable| variable.is_captured)
//     }
// }

impl Default for ScopeContext {
    fn default() -> Self {
        Self {
            depth: 0,
            kind: ScopeKind::Script,
            entry_point: None,
            exit_points: Vec::new(),
            variables: Vec::new(),
            upvalues: Vec::new(),
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

    /// Start a new scope of the specified kind, with an optional entry point.
    pub fn push_scope(&mut self, kind: ScopeKind, entry_point: Option<usize>) {
        self.stack.push(ScopeContext {
            kind,
            depth: self.stack.len(),
            entry_point,
            ..Default::default()
        });
    }

    /// Pop the top scope off the scope stack and return it.
    pub fn pop_scope(&mut self) -> Result<ScopeContext, CompilerError> {
        self.stack
            .pop()
            .ok_or_else(|| CompilerError::InternalCompilerError(String::from("Scope stack underflowed.")))
    }

    /// Determine whether or not if the stack currently contains a context of the specified scope type.
    /// If the specified scope is of type Loop, Block, or If and a Function, Script, or Module boundary is encountered
    /// before the specified scope can be found, this function returns false.
    pub fn in_context_of(&self, kind: ScopeKind) -> bool {
        self.first_of_kind(kind).is_some()
    }

    /// Add a new local variable to the top level scope.
    pub fn add_local(&mut self, name: Symbol, is_mutable: bool) -> Result<usize, CompilerError> {
        self.top_mut()?.slot_count += 1;

        // TODO: Start at the top and work down until the first terminal scope is encountered.
        let mut slot_count = 0;

        for scope in self.stack.iter().rev() {
            slot_count += scope.slot_count;

            if scope.kind.is_terminal() {
                break;
            }
        }

        let slot = slot_count - 1;
        let local = ScopeVariable {
            name,
            is_mutable,
            slot,
            is_captured: false,
        };

        self.top_mut()?.variables.push(local);

        if slot_count > self.slot_count {
            self.slot_count = slot_count;
        }

        Ok(slot)
    }

    /// Find the first local variable with the specified name, starting at the top of the stack and working towards the bottom.
    /// This searches each scope in reverse order of variable declarations, so that the most recently used declaration is
    /// return first.
    pub fn local(&self, name: Symbol) -> Option<&ScopeVariable> {
        for scope in self.stack.iter().rev() {
            for variable in scope.variables.iter().rev() {
                if variable.name == name {
                    return Some(variable);
                }
            }

            if scope.kind.is_terminal() {
                return None;
            }
        }

        return None;
    }

    pub fn add_upvalue(&self, upvalue: ScopeUpvalue) -> Result<usize, CompilerError> {
        // Find the first function scope and insert this upvalue into it, if it does not already contain the upvalue.
        Ok(0)
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

    /// Get all terminal scopes on the stack, ordered top to bottom.
    fn terminal_scopes_mut(&mut self) -> impl Iterator<Item = &mut ScopeContext> {
        self.stack.iter_mut().rev().filter(|scope| scope.kind.is_terminal())
    }

    /// Find the first scope of the specified kind.
    /// If the specified scope is of type Loop, Block, or If and a Function, Script, or Module boundary is encountered
    /// before the specified scope can be found, this function returns None.
    fn first_of_kind(&self, kind: ScopeKind) -> Option<&ScopeContext> {
        for context in self.stack.iter().rev() {
            if context.kind == kind {
                return Some(context);
            } else if kind.is_block() && context.kind.is_terminal() {
                return None;
            }
        }

        None
    }

    /// Find the first scope of the specified kind as a mutable reference.
    /// If the specified scope is of type Loop, Block, or If and a Function, Script, or Module boundary is encountered
    /// before the specified scope can be found, this function returns None.
    fn first_of_kind_mut(&mut self, kind: ScopeKind) -> Option<&mut ScopeContext> {
        for context in self.stack.iter_mut().rev() {
            if context.kind == kind {
                return Some(context);
            } else if kind.is_block() && context.kind.is_terminal() {
                return None;
            }
        }

        None
    }

    fn outer_scopes_mut(&mut self) -> impl Iterator<Item = &mut ScopeContext> {
        // NOTE: Start from the top of the stack and skip every scope inside the current terminal scope.
        self.stack
            .iter_mut()
            .rev()
            .skip_while(|scope| !scope.kind.is_terminal())
            .skip(1)
    }

    /// Search for a variable that exist in any scope outside the current terminal scope and return a mutable reference to it.
    fn outer_local_mut(&mut self, name: Symbol) -> Option<&mut ScopeVariable> {
        self.outer_scopes_mut()
            .flat_map(|scope| scope.variables.iter_mut().rev())
            .find(|local| local.name == name)
    }
}
