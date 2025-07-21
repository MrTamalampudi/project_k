use std::fmt;

use crate::ast::{if_stmt::IfStmt, testcase::TestCase, teststep::TestStep};

pub mod arguments;
pub mod expression;
pub mod getter;
pub mod identifier_value;
pub mod if_stmt;
pub mod primitives;
pub mod testcase;
pub mod teststep;
pub mod var_decl;

#[derive(Debug, Clone)]
pub enum AST {
    TESTCASE(TestCase),
    TESTSTEP(TestStep),
    IF(IfStmt),
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AST")
    }
}

impl AST {
    pub fn get_testcase_from_ast(ast: Option<&mut AST>) -> Option<&mut TestCase> {
        if let Some(AST::TESTCASE(testcase)) = ast {
            Some(testcase)
        } else {
            None
        }
    }
}
