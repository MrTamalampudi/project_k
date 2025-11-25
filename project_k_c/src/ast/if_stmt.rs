#![allow(non_camel_case_types, unused)]

use crate::{
    ast::{
        action::Action,
        expression::Expr,
        teststep::{Body, GetMethod},
    },
    class::Method,
};
use span::{Location, Span, SpanData};
use span_macro::Span;

#[derive(Debug, Clone, PartialEq, Span)]
pub struct IfStmt {
    pub span: Span,
    pub condition: Expr,
    pub body: Body,
    pub or_else: Box<Option<IfStmt>>,
    pub method: Method,
}

impl GetMethod for IfStmt {
    fn get_method(&self) -> Method {
        self.method.clone()
    }
}
