use self::{
    assembler::Assembler,
    scope::{ScopeKind, ScopeStack},
};
use crate::{
    runtime::interpreter::bytecode::Bytecode,
    syntax::{Parser, SyntaxNode, SyntaxTree},
    Symbol,
};
use error::CompilerError;
use visitor::{BlockKind, NodeVisitor as _};

mod assembler;
pub mod error;
mod scope;
mod visitor;

#[allow(dead_code)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum CompilationKind {
    Script,
    Module,
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
        };
        let scope_stack = ScopeStack::new(scope_kind);

        Self {
            syntax_tree,
            assembler: Assembler::default(),
            scope_stack,
        }
    }

    pub fn compile_str(input: &str, kind: CompilationKind) -> Result<Bytecode, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self::new(syntax_tree, kind);

        compiler.visit(compiler.syntax_tree.root())?;

        Ok(compiler.assembler.generate(compiler.scope_stack.slot_count))
    }

    pub(self) fn compile_fn(
        syntax_tree: SyntaxTree,
        name: Symbol,
        args: &[impl AsRef<str>],
    ) -> Result<Bytecode, CompilerError> {
        let scope_stack = ScopeStack::new(ScopeKind::Function);
        let mut compiler = Self {
            syntax_tree,
            scope_stack,
            assembler: Assembler::default(),
        };

        compiler.scope_stack.add_local(name, false)?;

        for arg in args {
            compiler.scope_stack.add_local(Symbol::new(arg.as_ref()), false)?;
        }

        let root = compiler
            .syntax_tree
            .get(compiler.syntax_tree.root())
            .expect("Node should not be empty");

        if let SyntaxNode::Block(body) = root {
            let body = body.clone();
            compiler.visit((&body, BlockKind::Function))?;
        } else {
            unreachable!("Function body must be a block.")
        }

        Ok(compiler.assembler.generate(compiler.scope_stack.slot_count))
    }
}
