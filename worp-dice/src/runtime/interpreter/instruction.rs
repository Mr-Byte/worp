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
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct Instruction(u8);

        impl Instruction {
            $vis const $first: Self = Self(0);
            define_instructions! {
                prev=$first @
                $($sub_vis $name)*
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
                    Self::$first => write!(f, stringify!($first)),
                    $(Self::$name => write!(f, stringify!($name)),)*
                    _ => unreachable!()
                }
            }
        }
    };
}

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
