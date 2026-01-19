#![allow(non_camel_case_types, unused)]

use std::ops::Range;

use crate::{action::Action, expression::Expr, teststep::Body};
use class::Method;
use macros::{Method, Span};
use span::SpanData;

#[derive(Debug, Clone, PartialEq, Span, Method)]
pub struct IfStmt {
    pub span: Range<usize>,
    pub condition: Expr,
    pub body: Body,
    pub or_else: Box<Option<IfStmt>>,
    pub method: Method,
}
