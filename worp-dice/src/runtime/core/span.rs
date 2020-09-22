use std::ops::{Add, Range};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub const fn new(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
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

impl Into<Range<usize>> for Span {
    fn into(self) -> Range<usize> {
        self.start..self.end
    }
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Self::Output { start, end }
    }
}

impl Add<&Span> for Span {
    type Output = Span;

    fn add(self, rhs: &Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Self::Output { start, end }
    }
}

impl Add for &Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let start = self.start.min(rhs.start);
        let end = self.end.max(rhs.end);

        Self::Output { start, end }
    }
}
