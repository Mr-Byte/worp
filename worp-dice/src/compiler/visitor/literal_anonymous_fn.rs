use crate::{
    compiler::{scope_stack::State, Compiler},
    runtime::lib::FnScript,
    syntax::LitAnonymousFn,
    CompilerError, Symbol, Value,
};

use super::NodeVisitor;

impl NodeVisitor<&LitAnonymousFn> for Compiler {
    fn visit(&mut self, node: &LitAnonymousFn) -> Result<(), CompilerError> {
        let id = uuid::Uuid::new_v4();
        let name = format!("__anonymous_fn_{:X}", id.clone().to_simple());
        let fn_name = Symbol::new(name.clone());

        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let mut fn_context = self.compile_fn(body, &node.args)?;
        let upvalues = fn_context.upvalues().clone();
        let bytecode = fn_context.finish();
        let value = Value::FnScript(FnScript::new(name, node.args.len(), bytecode, id));
        let context = self.context()?;
        let slot = {
            let local = context.scope_stack().add_local(
                fn_name,
                State::Local {
                    is_initialized: true,
                    is_mutable: false,
                },
            )?;

            local as u8
        };

        context.assembler().closure(value, &upvalues, node.span.clone())?;
        context.assembler().store_local(slot as u8, node.span.clone());

        Ok(())
    }
}
