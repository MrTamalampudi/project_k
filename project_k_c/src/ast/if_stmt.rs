#![allow(non_camel_case_types, unused)]

use crate::{
    ast::{action::Action, expression::Expr, teststep::GetMethod},
    class::Method,
    location::Location,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub start: Location,
    pub end: Location,
    pub test: Expr,
    pub consequent: Vec<Action>,
    pub alternate: AlternateStatement,
    pub method: Method,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlternateStatement {
    IF(Box<IfStmt>),
    ELSE(Vec<Action>),
}

impl GetMethod for IfStmt {
    fn get_method(&self) -> Method {
        self.method.clone()
    }
}
