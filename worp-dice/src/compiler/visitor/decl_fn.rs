use crate::{
    compiler::{scope_stack::State, Compiler},
    runtime::lib::FnScript,
    syntax::FnDecl,
    CompilerError, Symbol, Value,
};

use super::NodeVisitor;

impl NodeVisitor<&FnDecl> for Compiler {
    fn visit(&mut self, node: &FnDecl) -> Result<(), CompilerError> {
        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let mut fn_context = self.compile_fn(body, &node.args)?;
        let upvalues = fn_context.upvalues().clone();
        let bytecode = fn_context.finish();
        let value = Value::FnScript(FnScript::new(
            node.name.clone(),
            node.args.len(),
            bytecode,
            uuid::Uuid::new_v4(),
        ));
        let context = self.context()?;
        let slot = {
            let fn_name = Symbol::new(&node.name);
            let local = context.scope_stack().local(fn_name.clone()).ok_or_else(|| {
                CompilerError::InternalCompilerError(String::from("Function not already declared in scope."))
            })?;

            // NOTE: Check if a function of the given name has already been initialized.
            if let State::Function { ref mut is_initialized } = &mut local.state {
                if *is_initialized {
                    return Err(CompilerError::ItemAlreadyDeclared(fn_name));
                }

                *is_initialized = true;
            }

            local.slot as u8
        };

        context.assembler().closure(value, &upvalues, node.span.clone())?;
        context.assembler().store_local(slot as u8, node.span.clone());

        Ok(())
    }
}
