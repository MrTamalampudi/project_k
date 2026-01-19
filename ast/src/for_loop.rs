use class::Method;
use macros::{Method, Span};
use span::SpanData;
use std::ops::Range;

use crate::{expression::Expr, teststep::Body};

#[derive(Debug, Clone, PartialEq, Span, Method)]
pub struct ForLoop {
    pub span: Range<usize>,
    pub iter: Expr,
    pub target: String,
    pub body: Body,
    pub method: Method,
}
