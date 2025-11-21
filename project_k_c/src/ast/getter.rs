use std::collections::HashMap;

use crate::{
    ast::{arguments::Args, primitives::Primitives, teststep::GetMethod},
    class::Method,
};
use span::{Span, SpanData};
use span_macro::Span;

#[derive(Debug, Clone, PartialEq, Span)]
pub struct Getter {
    pub span: Span,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub returns: Primitives,
}

impl GetMethod for Getter {
    fn get_method(&self) -> Method {
        self.method.clone()
    }
}
