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

#[derive(Debug, Clone, PartialEq)]
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
}

pub trait SpanTrait {
    fn get_span(&self) -> Span;
}
