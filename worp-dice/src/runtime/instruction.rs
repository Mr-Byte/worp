use std::fmt::Debug;

macro_rules! define_instructions {
    (prev=$prev:ident @) => {};
    (prev=$prev:ident @ $next:ident $($name:ident)*) => {
        pub const $next: Self = Self(Self::$prev.0 + 1);
        define_instructions! {
            prev=$next @
            $($name)*
        }
    };

    (pub const $first:ident; $(pub const $name:ident;)*) => {
        pub const $first: Self = Self(0);
        define_instructions! {
            prev=$first @
            $($name)*
        }
    };
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Instruction(u8);

impl Instruction {
    define_instructions! {
        pub const PUSH_NONE;
        pub const PUSH_UNIT;
        pub const PUSH_FALSE;
        pub const PUSH_TRUE;
        pub const PUSH_INT;
        pub const PUSH_FLOAT;
        pub const PUSH_CONST;

        pub const POP;
        pub const DUP;

        pub const LOAD_LOCAL;
        pub const STORE_LOCAL;

        pub const NEG;
        pub const NOT;

        pub const MUL;
        pub const DIV;
        pub const REM;
        pub const ADD;
        pub const SUB;

        pub const GT;
        pub const GTE;
        pub const LT;
        pub const LTE;
        pub const EQ;
        pub const NEQ;
        pub const LOGICAL_AND;
        pub const LOGICAL_OR;

        pub const JUMP;
        pub const JUMP_IF_FALSE;
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

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Instruction::PUSH_NONE => write!(f, "PUSH_NONE {:#04X}", self.0),
            Instruction::PUSH_UNIT => write!(f, "PUSH_UNIT {:#04X}", self.0),
            Instruction::PUSH_FALSE => write!(f, "PUSH_FALSE {:#04X}", self.0),
            Instruction::PUSH_TRUE => write!(f, "PUSH_TRUE {:#04X}", self.0),
            Instruction::PUSH_INT => write!(f, "PUSH_INT {:#04X}", self.0),
            Instruction::PUSH_FLOAT => write!(f, "PUSH_FLOAT {:#04X}", self.0),
            Instruction::PUSH_CONST => write!(f, "PUSH_CONST {:#04X}", self.0),
            Instruction::POP => write!(f, "POP {:#04X}", self.0),
            Instruction::DUP => write!(f, "DUP {:#04X}", self.0),
            Instruction::LOAD_LOCAL => write!(f, "LOAD_LOCAL {:#04X}", self.0),
            Instruction::STORE_LOCAL => write!(f, "STORE_LOCAL {:#04X}", self.0),
            Instruction::NEG => write!(f, "NEG {:#04X}", self.0),
            Instruction::NOT => write!(f, "NOT {:#04X}", self.0),
            Instruction::MUL => write!(f, "MUL {:#04X}", self.0),
            Instruction::DIV => write!(f, "DIV {:#04X}", self.0),
            Instruction::REM => write!(f, "REM {:#04X}", self.0),
            Instruction::ADD => write!(f, "ADD {:#04X}", self.0),
            Instruction::SUB => write!(f, "SUB {:#04X}", self.0),
            Instruction::GT => write!(f, "GT {:#04X}", self.0),
            Instruction::GTE => write!(f, "GTE {:#04X}", self.0),
            Instruction::LT => write!(f, "LT {:#04X}", self.0),
            Instruction::LTE => write!(f, "LTE {:#04X}", self.0),
            Instruction::EQ => write!(f, "EQ {:#04X}", self.0),
            Instruction::NEQ => write!(f, "NEQ {:#04X}", self.0),
            Instruction::LOGICAL_AND => write!(f, "LOGICAL_AND {:#04X}", self.0),
            Instruction::LOGICAL_OR => write!(f, "LOGICAL_OR {:#04X}", self.0),
            Instruction::JUMP => write!(f, "JUMP {:#04X}", self.0),
            Instruction::JUMP_IF_FALSE => write!(f, "JUMP_IF_FALSE {:#04X}", self.0),
            i => write!(f, "UNKNOWN {:#04X}", i.0),
        }
    }
}
