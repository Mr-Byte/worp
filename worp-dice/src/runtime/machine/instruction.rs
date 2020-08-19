#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Instruction(u8);

impl Instruction {
    pub const HALT: Self = Self(0);

    pub const JUMP: Self = Self::HALT.next();
    pub const JUMP_IF: Self = Self::JUMP.next();

    pub const PUSH_NONE: Self = Self::JUMP_IF.next();
    pub const PUSH_INT: Self = Self::PUSH_NONE.next();
    pub const PUSH_FLOAT: Self = Self::PUSH_INT.next();
    pub const PUSH_BOOL: Self = Self::PUSH_FLOAT.next();
    pub const POP: Self = Self::PUSH_BOOL.next();
    pub const DUP: Self = Self::POP.next();

    pub const LOAD: Self = Self::DUP.next();
    pub const STORE: Self = Self::LOAD.next();

    pub const MUL: Self = Self::STORE.next();
    pub const DIV: Self = Self::MUL.next();
    pub const REM: Self = Self::DIV.next();
    pub const ADD: Self = Self::REM.next();
    pub const SUB: Self = Self::ADD.next();

    const fn next(self) -> Instruction {
        Instruction(self.0 + 1)
    }
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        self.0
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        Instruction(value)
    }
}
