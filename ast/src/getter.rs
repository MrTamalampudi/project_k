use std::collections::HashMap;
use std::ops::Range;

use crate::{arguments::Args, primitives::Primitives};
use class::Method;
use macros::{Method, Span};
use span::SpanData;

#[derive(Debug, Clone, PartialEq, Span, Method)]
pub struct Getter {
    pub span: Range<usize>,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub returns: Primitives,
}
