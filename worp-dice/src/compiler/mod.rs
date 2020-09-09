use self::{
    assembler::Assembler,
    scope::{ScopeKind, ScopeStack},
};
use crate::{
    runtime::interpreter::bytecode::Bytecode,
    runtime::interpreter::callframe::CallFrame,
    syntax::{Parser, SyntaxTree},
    SyntaxError,
};
use error::CompilerError;
use visitor::NodeVisitor as _;

mod assembler;
pub mod error;
mod scope;
mod visitor;

#[allow(dead_code)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum CompilationKind {
    Script,
    Module,
    Function,
}

pub struct Compiler {
    syntax_tree: SyntaxTree,
    assembler: Assembler,
    scope_stack: ScopeStack,
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
            assembler: Assembler::default(),
            scope_stack: ScopeStack::new(scope_kind),
        }
    }

    pub fn compile(mut self) -> Result<Bytecode, CompilerError> {
        self.visit(self.syntax_tree.root())?;

        let call_frame = CallFrame {
            slot_count: self.scope_stack.slot_count,
        };

        Ok(self.assembler.generate(call_frame))
    }

    pub fn try_from_str(input: &str, kind: CompilationKind) -> Result<Self, SyntaxError> {
        let syntax_tree = Parser::new(input).parse()?;
        let compiler = Self::new(syntax_tree, kind);

        Ok(compiler)
    }
}
