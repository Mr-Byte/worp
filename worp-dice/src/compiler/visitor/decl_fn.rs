use crate::{compiler::Compiler, runtime::lib::FnScript, syntax::FnDecl, Symbol, Value};

use super::NodeVisitor;

impl NodeVisitor<&FnDecl> for Compiler {
    fn visit(&mut self, node: &FnDecl) -> Result<(), crate::CompilerError> {
        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let bytecode = self.compile_fn(body, Symbol::new(&node.name), &node.args)?;
        let value = Value::FnScript(FnScript::new(node.name.clone(), node.args.len(), bytecode));
        let slot = self.scope_stack.add_local(Symbol::new(&node.name), false)?;

        self.current_assembler().closure(value, node.span.clone());
        self.current_assembler().store_local(slot as u8, node.span.clone());

        Ok(())
    }
}
