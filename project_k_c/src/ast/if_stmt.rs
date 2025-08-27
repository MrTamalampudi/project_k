#![allow(non_camel_case_types, unused)]

use crate::{
    ast::{expression::Expression, testcase_body::GetMethod, teststep::TestStep},
    class::Method,
    location::Location,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub start: Location,
    pub end: Location,
    pub test: Expression,
    pub consequent: Vec<TestStep>,
    pub alternate: AlternateStatement,
    pub method: Method,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlternateStatement {
    IF(Box<IfStmt>),
    ELSE(Vec<TestStep>),
}

impl GetMethod for IfStmt {
    fn get_method(&self) -> Method {
        self.method.clone()
    }
}
