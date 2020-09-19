use crate::{compiler::Compiler, runtime::lib::FnScript, syntax::FnDecl, Symbol, Value};

use super::NodeVisitor;

impl NodeVisitor<&FnDecl> for Compiler {
    fn visit(&mut self, node: &FnDecl) -> Result<(), crate::CompilerError> {
        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let mut fn_context = self.compile_fn(body, &node.args)?;
        let upvalues = fn_context.upvalues().clone();
        let bytecode = fn_context.finish();
        let value = Value::FnScript(FnScript::new(node.name.clone(), node.args.len(), bytecode));
        let context = self.context()?;
        let slot = context.scope_stack().add_local(Symbol::new(&node.name), false)?;

        context.assembler().closure(value, &upvalues, node.span.clone());
        context.assembler().store_local(slot as u8, node.span.clone());

        Ok(())
    }
}
