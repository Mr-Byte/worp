use super::Compiler;
use crate::CompilerError;

mod expr;
mod expr_assignment;
mod expr_binary_op;
mod expr_block;
mod expr_break;
mod expr_continue;
mod expr_if;
mod expr_unary_op;
mod expr_variable_decl;
mod expr_while;
mod literal;
mod literal_bool;
mod literal_float;
mod literal_int;
mod literal_list;
mod literal_none;
mod literal_string;
mod literal_unit;
mod literal_variable;

pub(super) trait NodeCompiler<T> {
    fn compile_node(&mut self, node: T) -> Result<(), CompilerError>;
}

impl Compiler {}
