use std::fmt;

use crate::ast::{testcase::TestCase, teststep::TestStep};

pub mod testcase;
pub mod teststep;

#[derive(Debug, Clone)]
pub enum AST {
    TESTCASE(TestCase),
    TESTSTEP(TestStep),
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AST")
    }
}
