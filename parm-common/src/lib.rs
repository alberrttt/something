#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub struct Span {
    pub src_start: usize,
    pub src_end: usize,
    pub line_start: usize,
    pub line: usize,
}
impl Span {
    pub fn join(self, other: Self) -> Self {
        let src_start: usize = self.src_start.min(other.src_start);
        let src_end = self.src_end.max(other.src_end);
        let line_start = self.line_start.min(other.line_start);
        let line = self.line.max(other.line);
        Span {
            src_start,
            src_end,
            line_start,
            line,
        }
    }
}
impl From<(Span, Span)> for Span {
    fn from((start, end): (Span, Span)) -> Self {
        Span {
            src_start: start.src_start,
            src_end: end.src_end,
            line: end.line,
            line_start: start.line_start,
        }
    }
}
pub trait Spanned {
    fn span(&self) -> Span;
}
impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self) -> Span {
        let first = self.first().unwrap();
        let last = self.last().unwrap();
        (first.span(), last.span()).into()
    }
}
impl<T: Spanned> Spanned for Option<T> {
    fn span(&self) -> Span {
        match self {
            Some(t) => t.span(),
            None => panic!(),
        }
    }
}
impl<T> Spanned for Box<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        todo!()
    }
}

impl<A, B> Spanned for (A, B)
where
    A: Spanned,
    B: Spanned,
{
    fn span(&self) -> Span {
        self.0.span().join(self.1.span())
    }
}
impl<'a, T: Spanned + 'a> Spanned for &'a [T] {
    fn span(&self) -> Span {
        let first = self.first().unwrap();
        let last = self.last().unwrap();
        (first.span(), last.span()).into()
    }
}
