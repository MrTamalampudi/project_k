#![allow(non_camel_case_types, unused)]

use crate::{
    ast::{expression::Expression, teststep::TestStep},
    location::Location,
};

#[derive(Debug, Clone)]
pub struct IfStmt {
    start: Location,
    end: Location,
    test: Expression,
    consequent: Vec<TestStep>,
    alternate: AlternateStatement,
}

#[derive(Debug, Clone)]
pub enum AlternateStatement {
    IF(Box<IfStmt>),
    ELSE(Vec<TestStep>),
}
