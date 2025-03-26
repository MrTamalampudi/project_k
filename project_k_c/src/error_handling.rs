use std::collections::HashSet;

use crate::ast::Location;

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    message: String,
    start_location: Location,
    end_location: Location,
    source_path: String,
}

#[derive(Debug, Clone)]
pub struct ErrorManager {
    pub errors: Vec<ErrorInfo>,
}

impl ErrorManager {
    pub fn new() -> ErrorManager {
        ErrorManager { errors: Vec::new() }
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
