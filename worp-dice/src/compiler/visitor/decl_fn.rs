use crate::{compiler::Compiler, runtime::lib::Func, syntax::FnDecl, Symbol, Value};

use super::NodeVisitor;

impl NodeVisitor<&FnDecl> for Compiler {
    fn visit(&mut self, node: &FnDecl) -> Result<(), crate::CompilerError> {
        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let bytecode = Self::compile_fn(body, Symbol::new(&node.name), &node.args)?;
        let value = Value::Func(Func::new_fn(node.name.clone(), node.args.len(), bytecode));
        let slot = self.scope_stack.add_local(Symbol::new(&node.name), false)?;

        self.assembler.push_const(value, node.span.clone());
        self.assembler.store_local(slot as u8, node.span.clone());

        Ok(())
    }
}
