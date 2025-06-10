use crate::ast::{testcase::TestCase, teststep::TestStep};

pub mod testcase;
pub mod teststep;

pub enum AST {
    TESTCASE(TestCase),
    TESTSTEP(TestStep),
}
