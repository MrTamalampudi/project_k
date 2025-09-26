use std::collections::HashMap;

use crate::{
    ast::{arguments::Args, primitives::Primitives, testcase_body::GetMethod},
    class::Method,
    location::{Span, Span_Trait},
};

#[derive(Debug, Clone, PartialEq)]
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

impl Span_Trait for Getter {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}
