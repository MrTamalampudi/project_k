use std::fmt;

use crate::{keywords::TokenType, location::Location};

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Location,
    pub end: Location,
    pub source_path: String,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.token_type.to_string()
    }
}

impl Token {
    pub fn new(token_type: TokenType, start: Location, end: Location, source_path: String) -> Self {
        Self {
            token_type,
            start,
            end,
            source_path,
        }
    }

    pub fn get_start_location(&self) -> Location {
        self.start
    }

    pub fn get_end_location(&self) -> Location {
        self.end
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn get_source_path(&self) -> String {
        self.source_path.clone()
    }
}
