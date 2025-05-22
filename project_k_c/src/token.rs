use std::fmt;

use slr_parser::terminal::Terminal;

use crate::{ast::Location, keywords::TokenType};

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Location,
    pub end: Location,
    pub source_path: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
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

impl Terminal for Token {
    fn get_ending_token() -> String {
        "EOF".to_string()
    }

    fn to_string_c(&self) -> String {
        self.token_type.to_string()
    }
}
