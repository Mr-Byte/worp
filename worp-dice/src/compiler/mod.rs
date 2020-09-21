use crate::{
    runtime::{core::Span, interpreter::bytecode::Bytecode},
    syntax::{Block, Parser, SyntaxNode, SyntaxTree},
};
use compiler::{CompilerContext, CompilerKind, CompilerStack};
use error::CompilerError;
use visitor::{BlockKind, NodeVisitor as _};

mod assembler;
mod compiler;
mod decl_scan;
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
    ) -> Result<CompilerContext, CompilerError> {
        self.compiler_stack.push(CompilerKind::Function);

        let root = syntax_tree.get(syntax_tree.root()).expect("Node should not be empty");

        let body = if let SyntaxNode::Block(body) = root {
            body.clone()
        } else {
            Block {
                expressions: Vec::new(),
                trailing_expression: Some(syntax_tree.root()),
                span: Span::new(0..0), // TODO: Make it easier to get a syntax node's span.
            }
        };

        self.visit((&body, BlockKind::Function(args)))?;

        let compiler_context = self.compiler_stack.pop()?;

        Ok(compiler_context)
    }

    pub(self) fn context(&mut self) -> Result<&mut CompilerContext, CompilerError> {
        self.compiler_stack.top_mut()
    }
}
