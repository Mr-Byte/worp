use super::{instruction::Instruction, Module};
use crate::runtime::{
    core::{
        symbol::common::operators::{OP_ADD, OP_DIV, OP_MUL, OP_REM, OP_SUB},
        Value, ValueKey,
    },
    error::RuntimeError,
};

#[derive(Default)]
pub struct VirtualMachine {
    stack: Vec<Value>,
}

impl VirtualMachine {
    // TODO: Load the specified module into the VM for use during execution of other modules.
    fn load_module(&mut self, mut module: Module) -> Result<(), RuntimeError> {
        todo!()
    }

    pub fn execute(&mut self, mut module: Module) -> Result<Value, RuntimeError> {
        while let Some(instruction) = module.bytecode().read_instruction() {
            match instruction {
                Instruction::PUSH_NONE => {
                    self.stack.push(Value::NONE);
                }
                Instruction::PUSH_INT => {
                    let int = module.bytecode().read_int();
                    self.stack.push(Value::new(int));
                }
                Instruction::PUSH_FLOAT => {
                    let int = module.bytecode().read_float();
                    self.stack.push(Value::new(int));
                }
                Instruction::PUSH_BOOL => {
                    let int = module.bytecode().read_bool();
                    self.stack.push(Value::new(int));
                }
                Instruction::POP => {
                    self.stack.pop();
                }
                Instruction::DUP => {
                    let value = self.stack.last().ok_or_else(|| RuntimeError::StackUnderflowed)?.clone();
                    self.stack.push(value);
                }
                Instruction::MUL => {
                    let lhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let rhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let result = lhs.get(&ValueKey::Symbol(OP_MUL))?.call(&[lhs, rhs])?;
                    self.stack.push(result);
                }
                Instruction::DIV => {
                    let lhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let rhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let result = lhs.get(&ValueKey::Symbol(OP_DIV))?.call(&[lhs, rhs])?;
                    self.stack.push(result);
                }
                Instruction::REM => {
                    let lhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let rhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let result = lhs.get(&ValueKey::Symbol(OP_REM))?.call(&[lhs, rhs])?;
                    self.stack.push(result);
                }
                Instruction::ADD => {
                    let lhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let rhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let result = lhs.get(&ValueKey::Symbol(OP_ADD))?.call(&[lhs, rhs])?;
                    self.stack.push(result);
                }
                Instruction::SUB => {
                    let lhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let rhs = self.stack.pop().ok_or_else(|| RuntimeError::StackUnderflowed)?;
                    let result = lhs.get(&ValueKey::Symbol(OP_SUB))?.call(&[lhs, rhs])?;
                    self.stack.push(result);
                }
                Instruction::HALT => return Ok(self.stack.pop().unwrap_or(Value::NONE)),
                unknown => return Err(RuntimeError::UnknownInstruction(unknown.into())),
            }
        }

        Ok(self.stack.pop().unwrap_or(Value::NONE))
    }
}

#[cfg(test)]
mod test {
    use super::VirtualMachine;
    use crate::runtime::{error::RuntimeError, machine::Module};

    #[test]
    fn test_add_opcode() -> Result<(), RuntimeError> {
        let mut vm = VirtualMachine::default();
        let mut module = Module::builder();

        module.bytecode().push_int(40);
        module.bytecode().push_int(2);
        module.bytecode().add();

        let result = vm.execute(module.build())?;
        let value = *result.try_value::<i64>()?;

        assert_eq!(42, value);

        Ok(())
    }

    #[test]
    fn test_add_opcode_chained() -> Result<(), RuntimeError> {
        let mut vm = VirtualMachine::default();
        let mut module = Module::builder();

        module.bytecode().push_int(40);
        module.bytecode().push_int(1);
        module.bytecode().add();
        module.bytecode().push_int(1);
        module.bytecode().add();

        let result = vm.execute(module.build())?;
        let value = *result.try_value::<i64>()?;

        assert_eq!(42, value);

        Ok(())
    }
}
