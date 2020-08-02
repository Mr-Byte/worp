use crate::interpreter::symbol::Symbol;

#[derive(Clone)]
pub struct TypeData {
    tag: Symbol,
    tags: Vec<Symbol>,
}

impl TypeData {
    pub fn new(tag: Symbol, tags: Vec<Symbol>) -> Self {
        Self { tag, tags }
    }

    pub fn tag(&self) -> &Symbol {
        &self.tag
    }

    pub fn tags(&self) -> &[Symbol] {
        self.tags.as_slice()
    }

    pub fn add_tag(&mut self, tag: Symbol) {
        self.tags.push(tag)
    }
}
