use std::rc::Rc;

pub fn to_string<I: ToString>(input: &I) -> Rc<str> {
    input.to_string().into()
}
