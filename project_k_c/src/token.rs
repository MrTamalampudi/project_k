use std::fmt;

use crate::{
    keywords::TokenType,
    location::{Location, Span},
};

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
    pub source_path: String,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.token_type.to_string()
    }
}

impl Token {
    pub fn new(token_type: TokenType, start: Location, end: Location, source_path: String) -> Self {
        let span = Span { start, end };
        Self {
            token_type,
            span,
            source_path,
        }
    }

    pub fn get_start_location(&self) -> Location {
        self.span.start
    }

    pub fn get_end_location(&self) -> Location {
        self.span.end
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn get_source_path(&self) -> String {
        self.source_path.clone()
    }
}
