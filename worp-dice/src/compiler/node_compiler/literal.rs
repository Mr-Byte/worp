use super::NodeCompiler;
use crate::{compiler::Compiler, syntax::Literal, CompilerError, Value};

impl NodeCompiler<Literal> for Compiler {
    fn compile_node(&mut self, node: Literal) -> Result<(), CompilerError> {
        match node {
            Literal::Identifier(name, span) => self.load_variable(name, span)?,
            Literal::None(span) => self.assembler.push_none(span),
            Literal::Unit(span) => self.assembler.push_unit(span),
            Literal::Integer(value, span) => match value {
                0 => self.assembler.push_i0(span),
                1 => self.assembler.push_i1(span),
                _ => self.assembler.push_const(Value::Int(value), span),
            },
            Literal::Float(value, span) => {
                if value == 0.0 {
                    self.assembler.push_f0(span);
                } else if value == 1.0 {
                    self.assembler.push_f1(span);
                } else {
                    self.assembler.push_const(Value::Float(value), span);
                }
            }
            Literal::String(value, span) => self.assembler.push_const(Value::String(value), span),
            Literal::Boolean(value, span) => self.assembler.push_bool(value, span),
            Literal::List(list, span) => {
                for item in &list {
                    self.compile_node(*item)?;
                }

                self.assembler.build_list(list.len() as u8, span);
            }
            Literal::Object(_, _) => todo!(),
        };

        Ok(())
    }
}
