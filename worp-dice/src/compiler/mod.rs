use crate::{
    runtime::{
        core::Symbol,
        interpreter::{callframe::CallFrame, script::Script},
    },
    syntax::{Parser, SyntaxTree},
    SyntaxError,
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
    Script,
    Module,
    Function,
}

pub enum CompilationUnit {
    Script(Script),
    Module,
    Function,
}

pub struct Compiler {
    syntax_tree: SyntaxTree,
    bytecode: BytecodeGenerator,
    scope_stack: ScopeStack,
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
            scope_stack: ScopeStack::new(scope_kind),
        }
    }

    pub fn compile(mut self) -> Result<CompilationUnit, CompilerError> {
        self.expression(self.syntax_tree.root())?;

        let call_frame = CallFrame {
            slot_count: self.scope_stack.slot_count,
        };

        let compilation_unit = match self.kind {
            CompilationKind::Script => CompilationUnit::Script(Script::new(self.bytecode.generate(), call_frame)),
            _ => todo!(),
        };

        Ok(compilation_unit)
    }

    pub fn try_from_str(input: &str, kind: CompilationKind) -> Result<Self, SyntaxError> {
        let syntax_tree = Parser::new(input).parse()?;
        let compiler = Self::new(syntax_tree, kind);

        Ok(compiler)
    }
}
