use self::{
    assembler::Assembler,
    scope::{ScopeKind, ScopeStack},
};
use crate::{
    runtime::interpreter::bytecode::Bytecode,
    syntax::{Parser, SyntaxTree},
    Symbol,
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
    fn try_new(
        syntax_tree: SyntaxTree,
        kind: CompilationKind,
        args: &[impl AsRef<str>],
    ) -> Result<Self, CompilerError> {
        let scope_kind = match kind {
            CompilationKind::Module => ScopeKind::Module,
            CompilationKind::Script => ScopeKind::Script,
            CompilationKind::Function => ScopeKind::Function,
        };
        let mut scope_stack = ScopeStack::new(scope_kind);

        for arg in args {
            scope_stack.add_local(Symbol::new(arg.as_ref()), false)?;
        }

        let result = Self {
            syntax_tree,
            assembler: Assembler::default(),
            scope_stack,
        };

        Ok(result)
    }

    pub fn compile(mut self) -> Result<Bytecode, CompilerError> {
        self.visit(self.syntax_tree.root())?;

        Ok(self.assembler.generate(self.scope_stack.slot_count))
    }

    pub fn try_from_str(input: &str, kind: CompilationKind) -> Result<Self, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let compiler = Self::try_new(syntax_tree, kind, &[] as &[&str])?;

        Ok(compiler)
    }
}
