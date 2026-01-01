use class::Method;
use macros::{Method, Span};
use span::{Span, SpanData};

use crate::{expression::Expr, teststep::Body};

#[derive(Debug, Clone, PartialEq, Span, Method)]
pub struct ForLoop {
    pub span: Span,
    pub iter: Expr,
    pub target: String,
    pub body: Body,
    pub method: Method,
}
