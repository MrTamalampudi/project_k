use logos::Span;
use manodae::error::ParseError;

pub trait ActionError {
    fn push_error(&mut self, span: &Span, message: String);
}

impl ActionError for Vec<ParseError> {
    fn push_error(&mut self, span: &Span, message: String) {
        self.push(ParseError::new(span.clone(), message));
    }
}
