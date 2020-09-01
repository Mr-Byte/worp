use crate::{
    runtime::{
        core::Symbol,
        script::{CallFrame, Module, Script},
    },
    syntax::{Parser, SyntaxTree},
};
use bytecode::BytecodeGenerator;
use error::CompilerError;

mod bytecode;
pub mod error;
mod expression;

pub struct Compiler {
    syntax_tree: SyntaxTree,
    bytecode: BytecodeGenerator,
    call_frame: CallFrame,
    scope_depth: usize,
    locals: Vec<Local>,
}

impl Compiler {
    #[allow(dead_code)]
    pub fn compile_module(input: &str) -> Result<Module, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self {
            syntax_tree,
            bytecode: BytecodeGenerator::default(),
            call_frame: CallFrame::default(),
            scope_depth: 0,
            locals: Vec::new(),
        };

        compiler.compile(compiler.syntax_tree.root())?;

        let module = Module::new(compiler.bytecode.generate());

        Ok(module)
    }

    // TODO: Disallow item exports in scripts.
    pub fn compile_script(input: &str) -> Result<Script, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self {
            syntax_tree,
            bytecode: BytecodeGenerator::default(),
            call_frame: CallFrame::default(),
            scope_depth: 0,
            locals: Vec::new(),
        };

        compiler.compile(compiler.syntax_tree.root())?;

        let script = Script::new(compiler.bytecode.generate(), compiler.call_frame);

        Ok(script)
    }

    pub(self) fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub(self) fn end_scope(&mut self) {
        while let Some(scope_depth) = self.locals.last().map(|local| local.scope_depth) {
            if scope_depth < self.scope_depth {
                break;
            }

            self.locals.pop();
        }

        self.scope_depth -= 1;
    }

    pub(self) fn add_local(&mut self, name: Symbol, is_mutable: bool) -> u8 {
        let slot = self.locals.len();

        let local = Local {
            name,
            is_mutable,
            scope_depth: self.scope_depth,
            slot: self.locals.len() as u8,
        };

        self.locals.push(local);

        // Increment slot count if the slot's index is greater than or equal to that of the slot count.
        if slot >= self.call_frame.slot_count {
            self.call_frame.slot_count = slot + 1;
        }

        slot as u8
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
