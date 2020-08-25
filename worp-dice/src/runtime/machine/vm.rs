use super::{instruction::Instruction, Module};
use crate::runtime::{
    core::{
        symbol::common::operators::{
            OP_ADD, OP_DIV, OP_EQ, OP_GT, OP_GTE, OP_LT, OP_LTE, OP_MUL, OP_NEG, OP_NEQ, OP_NOT, OP_REM, OP_SUB,
        },
        Value, ValueKey,
    },
    error::{RuntimeError, Spanned as _, SpannedRuntimeError},
};

macro_rules! binary_op {
    ($bytecode:expr, $stack:expr, $op:ident) => {{
        let lhs = $stack
            .pop()
            .ok_or_else(|| RuntimeError::StackUnderflowed)
            .with_span(|| $bytecode.span())?;
        let rhs = $stack
            .pop()
            .ok_or_else(|| RuntimeError::StackUnderflowed)
            .with_span(|| $bytecode.span())?;
        let result = lhs
            .get(&ValueKey::Symbol($op))
            .with_span(|| $bytecode.span())?
            .call(&[lhs, rhs])
            .with_span(|| $bytecode.span())?;
        $stack.push(result);
    }};
}

macro_rules! unary_op {
    ($bytecode:expr, $stack:expr, $op:expr) => {{
        let value = $stack
            .pop()
            .ok_or_else(|| RuntimeError::StackUnderflowed)
            .with_span(|| $bytecode.span())?;
        let result = value
            .get(&ValueKey::Symbol($op))
            .with_span(|| $bytecode.span())?
            .call(&[value])
            .with_span(|| $bytecode.span())?;
        $stack.push(result);
    }};
}

#[derive(Default)]
pub struct VirtualMachine {
    stack: Vec<Value>,
    // globals: HashMap<Symbol, String>,
}

impl VirtualMachine {
    // TODO: Load the specified module into the VM for use during execution of other modules.
    // fn load_module(&mut self, mut module: Module) -> Result<(), RuntimeError> {
    //     todo!()
    // }

    pub fn execute(&mut self, mut module: Module) -> Result<Value, SpannedRuntimeError> {
        while let Some(instruction) = module.bytecode().read_instruction() {
            match instruction {
                Instruction::PUSH_NONE => {
                    self.stack.push(Value::NONE);
                }
                Instruction::PUSH_FALSE => self.stack.push(Value::new(false)),
                Instruction::PUSH_TRUE => self.stack.push(Value::new(true)),
                Instruction::PUSH_INT => {
                    let int = module.bytecode().read_int();
                    self.stack.push(Value::new(int));
                }
                Instruction::PUSH_FLOAT => {
                    let float = module.bytecode().read_float();
                    self.stack.push(Value::new(float));
                }
                Instruction::PUSH_CONST => {
                    let const_pos = module.bytecode().read_int();
                    let value = module.bytecode().constants()[const_pos as usize].clone();
                    self.stack.push(value);
                }

                Instruction::POP => {
                    self.stack.pop();
                }
                Instruction::DUP => {
                    let value = self
                        .stack
                        .last()
                        .ok_or_else(|| RuntimeError::StackUnderflowed)
                        .with_span(|| module.bytecode().span())?
                        .clone();
                    self.stack.push(value);
                }

                Instruction::NEG => unary_op!(module.bytecode(), self.stack, OP_NEG),
                Instruction::NOT => unary_op!(module.bytecode(), self.stack, OP_NOT),

                Instruction::MUL => binary_op!(module.bytecode(), self.stack, OP_MUL),
                Instruction::DIV => binary_op!(module.bytecode(), self.stack, OP_DIV),
                Instruction::REM => binary_op!(module.bytecode(), self.stack, OP_REM),
                Instruction::ADD => binary_op!(module.bytecode(), self.stack, OP_ADD),
                Instruction::SUB => binary_op!(module.bytecode(), self.stack, OP_SUB),

                Instruction::GT => binary_op!(module.bytecode(), self.stack, OP_GT),
                Instruction::GTE => binary_op!(module.bytecode(), self.stack, OP_GTE),
                Instruction::LT => binary_op!(module.bytecode(), self.stack, OP_LT),
                Instruction::LTE => binary_op!(module.bytecode(), self.stack, OP_LTE),
                Instruction::EQ => binary_op!(module.bytecode(), self.stack, OP_EQ),
                Instruction::NEQ => binary_op!(module.bytecode(), self.stack, OP_NEQ),
                Instruction::HALT => return Ok(self.stack.pop().unwrap_or(Value::NONE)),
                unknown => {
                    return Err(RuntimeError::UnknownInstruction(unknown.into())).with_span(|| module.bytecode().span())
                }
            }
        }

        Ok(self.stack.pop().unwrap_or(Value::NONE))
    }
}
