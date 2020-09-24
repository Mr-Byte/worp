use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct None;

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}
