use crate::keywords::NTokenType;
use logos::Span;
use manodae::token::TokenKind;

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Token {
    pub token_type: NTokenType,
    pub span: Span,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.token_type.to_string()
    }
}

impl Token {
    pub fn new(token_type: NTokenType, span: Span) -> Self {
        Self { token_type, span }
    }

    pub fn get_start_location(&self) -> usize {
        self.span.start
    }

    pub fn get_end_location(&self) -> usize {
        self.span.end
    }

    pub fn get_token_type(&self) -> NTokenType {
        self.token_type.clone()
    }

    ///Creates a dummy token with a new span
    pub fn make_dummy_token(&self, span: &Span) -> Token {
        let mut new = self.clone();
        new.span = span.clone();
        new
    }
}
