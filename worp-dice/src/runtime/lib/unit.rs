use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unit;

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}
