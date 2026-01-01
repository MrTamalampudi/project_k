use std::collections::HashMap;

use crate::{arguments::Args, primitives::Primitives};
use class::Method;
use macros::{Method, Span};
use span::{Span, SpanData};

#[derive(Debug, Clone, PartialEq, Span, Method)]
pub struct Getter {
    pub span: Span,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub returns: Primitives,
}
