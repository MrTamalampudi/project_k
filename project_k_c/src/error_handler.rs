use manodae::error::ParseError;

use crate::{location::Location, token::Token};

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub message: String,
    pub start_location: Location,
    pub end_location: Location,
    pub source_path: String,
}

#[derive(Debug, Clone)]
pub struct ErrorManager {
    pub errors: Vec<ErrorInfo>,
}

pub fn parse_error_to_error_info(error: ParseError<Token>) -> ErrorInfo {
    ErrorInfo {
        message: error.message,
        start_location: error.token.start,
        end_location: error.token.end,
        source_path: error.token.source_path,
    }
}

impl ErrorManager {
    pub fn new() -> ErrorManager {
        ErrorManager { errors: Vec::new() }
    }

    pub fn insert_parsing_error(&mut self, message: String, token: &Token) {
        self.errors.push(ErrorInfo {
            message,
            start_location: token.get_start_location(),
            end_location: token.get_end_location(),
            source_path: token.get_source_path(),
        });
    }

    pub fn insert_error(
        &mut self,
        message: String,
        start_location: Location,
        end_location: Location,
        source_path: String,
    ) {
        self.errors.push(ErrorInfo {
            message,
            start_location,
            end_location,
            source_path,
        });
    }
}
