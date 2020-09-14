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
    scope_stack: ScopeStack,
    assembler_stack: Vec<Assembler>,
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
            scope_stack,
            assembler_stack: vec![Assembler::default()],
        }
    }

    pub fn compile_str(input: &str, kind: CompilationKind) -> Result<Bytecode, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self::new(syntax_tree, kind);

        compiler.visit(compiler.syntax_tree.root())?;

        Ok(compiler.pop_assembler().generate(compiler.scope_stack.slot_count))
    }

    pub(self) fn compile_fn(
        &mut self,
        syntax_tree: SyntaxTree,
        name: Symbol,
        args: &[impl AsRef<str>],
    ) -> Result<Bytecode, CompilerError> {
        self.push_assembler();
        self.scope_stack.push_scope(ScopeKind::Function, None);
        self.scope_stack.add_local(name, false)?;

        for arg in args {
            self.scope_stack.add_local(Symbol::new(arg.as_ref()), false)?;
        }

        let root = syntax_tree.get(syntax_tree.root()).expect("Node should not be empty");

        if let SyntaxNode::Block(body) = root {
            let body = body.clone();
            self.visit((&body, BlockKind::Function))?;
        } else {
            unreachable!("Function body must be a block.")
        }

        self.scope_stack.pop_scope()?;

        Ok(self.pop_assembler().generate(self.scope_stack.slot_count))
    }

    pub(self) fn current_assembler(&mut self) -> &mut Assembler {
        self.assembler_stack
            .last_mut()
            .expect("Assembler stack should not be empty.")
    }

    pub(self) fn pop_assembler(&mut self) -> Assembler {
        self.assembler_stack
            .pop()
            .expect("Assembler stack should not be empty.")
    }

    pub(self) fn push_assembler(&mut self) {
        self.assembler_stack.push(Assembler::default())
    }
}
