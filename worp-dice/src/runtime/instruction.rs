use std::fmt::{Debug, Display};

macro_rules! define_instructions {
    (prev=$prev:ident @) => {};
    (prev=$prev:ident @ $vis:vis $next:ident $($sub_vis:vis $name:ident)*) => {
        $vis const $next: Self = Self(Self::$prev.0 + 1);
        define_instructions! {
            prev=$next @
            $($sub_vis $name)*
        }
    };

    ($vis:vis const $first:ident; $($sub_vis:vis const $name:ident;)*) => {
        $vis const $first: Self = Self(0);
        define_instructions! {
            prev=$first @
            $($sub_vis $name)*
        }
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction(u8);

// TODO: Fuse PUSH_INT/FLOAT/CONST into a single opcode, that use a single byte to represent the index of the const
// TODO: Add an opcode to represent 0 and 1 literals for ints and floats.

impl Instruction {
    define_instructions! {
        pub const PUSH_NONE;
        pub const PUSH_UNIT;

        pub const PUSH_FALSE;
        pub const PUSH_TRUE;

        pub const PUSHI_ZERO;
        pub const PUSHI_ONE;
        pub const PUSHF_ZERO;
        pub const PUSHF_ONE;

        // TODO: Fuse these.
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

        pub const ADD_ASSIGN_LOCAL;

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

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        Instruction(value)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Instruction::PUSH_NONE => write!(f, "{:#04X} | PUSH_NONE", self.0),
            Instruction::PUSH_UNIT => write!(f, "{:#04X} | PUSH_UNIT", self.0),
            Instruction::PUSH_FALSE => write!(f, "{:#04X} | PUSH_FALSE", self.0),
            Instruction::PUSH_TRUE => write!(f, "{:#04X} | PUSH_TRUE", self.0),
            Instruction::PUSHI_ZERO => write!(f, "{:04X} | PUSHI_ZERO", self.0),
            Instruction::PUSHI_ONE => write!(f, "{:04X} | PUSHI_ONE", self.0),
            Instruction::PUSH_INT => write!(f, "{:#04X} | PUSH_INT", self.0),
            Instruction::PUSH_FLOAT => write!(f, "{:#04X} | PUSH_FLOAT", self.0),
            Instruction::PUSH_CONST => write!(f, "{:#04X} | PUSH_CONST", self.0),
            Instruction::POP => write!(f, "{:#04X} | POP", self.0),
            Instruction::DUP => write!(f, "{:#04X} | DUP", self.0),
            Instruction::LOAD_LOCAL => write!(f, "{:#04X} | LOAD_LOCAL", self.0),
            Instruction::STORE_LOCAL => write!(f, "{:#04X} | STORE_LOCAL", self.0),
            Instruction::ADD_ASSIGN_LOCAL => write!(f, "{:#04X} | ADD_ASSIGN_LOCAL", self.0),
            Instruction::NEG => write!(f, "{:#04X} | NEG", self.0),
            Instruction::NOT => write!(f, "{:#04X} | NOT", self.0),
            Instruction::MUL => write!(f, "{:#04X} | MUL", self.0),
            Instruction::DIV => write!(f, "{:#04X} | DIV", self.0),
            Instruction::REM => write!(f, "{:#04X} | REM", self.0),
            Instruction::ADD => write!(f, "{:#04X} | ADD", self.0),
            Instruction::SUB => write!(f, "{:#04X} | SUB", self.0),
            Instruction::GT => write!(f, "{:#04X} | GT", self.0),
            Instruction::GTE => write!(f, "{:#04X} | GTE", self.0),
            Instruction::LT => write!(f, "{:#04X} | LT", self.0),
            Instruction::LTE => write!(f, "{:#04X} | LTE", self.0),
            Instruction::EQ => write!(f, "{:#04X} | EQ", self.0),
            Instruction::NEQ => write!(f, "{:#04X} | NEQ", self.0),
            Instruction::LOGICAL_AND => write!(f, "{:#04X} | LOGICAL_AND", self.0),
            Instruction::LOGICAL_OR => write!(f, "{:#04X} | LOGICAL_OR", self.0),
            Instruction::JUMP => write!(f, "{:#04X} | JUMP", self.0),
            Instruction::JUMP_IF_FALSE => write!(f, "{:#04X} | JUMP_IF_FALSE", self.0),
            i => write!(f, "{:#04X} | UNKNOWN", i.0),
        }
    }
}
