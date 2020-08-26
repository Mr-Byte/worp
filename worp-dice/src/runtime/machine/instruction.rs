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
        pub const HALT;

        pub const PUSH_NONE;
        pub const PUSH_FALSE;
        pub const PUSH_TRUE;
        pub const PUSH_INT;
        pub const PUSH_FLOAT;
        pub const PUSH_CONST;

        pub const POP;
        pub const DUP;

        pub const LOAD_LOCAL;
        pub const STORE_LOCAL;

        pub const LOAD_GLOBAL;
        pub const STORE_GLOBAL;

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
