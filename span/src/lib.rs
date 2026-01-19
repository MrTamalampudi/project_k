use std::ops::Range;

///Getter and Setter beahaviour for Span
pub trait SpanData {
    fn get_span(&self) -> Range<usize>;
    fn set_span(&mut self, span: Range<usize>);
    fn span_to(&self, span: &Range<usize>) -> Range<usize> {
        self.get_span().start..span.end
    }
}
