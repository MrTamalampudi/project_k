use std::ops::Range;

use manodae::error::ParseError;

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub message: String,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct ErrorManager {
    pub errors: Vec<ErrorInfo>,
}

pub fn parse_error_to_error_info(error: ParseError) -> ErrorInfo {
    ErrorInfo {
        message: error.message,
        span: error.span,
    }
}

impl ErrorManager {
    pub fn new() -> ErrorManager {
        ErrorManager { errors: Vec::new() }
    }
}
