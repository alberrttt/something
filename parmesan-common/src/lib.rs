#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub struct Span {
    pub src_start: usize,
    pub src_end: usize,
    pub line_start: usize,
    pub line: usize,
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
