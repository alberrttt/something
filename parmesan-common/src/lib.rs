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
