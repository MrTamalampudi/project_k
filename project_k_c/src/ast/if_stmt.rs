#![allow(non_camel_case_types, unused)]

use crate::{
    ast::{action::Action, expression::Expr, teststep::GetMethod},
    class::Method,
};
use span::{Location, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub span: Span,
    pub condition: Expr,
    pub body: Vec<Action>,
    pub or_else: AlternateStatement,
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
