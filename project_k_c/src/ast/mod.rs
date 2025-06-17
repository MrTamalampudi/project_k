use std::fmt;

use crate::ast::{if_stmt::IfStmt, testcase::TestCase, teststep::TestStep};

pub mod expression;
pub mod if_stmt;
pub mod primitives;
pub mod testcase;
pub mod teststep;
pub mod var_decl;

//
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
