use manodae::error::ParseError;

use crate::token::Token;
use span::{Location, Span};

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub message: String,
    pub span: Span,
    pub source_path: String,
}

#[derive(Debug, Clone)]
pub struct ErrorManager {
    pub errors: Vec<ErrorInfo>,
}

pub fn parse_error_to_error_info(error: ParseError<Token>) -> ErrorInfo {
    ErrorInfo {
        message: error.message,
        span: error.token.span,
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
            span: token.span.clone(),
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
        let span = Span {
            start: start_location,
            end: end_location,
        };
        self.errors.push(ErrorInfo {
            message,
            span,
            source_path,
        });
    }
}
