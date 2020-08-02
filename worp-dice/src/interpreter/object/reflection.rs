use crate::interpreter::symbol::Symbol;
use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

#[derive(Clone)]
pub struct TypeData {
    inner: Rc<RefCell<TypeDataInner>>,
}

struct TypeDataInner {
    tag: Symbol,
    tags: Vec<Symbol>,
}

impl TypeData {
    pub fn new(tag: Symbol, tags: Vec<Symbol>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(TypeDataInner { tag, tags })),
        }
    }

    pub fn tag(&self) -> impl Deref<Target = Symbol> + '_ {
        Ref::map(self.inner.borrow(), |inner| &inner.tag)
    }

    pub fn tags(&self) -> impl Deref<Target = [Symbol]> + '_ {
        Ref::map(self.inner.borrow(), |inner| inner.tags.as_slice())
    }

    pub fn add_tag(&mut self, tag: Symbol) {
        self.inner.borrow_mut().tags.push(tag)
    }
}
