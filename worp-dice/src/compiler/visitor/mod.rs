use crate::CompilerError;

mod decl_fn;
mod decl_var;
mod expr_assignment;
mod expr_binary_op;
mod expr_block;
mod expr_break;
mod expr_continue;
mod expr_if;
mod expr_unary_op;
mod expr_while;
mod literal_bool;
mod literal_float;
mod literal_int;
mod literal_list;
mod literal_none;
mod literal_string;
mod literal_unit;
mod literal_variable;
mod syntax_node;

pub(super) trait NodeVisitor<T> {
    fn visit(&mut self, node: T) -> Result<(), CompilerError>;
}
