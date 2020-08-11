use std::ops::{Add, Deref, Range};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    range: Range<usize>,
}

impl Span {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}

impl From<Range<usize>> for Span {
    fn from(span: Range<usize>) -> Self {
        Self::new(span)
    }
}

impl From<&Range<usize>> for Span {
    fn from(span: &Range<usize>) -> Self {
        Self::new(span.clone())
    }
}

impl Deref for Span {
    type Target = Range<usize>;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Span::new(start..end)
    }
}

impl Add<&Span> for Span {
    type Output = Span;

    fn add(self, rhs: &Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Span::new(start..end)
    }
}

impl Add for &Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Span::new(start..end)
    }
}
