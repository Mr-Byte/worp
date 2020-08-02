use crate::interpreter::symbol::Symbol;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Clone)]
pub struct TypeData {
    inner: Rc<RefCell<TypeDataInner>>,
}

struct TypeDataInner {
    type_tag: Symbol,
    type_tag_impls: Vec<Symbol>,
}

impl TypeData {
    pub fn new(type_tag: Symbol, type_tag_impls: Vec<Symbol>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(TypeDataInner { type_tag, type_tag_impls })),
        }
    }

    pub fn type_tag(&self) -> Ref<'_, Symbol> {
        Ref::map(self.inner.borrow(), |inner| &inner.type_tag)
    }

    pub fn type_tag_impls(&self) -> Ref<'_, [Symbol]> {
        Ref::map(self.inner.borrow(), |inner| inner.type_tag_impls.as_slice())
    }

    pub fn add_tag(&mut self, tag: Symbol) {
        self.inner.borrow_mut().type_tag_impls.push(tag)
    }
}
