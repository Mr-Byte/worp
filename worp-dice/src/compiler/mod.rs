use crate::{
    runtime::{
        core::Symbol,
        interpreter::{callframe::CallFrame, script::Script},
    },
    syntax::{Parser, SyntaxTree},
};
use bytecode::BytecodeGenerator;
use error::CompilerError;
use scope::{ScopeKind, ScopeStack};

pub mod error;

mod bytecode;
mod expression;
mod scope;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum CompilationKind {
    Module,
    Script,
    Function,
}

pub struct Compiler {
    syntax_tree: SyntaxTree,
    bytecode: BytecodeGenerator,
    call_frame: CallFrame,
    scope_stack: ScopeStack,
    locals: Vec<Local>,
    kind: CompilationKind,
}

impl Compiler {
    fn new(syntax_tree: SyntaxTree, kind: CompilationKind) -> Self {
        let scope_kind = match kind {
            CompilationKind::Module => ScopeKind::Module,
            CompilationKind::Script => ScopeKind::Script,
            CompilationKind::Function => ScopeKind::Function,
        };

        Self {
            syntax_tree,
            kind,
            bytecode: BytecodeGenerator::default(),
            call_frame: CallFrame::default(),
            locals: Vec::default(),
            scope_stack: ScopeStack::new(scope_kind),
        }
    }

    pub fn compile_script(input: &str) -> Result<Script, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self::new(syntax_tree, CompilationKind::Script);
        compiler.compile(compiler.syntax_tree.root())?;

        let script = Script::new(compiler.bytecode.generate(), compiler.call_frame);

        Ok(script)
    }

    pub(self) fn begin_scope(&mut self, kind: ScopeKind) {
        self.scope_stack.push_scope(kind);
    }

    pub(self) fn end_scope(&mut self) -> Result<(), CompilerError> {
        while let Some(scope_depth) = self.locals.last().map(|local| local.scope_depth) {
            if scope_depth < self.scope_stack.context()?.depth {
                break;
            }

            self.locals.pop();
        }

        self.scope_stack.pop_scope();

        Ok(())
    }

    pub(self) fn add_local(&mut self, name: Symbol, is_mutable: bool) -> Result<u8, CompilerError> {
        let slot = self.locals.len();

        let local = Local {
            name,
            is_mutable,
            scope_depth: self.scope_stack.context()?.depth,
            slot: self.locals.len() as u8,
        };

        self.locals.push(local);

        // Increment slot count if the slot's index is greater than or equal to that of the slot count.
        if slot >= self.call_frame.slot_count {
            self.call_frame.slot_count = slot + 1;
        }

        Ok(slot as u8)
    }

    pub(self) fn local(&self, name: Symbol) -> Result<&Local, CompilerError> {
        for local in self.locals.iter().rev() {
            if local.name == name {
                return Ok(local);
            }
        }

        Err(CompilerError::UndeclaredVariable(name))
    }
}

struct Local {
    name: Symbol,
    scope_depth: usize,
    slot: u8,
    is_mutable: bool,
}
