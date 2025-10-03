use manodae::error::ParseError;

use crate::{location::Span, token::Token};

pub trait ActionError {
    fn push_error(&mut self, token: &Token, span: &Span, message: String);
}

impl ActionError for Vec<ParseError<Token>> {
    fn push_error(&mut self, token: &Token, span: &Span, message: String) {
        let dummy_token = token.make_dummy_token(span);
        self.push(ParseError::new(dummy_token, message));
    }
}
