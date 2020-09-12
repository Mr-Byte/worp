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

impl Instruction {
    define_instructions! {
        pub const PUSH_NONE;
        pub const PUSH_UNIT;

        pub const PUSH_FALSE;
        pub const PUSH_TRUE;

        pub const PUSH_I0;
        pub const PUSH_I1;
        pub const PUSH_F0;
        pub const PUSH_F1;
        pub const PUSH_CONST;

        pub const POP;
        pub const DUP;

        pub const BUILD_TUPLE;
        pub const BUILD_LIST;
        pub const BUILD_OBJECT;

        pub const LOAD_LOCAL;
        pub const STORE_LOCAL;

        pub const LOAD_UPVALUE;
        pub const STORE_UPVALUE;

        pub const NEG;
        pub const NOT;

        pub const MUL;
        pub const DIV;
        pub const REM;
        pub const ADD;
        pub const SUB;

        pub const MUL_ASSIGN_LOCAL;
        pub const DIV_ASSIGN_LOCAL;
        pub const ADD_ASSIGN_LOCAL;
        pub const SUB_ASSIGN_LOCAL;
        // TODO: &= and |= ?

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

        pub const CALL;
        pub const CLOSURE;
        pub const RETURN;
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
        write!(f, "{:02X} | ", self.0)?;

        match *self {
            Instruction::PUSH_NONE => write!(f, "PUSH_NONE"),
            Instruction::PUSH_UNIT => write!(f, "PUSH_UNIT"),
            Instruction::PUSH_FALSE => write!(f, "PUSH_FALSE"),
            Instruction::PUSH_TRUE => write!(f, "PUSH_TRUE"),
            Instruction::PUSH_I0 => write!(f, "PUSH_I0"),
            Instruction::PUSH_I1 => write!(f, "PUSH_I1"),
            Instruction::PUSH_F0 => write!(f, "PUSHF_F0"),
            Instruction::PUSH_F1 => write!(f, "PUSH_F1"),
            Instruction::PUSH_CONST => write!(f, "PUSH_CONST"),
            Instruction::POP => write!(f, "POP"),
            Instruction::DUP => write!(f, "DUP"),
            Instruction::BUILD_LIST => write!(f, "BUILD_LIST"),
            Instruction::BUILD_OBJECT => write!(f, "BUILD_OBJECT"),
            Instruction::LOAD_LOCAL => write!(f, "LOAD_LOCAL"),
            Instruction::STORE_LOCAL => write!(f, "STORE_LOCAL"),
            Instruction::LOAD_UPVALUE => write!(f, "LOAD_UPVALUE"),
            Instruction::STORE_UPVALUE => write!(f, "STORE_UPVALUE"),
            // TODO: Do I keep these? Do I need UPVALUE versions?
            Instruction::MUL_ASSIGN_LOCAL => write!(f, "MUL_ASSIGN_LOCAL"),
            Instruction::DIV_ASSIGN_LOCAL => write!(f, "DIV_ASSIGN_LOCAL"),
            Instruction::ADD_ASSIGN_LOCAL => write!(f, "ADD_ASSIGN_LOCAL"),
            Instruction::SUB_ASSIGN_LOCAL => write!(f, "SUB_ASSIGN_LOCAL"),
            Instruction::NEG => write!(f, "NEG"),
            Instruction::NOT => write!(f, "NOT"),
            Instruction::MUL => write!(f, "MUL"),
            Instruction::DIV => write!(f, "DIV"),
            Instruction::REM => write!(f, "REM"),
            Instruction::ADD => write!(f, "ADD"),
            Instruction::SUB => write!(f, "SUB"),
            Instruction::GT => write!(f, "GT"),
            Instruction::GTE => write!(f, "GTE"),
            Instruction::LT => write!(f, "LT"),
            Instruction::LTE => write!(f, "LTE"),
            Instruction::EQ => write!(f, "EQ"),
            Instruction::NEQ => write!(f, "NEQ"),
            Instruction::LOGICAL_AND => write!(f, "LOGICAL_AND"),
            Instruction::LOGICAL_OR => write!(f, "LOGICAL_OR"),
            Instruction::JUMP => write!(f, "JUMP"),
            Instruction::JUMP_IF_FALSE => write!(f, "JUMP_IF_FALSE"),
            Instruction::CALL => write!(f, "CALL"),
            Instruction::RETURN => write!(f, "RETURN"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}
