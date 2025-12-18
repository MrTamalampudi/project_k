use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line:{} column:{}", self.line, self.column,)
    }
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }

    pub fn next_column(&mut self) -> Self {
        self.column += 1;
        *self
    }

    pub fn get_location(&self) -> Location {
        self.clone()
    }

    pub fn dummy() -> Location {
        Location { line: 0, column: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    pub fn to(&self, span: &Span) -> Span {
        Span {
            start: self.start,
            end: span.end,
        }
    }

    pub fn dummy() -> Span {
        Span::new(Location::dummy(), Location::dummy())
    }

    pub fn len(&self) -> usize {
        if self.start.line == self.end.line {
            return self.end.column - self.start.column;
        } else {
            //this is for lsp
            //we dont support & not require multiline yet
            // so returns 0
            return 0;
        }
    }
}

///Getter and Setter beahaviour for Span
pub trait SpanData {
    fn get_span(&self) -> Span;
    fn set_span(&mut self, span: Span);
}
