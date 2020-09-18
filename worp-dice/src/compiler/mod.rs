use crate::{
    runtime::interpreter::bytecode::Bytecode,
    syntax::{Parser, SyntaxNode, SyntaxTree},
    Symbol,
};
use compiler::{CompilerContext, CompilerKind, CompilerStack};
use error::CompilerError;
use visitor::{BlockKind, NodeVisitor as _};

mod assembler;
mod compiler;
pub mod error;
mod scope;
mod upvalue;
mod visitor;

#[allow(dead_code)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum CompilationKind {
    Script,
    Module,
}

pub struct Compiler {
    syntax_tree: SyntaxTree,
    compiler_stack: CompilerStack,
}

impl Compiler {
    fn new(syntax_tree: SyntaxTree, kind: CompilationKind) -> Self {
        let compiler_stack = CompilerStack::new(CompilerKind::Script);

        Self {
            syntax_tree,
            compiler_stack,
        }
    }

    pub fn compile_str(input: &str, kind: CompilationKind) -> Result<Bytecode, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self::new(syntax_tree, kind);

        compiler.visit(compiler.syntax_tree.root())?;
        let compiler_context = compiler.compiler_stack.pop()?;

        Ok(compiler_context.finish())
    }

    pub(self) fn compile_fn(
        &mut self,
        syntax_tree: SyntaxTree,
        args: &[impl AsRef<str>],
    ) -> Result<Bytecode, CompilerError> {
        // TODO: Push a new CompilerContext onto the CompilerStack.
        self.compiler_stack.push(CompilerKind::Function);

        for arg in args {
            self.compiler_stack
                .top_mut()?
                .scope_stack()
                .add_local(Symbol::new(arg.as_ref()), false)?;
        }

        let root = syntax_tree.get(syntax_tree.root()).expect("Node should not be empty");

        if let SyntaxNode::Block(body) = root {
            let body = body.clone();
            self.visit((&body, BlockKind::Function))?;
        } else {
            unreachable!("Function body must be a block.")
        }

        let compiler_context = self.compiler_stack.pop()?;

        Ok(compiler_context.finish())
    }

    pub(self) fn context(&mut self) -> Result<&mut CompilerContext, CompilerError> {
        self.compiler_stack.top_mut()
    }
}
